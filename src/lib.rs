#![allow(unused_comparisons)]

use crate::{card::modifier, prelude::*, state::GameState};

pub mod app_tracing;
pub mod card;
pub mod prelude;
pub mod state;

/// This assumption is cross cutting, use this in assertion error messages.
/// This assumption isn't always true, but when using "reasonable" strategies
/// it should be true
pub const ALWAYS_VALID_SINK: &str = "ESINK to always have a valid sink";

/// 1 to 12 only
pub type DiceRoll = u8;

#[ensures(0 < ret && ret <= 8)]
pub fn roll(rng: &mut impl rand::Rng) -> DiceRoll {
	rng.random::<u8>() % 8 + 1
}

/// [DiceRoll] after any modifiers, can include negatives
pub type AnyNumber = i16;

pub type Score = AnyNumber;

pub struct BasicGame {
	turn_num: u8,
	pub card: card::Morfi,
}

impl BasicGame {
	pub fn new() -> BasicGame {
		BasicGame {
			turn_num: 0,
			card: card::Morfi::default(),
		}
	}

	/// 0 indexed.
	/// Don't call on a finished game.
	#[requires(!self.finished())]
	#[ensures(0 <= ret && ret < 12)]
	pub fn turn_num(&self) -> u8 {
		self.turn_num
	}

	#[ensures(self.turn_num <= 12)]
	pub fn next_turn(&mut self) {
		if self.turn_num == 12 {
			return;
		}
		self.turn_num += 1;
	}

	pub fn finished(&self) -> bool {
		self.turn_num == 12
	}

	/// First and last roll
	pub fn your_turn(&self) -> bool {
		self.turn_num() == 0 || self.turn_num() == 11
	}

	pub fn state(&self) -> GameState {
		GameState::default()
			.with_your_turn(self.your_turn())
	}
}

pub struct RandomStrategy;

impl RandomStrategy {
	// pub fn apply(&mut self, game: BasicGame)
}
