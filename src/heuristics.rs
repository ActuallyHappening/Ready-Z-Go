use std::{any::Any, marker::PhantomData, ops::RangeInclusive};

use crate::{card::sink, prelude::*, BasicGame};

#[dyn_safe(true)]
pub trait SinkHeuristic {
	/// May aggressively reject all sinks
	fn filter<'a, 'b, 'c>(&'a mut self, sim_game: &'b BasicGame, sink: &'c dyn Any) -> bool;
}

/// [None] is a no-op filter (allows everything) and disabled
impl<T> SinkHeuristic for Option<T>
where
	T: SinkHeuristic,
{
	fn filter(&mut self, sim_game: &BasicGame, sink: &dyn Any) -> bool {
		let Some(inner) = self.as_mut() else {
			return true;
		};
		inner.filter(sim_game, sink)
	}
}

/// idk exactly why I need this yet
impl<T> SinkHeuristic for &mut T
where
	T: SinkHeuristic,
{
	fn filter(&mut self, sim_game: &BasicGame, sink: &dyn Any) -> bool {
		(*self).filter(sim_game, sink)
	}
}

pub struct PreferSink<Sink> {
	pub turns: RangeInclusive<u8>,
	rejected_count: u128,
	_phantom: PhantomData<Sink>,
}

impl<T> PreferSink<T> {
	pub fn new(turns: RangeInclusive<u8>) -> Self {
		Self {
			turns,
			rejected_count: 0,
			_phantom: PhantomData,
		}
	}

	pub fn enabled(&self, sim_game: &BasicGame) -> bool {
		self.turns.contains(&sim_game.turn_num())
	}
}

impl<T: 'static> SinkHeuristic for PreferSink<T> {
	fn filter(&mut self, sim_game: &BasicGame, sink: &dyn Any) -> bool {
		if !self.enabled(sim_game) {
			return true;
		}
		if let Some(_bingo) = sink.downcast_ref::<T>() {
			true
		} else {
			self.rejected_count += 1;
			false
		}
	}
}
