use crate::{AnyNumber, prelude::*};

#[derive(Default, Clone)]
pub struct GameState {
	your_turn: bool,
	next_resolve_stack: Option<AnyNumber>,
}

impl GameState {
	pub fn with_your_turn(mut self, val: bool) -> Self {
		self.your_turn = val;
		self
	}

	pub fn your_turn(&self) -> bool {
		self.your_turn
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
