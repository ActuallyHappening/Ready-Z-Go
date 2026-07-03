use crate::{prelude::*, AnyNumber, GameState, Score};

pub mod bingo;

/// Can eat exactly one dice roll
#[dyn_safe(true)]
pub trait Sink: std::any::Any {
	fn score(&self) -> Score;

	/// If [can_fill] succeeds, [fill] must succeed as well.
	fn can_fill(&self, state: &GameState, num: AnyNumber) -> Result<(), CannotSink>;

	fn fill(&mut self, state: &GameState, num: AnyNumber) -> Result<Score, CannotSink>;
}

pub const CAN_FILL_GUARENTEES_FILL_SUCCEEDS: &str = "ECANFILL can_fill to guarentee fill doesn't error";

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum CannotSink {
	#[error("sink already filled")]
	AlreadyFilled,
	#[error("invalid value for sink")]
	InvalidValue,
	#[error("invalid state to fill sink")]
	InvalidState,
}

#[derive(Default, Clone, Hash, PartialEq, Eq)]
pub struct Any(Option<AnyNumber>);
impl Sink for Any {
	fn score(&self) -> Score {
		self.0.unwrap_or_default()
	}

	fn fill(&mut self, state: &GameState, num: AnyNumber) -> Result<Score, CannotSink> {
		self.can_fill(state, num)?;
		self.0 = Some(num);
		trace!(%num, "Filled sink Any");
		Ok(num)
	}

	fn can_fill(&self, _state: &GameState, _num: AnyNumber) -> Result<(), CannotSink> {
		if self.0.is_some() {
			return Err(CannotSink::AlreadyFilled);
		}
		Ok(())
	}
}

#[derive(Default, Clone, Hash, PartialEq, Eq)]
pub struct TwoXYourTurn(Option<AnyNumber>);
impl Sink for TwoXYourTurn {
	fn score(&self) -> Score {
		self.0.map(|n| n * 2).unwrap_or_default()
	}

	fn can_fill(&self, state: &GameState, _num: AnyNumber) -> Result<(), CannotSink> {
		if self.0.is_some() {
			return Err(CannotSink::AlreadyFilled);
		}
		if !state.your_turn() {
			return Err(CannotSink::InvalidState);
		}
		Ok(())
	}

	fn fill(&mut self, state: &GameState, num: AnyNumber) -> Result<Score, CannotSink> {
		self.can_fill(state, num)?;
		self.0 = Some(num);
		trace!(%num, "Filled sink x2 on your roll");
		Ok(num)
	}
}

#[derive(Default, Clone, Hash, PartialEq, Eq)]
pub struct MaxFour(Option<AnyNumber>);
impl Sink for MaxFour {
	#[ensures(ret <= 4)]
	fn score(&self) -> Score {
		// max 4
		self.0.unwrap_or_default()
	}

	#[ensures(self.score() <= 4)]
	fn fill(&mut self, state: &GameState, num: AnyNumber) -> Result<Score, CannotSink> {
		self.can_fill(state, num)?;
		self.0 = Some(num.min(4));
		trace!(%num, "Filled sink <=4 max value of 4");
		Ok(num)
	}
	fn can_fill(&self, _state: &GameState, num: AnyNumber) -> Result<(), CannotSink> {
		if self.0.is_some() {
			return Err(CannotSink::AlreadyFilled);
		}
		if !(num <= 4) {
			return Err(CannotSink::InvalidValue);
		}
		Ok(())
	}
}

#[test]
fn test_max_four() {
	{
		let empty = MaxFour(None);
		assert!(empty.score() == 0);
	}
	{
		let mut empty = MaxFour(None);
		empty
			.fill(&GameState::default(), AnyNumber::from(3i16))
			.unwrap();
		assert_eq!(empty.score(), 3);
	}
	{
		let mut empty = MaxFour(None);
		let res = empty.fill(&GameState::default(), AnyNumber::from(6i16));
		assert_eq!(res, Err(CannotSink::InvalidValue));
	}
}

#[derive(Default, Clone, Hash, PartialEq, Eq)]
pub struct SetOne<const N: u8> {
	filled: u8,
}
impl<const N: u8> Sink for SetOne<N> {
	#[ensures(ret <= N as i16)]
	fn score(&self) -> Score {
		// 1 each
		self.filled as i16
	}

	fn can_fill(&self, _state: &GameState, _num: AnyNumber) -> Result<(), CannotSink> {
		if self.filled > N {
			unreachable!()
		}
		if self.filled == N {
			return Err(CannotSink::AlreadyFilled);
		}
		Ok(())
	}

	fn fill(&mut self, state: &GameState, num: AnyNumber) -> Result<Score, CannotSink> {
		self.can_fill(state, num)?;
		self.filled += 1;
		trace!(%num, "Filled sink Set1 filled={}", self.filled);
		Ok(num)
	}
}
