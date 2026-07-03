use crate::{card::sink, prelude::*, AnyNumber, GameState};

#[derive(Clone)]
pub struct SinkBingo {
	score: u8,
	bingo: [[(bool, AnyNumber); 3]; 3],
}

impl SinkBingo {
	pub fn new(bingo: [[AnyNumber; 3]; 3]) -> Self {
		Self {
			score: 4,
			bingo: [
				[
					(false, bingo[0][0]),
					(false, bingo[0][1]),
					(false, bingo[0][2]),
				],
				[
					(false, bingo[1][0]),
					(false, bingo[1][1]),
					(false, bingo[1][2]),
				],
				[
					(false, bingo[2][0]),
					(false, bingo[2][1]),
					(false, bingo[2][2]),
				],
			],
		}
	}

	pub fn get(&self, row: usize, col: usize) -> bool {
		self.bingo[row][col].0
	}

	pub fn left_diag(&self) -> bool {
		self.bingo[0][0].0 && self.bingo[1][1].0 && self.bingo[2][2].0
	}

	pub fn right_diag(&self) -> bool {
		self.bingo[0][2].0 && self.bingo[1][1].0 && self.bingo[2][0].0
	}

	pub fn row(&self, row: usize) -> bool {
		self.bingo[row][0].0 && self.bingo[row][1].0 && self.bingo[row][2].0
	}

	pub fn col(&self, col: usize) -> bool {
		self.bingo[0][col].0 && self.bingo[1][col].0 && self.bingo[2][col].0
	}

	#[ensures(ret <= 8)]
	pub fn combos(&self) -> u8 {
		let mut count = 0;
		if self.left_diag() {
			count += 1;
		}
		if self.right_diag() {
			count += 1;
		}
		for row in 0..3 {
			if self.row(row) {
				count += 1;
			}
		}
		for col in 0..3 {
			if self.col(col) {
				count += 1;
			}
		}
		count
	}

	fn try_find_slot(&self, num: AnyNumber) -> Option<(usize, usize)> {
		for row in 0..3 {
			for col in 0..3 {
				if self.bingo[row][col].1 == num {
					return Some((row, col));
				}
			}
		}
		None
	}

	fn try_fill(&mut self, index: (usize, usize)) -> Result<(), sink::CannotSink> {
		if self.bingo[index.0][index.1].0 {
			return Err(sink::CannotSink::AlreadyFilled);
		}
		self.bingo[index.0][index.1].0 = true;
		Ok(())
	}
}

impl sink::Sink for SinkBingo {
	fn score(&self) -> crate::Score {
		(self.combos() * self.score).try_into().unwrap()
	}

	fn fill(&mut self, state: &GameState, num: AnyNumber) -> Result<crate::Score, sink::CannotSink> {
		self.can_fill(state, num)?;

		let index = self.try_find_slot(num).ok_or_else(|| unreachable!())?;
		self.try_fill(index)?;
		Ok(self.score())
	}

	fn can_fill(&self, _state: &GameState, num: AnyNumber) -> Result<(), sink::CannotSink> {
		let Some((row, col)) = self.try_find_slot(num) else {
			// number not in bingo
			return Err(sink::CannotSink::InvalidValue);
		};
		if self.get(row, col) {
			return Err(sink::CannotSink::AlreadyFilled);
		}
		Ok(())
	}
}

/// PERF: I'm pretty sure the compiler is smart enough to realise this
/// isomorphism.
///
/// [1, 2, 3], [8, 0, 4], [7, 6, 5]
#[derive(Default, Clone, Hash, PartialEq, Eq)]
pub struct SinkBingoMorfi {
	bingo: [[bool; 3]; 3],
}

impl From<SinkBingoMorfi> for SinkBingo {
	fn from(bingo: SinkBingoMorfi) -> Self {
		Self {
			score: 4,
			bingo: [
				[(bingo.bingo[0][0], 1), (bingo.bingo[0][1], 2), (bingo.bingo[0][2], 3)],
				[(bingo.bingo[1][0], 8), (bingo.bingo[1][1], 0), (bingo.bingo[1][2], 4)],
				[(bingo.bingo[2][0], 7), (bingo.bingo[2][1], 6), (bingo.bingo[2][2], 5)],
			],
		}
	}
}

impl From<SinkBingo> for SinkBingoMorfi {
	fn from(bingo: SinkBingo) -> Self {
		Self {
			bingo: [
				[bingo.bingo[0][0].0, bingo.bingo[0][1].0, bingo.bingo[0][2].0],
				[bingo.bingo[1][0].0, bingo.bingo[1][1].0, bingo.bingo[1][2].0],
				[bingo.bingo[2][0].0, bingo.bingo[2][1].0, bingo.bingo[2][2].0],
			],
		}
	}
}

impl sink::Sink for SinkBingoMorfi {
	fn score(&self) -> crate::Score {
		SinkBingo::from(self.clone()).score()
	}

	fn fill(&mut self, state: &GameState, num: AnyNumber) -> Result<crate::Score, sink::CannotSink> {
		let mut this = SinkBingo::from(self.clone());
		let score = this.fill(state, num)?;
		*self = SinkBingoMorfi::from(this);
		Ok(score)
	}

	fn can_fill(&self, _state: &GameState, num: AnyNumber) -> Result<(), sink::CannotSink> {
		SinkBingo::from(self.clone()).can_fill(_state, num)
	}
}
