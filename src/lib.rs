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
	state: GameState,
	card: card::Morfi,
}

impl BasicGame {
	pub fn new() -> BasicGame {
		BasicGame {
			turn_num: 0,
			state: GameState::default().your_turn(true),
			card: card::Morfi::default(),
		}
	}
}

pub struct RandomStrategy;

impl RandomStrategy {
	// pub fn apply(&mut self, game: BasicGame)
}
