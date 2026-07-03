#![allow(unused_comparisons)]

use crate::prelude::*;

pub mod app_tracing;
pub mod card;
pub mod prelude;

/// 1 to 12 only
pub type DiceRoll = u8;

#[ensures(0 <= ret && ret <= 12)]
pub fn roll() -> DiceRoll {
	::rand::random::<u8>() % 12
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

	/// 0 indexed
	#[ensures(0 <= ret && ret < 12)]
	pub fn turn_num(&self) -> u8 {
		self.turn_num
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
