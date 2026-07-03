use crate::{prelude::*, AnyNumber, GameState, Score};

pub trait Sink: Clone {
	fn score(&self) -> Option<Score>;

	fn fill(&mut self, state: GameState, num: AnyNumber) -> Result<Score, CannotSink>;

	fn can_fill(&self, state: GameState, num: AnyNumber) -> Result<(), CannotSink> {
		let mut clone = self.clone();
		clone.fill(state, num)?;
		Ok(())
	}
}

#[derive(Debug, PartialEq)]
pub enum CannotSink {
	AlreadyFilled,
	InvalidValue,
	InvalidState,
}

#[derive(Clone)]
pub struct Any(Option<AnyNumber>);
impl Sink for Any {
	fn score(&self) -> Option<Score> {
		self.0
	}
	fn fill(&mut self, _state: GameState, num: AnyNumber) -> Result<Score, CannotSink> {
		if self.0.is_some() {
			return Err(CannotSink::AlreadyFilled);
		}
		self.0 = Some(num);
		Ok(num)
	}
}

#[derive(Clone)]
pub struct TwoXYourTurn(Option<AnyNumber>);
impl Sink for TwoXYourTurn {
	fn score(&self) -> Option<Score> {
		self.0.map(|n| n * 2)
	}
	fn fill(&mut self, state: GameState, num: AnyNumber) -> Result<Score, CannotSink> {
		if self.0.is_some() {
			return Err(CannotSink::AlreadyFilled);
		}
		if !state.your_turn {
			return Err(CannotSink::InvalidState);
		}
		self.0 = Some(num);
		Ok(num)
	}
}

#[derive(Clone)]
pub struct MaxFour(Option<AnyNumber>);
impl Sink for MaxFour {
	#[ensures(ret.is_none_or(|n| n <= 4))]
	fn score(&self) -> Option<Score> {
		// max 4
		self.0.map(|n| n.min(4))
	}

	fn fill(&mut self, _state: GameState, num: AnyNumber) -> Result<Score, CannotSink> {
		if self.0.is_some() {
			return Err(CannotSink::AlreadyFilled);
		}
		if !(num <= 4) {
			return Err(CannotSink::InvalidValue);
		}
		self.0 = Some(num);
		Ok(num)
	}
}

#[test]
fn test_max_four() {
	{
		let empty = MaxFour(None);
		assert!(empty.score().is_none());
	}
	{
		let mut empty = MaxFour(None);
		empty.fill(GameState::default(), AnyNumber::from(3)).unwrap();
		assert_eq!(empty.score(), Some(3));
	}
	{
		let mut empty = MaxFour(None);
		let res = empty.fill(GameState::default(), AnyNumber::from(6));
		assert_eq!(res, Err(CannotSink::InvalidValue));
	}
}

#[derive(Clone)]
pub struct SetOne(Option<AnyNumber>);
impl Sink for SetOne {
	fn score(&self) -> Option<Score> {
		// set 1
		self.0.map(|_| 1)
	}
	fn fill(&mut self, _state: GameState, num: AnyNumber) -> Result<Score, CannotSink> {
		if self.0.is_some() {
			return Err(CannotSink::AlreadyFilled);
		}
		self.0 = Some(num);
		Ok(num)
	}
}
