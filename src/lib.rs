#![allow(unused_comparisons)]

use crate::{card::modifier, prelude::*};

pub mod app_tracing;
pub mod card;
pub mod prelude;

/// This assumption is cross cutting, use this in assertion error messages
pub const ALWAYS_VALID_SINK: &str = "ESINK to always have a valid sink";

/// 1 to 12 only
pub type DiceRoll = u8;

#[ensures(0 < ret && ret <= 8)]
pub fn roll() -> DiceRoll {
	::rand::random::<u8>() % 8 + 1
}

/// [DiceRoll] after any modifiers, can include negatives
pub type AnyNumber = i16;

pub type Score = AnyNumber;

#[derive(Default, Clone)]
pub struct GameState {
	your_turn: bool,
	next_resolve_stack: Option<AnyNumber>,
}

impl GameState {
	pub fn your_turn(mut self, val: bool) -> Self {
		self.your_turn = val;
		self
	}
}

impl GameState {
	pub fn add_number_to_resolve_stack(&mut self, number: AnyNumber) {
		if self.next_resolve_stack.is_some() {
			todo!("Stack should only ever be one deep")
		}
		self.next_resolve_stack = Some(number);
	}
}

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

	#[ensures(self.turn_num() <= 12)]
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
			.your_turn(self.your_turn())
	}
}

pub struct RandomStrategy;

impl RandomStrategy {
	// pub fn apply(&mut self, game: BasicGame)
}
