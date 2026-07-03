use crate::{AnyNumber, GameState, card::sink, prelude::*};

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
				[(false, bingo[0][0]), (false, bingo[0][1]), (false, bingo[0][2])],
				[(false, bingo[1][0]), (false, bingo[1][1]), (false, bingo[1][2])],
				[(false, bingo[2][0]), (false, bingo[2][1]), (false, bingo[2][2])],
			]
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
		if self.left_diag() { count += 1; }
		if self.right_diag() { count += 1; }
		for row in 0..3 {
			if self.row(row) { count += 1; }
		}
		for col in 0..3 {
			if self.col(col) { count += 1; }
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

	/// panics
	fn try_fill(&mut self, index: (usize, usize)) {
		if self.bingo[index.0][index.1].0 {
			panic!("slot {:?} already filled", index);
		}
		self.bingo[index.0][index.1].0 = true;
	}
}

impl sink::Sink for SinkBingo {
	fn score(&self) -> crate::Score {
		(self.combos() * self.score).try_into().unwrap()
	}

	fn fill(&mut self, _state: GameState, num: AnyNumber) -> Result<crate::Score, sink::CannotSink> {
		if let Some(index) = self.try_find_slot(num) {
			self.try_fill(index);
			Ok(self.score())
		} else {
			Err(sink::CannotSink::InvalidValue)
		}
	}
}
