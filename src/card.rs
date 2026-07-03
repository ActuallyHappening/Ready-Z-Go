use crate::{card::sink::bingo::SinkBingoMorfi, Score};

pub mod modifier;
pub mod sink;

#[derive(Clone, Default, Hash, PartialEq, Eq)]
pub struct Morfi {
	// pub modifiers: ModifiersMorfi,
	pub sinks: SinksMorfi,
}

impl Morfi {
	pub fn score(&self) -> Score {
		// interesting assumption that may be broken, modifiers can't grant score
		self.sinks.score()
	}
}

#[derive(Default)]
pub struct ModifiersMorfi {
	pub copy: modifier::Copy,
	pub squish: modifier::Squish,
	pub morph: modifier::Morph,
}

#[derive(Clone, Default, Hash, PartialEq, Eq)]
pub struct SinksMorfi {
	pub bingo: SinkBingoMorfi,
	pub value: SinksValueMorfi,
}

impl SinksMorfi {
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut dyn sink::Sink> {
		let mut ret = Vec::with_capacity(8);
		ret.push((&mut self.bingo) as &mut dyn sink::Sink);
		ret.extend(self.value.iter_mut());
		ret.into_iter()
	}

	pub fn iter(&self) -> impl Iterator<Item = &dyn sink::Sink> {
		let mut ret = Vec::with_capacity(8);
		ret.push(&self.bingo as &dyn sink::Sink);
		ret.extend(self.value.iter());
		ret.into_iter()
	}

	pub fn score(&self) -> Score {
		self.iter().map(|s| s.score()).sum()
	}
}

#[derive(Clone, Default, Hash, PartialEq, Eq)]
pub struct SinksValueMorfi {
	pub any: [sink::Any; 2],
	pub two_x_your_turn: sink::TwoXYourTurn,
	pub max_four: sink::MaxFour,
	pub set_one: [sink::SetOne; 4],
}

impl SinksValueMorfi {
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut dyn sink::Sink> {
		let mut ret = Vec::with_capacity(8);
		ret.extend(self.any.iter_mut().map(|sink| sink as &mut dyn sink::Sink));
		ret.push((&mut self.two_x_your_turn) as &mut dyn sink::Sink);
		ret.push((&mut self.max_four) as &mut dyn sink::Sink);
		ret.extend(self.set_one.iter_mut().map(|s| s as &mut dyn sink::Sink));
		ret.into_iter()
	}

	pub fn iter(&self) -> impl Iterator<Item = &dyn sink::Sink> {
		let mut ret = Vec::with_capacity(8);
		ret.extend(self.any.iter().map(|sink| sink as &dyn sink::Sink));
		ret.push((&self.two_x_your_turn) as &dyn sink::Sink);
		ret.push((&self.max_four) as &dyn sink::Sink);
		ret.extend(self.set_one.iter().map(|s| s as &dyn sink::Sink));
		ret.into_iter()
	}

	pub fn score(&self) -> Score {
		self.iter().map(|s| s.score()).sum()
	}
}
