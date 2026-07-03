pub mod app_tracing;
pub mod card;
pub mod prelude;

/// 1 to 12 only
pub type DiceRoll = u8;

/// [DiceRoll] after any modifiers, can include negatives
pub type AnyNumber = i8;

pub type Score = AnyNumber;

#[derive(Default, Clone)]
pub struct GameState {
	your_turn: bool,
}

pub struct BasicGame {
	turn_num: u8,
	state: GameState,
}

impl BasicGame {
	pub fn new() -> BasicGame {
		BasicGame {
			turn_num: 0,
			state: GameState { your_turn: true },
		}
	}
}
