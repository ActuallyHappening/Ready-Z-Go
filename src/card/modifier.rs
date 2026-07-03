use crate::{prelude::*, AnyNumber, GameState};

// #[dyn_safe(true)]
trait Example {
	type Ass;
}

fn example() {
	let a: Box<dyn Example<Ass = i32>> = todo!();
}

// #[dyn_safe(true)]
pub trait Modifier {
	type ModifierOption;

	fn all_options(&self) -> impl IntoIterator<Item = Self::ModifierOption>;

	/// PERF: More efficient implementations exist for subtypes
	fn usable_options(&self, state: &GameState, input: AnyNumber) -> impl IntoIterator<Item = Self::ModifierOption> {
		let mut initial: Vec<Self::ModifierOption> = self.all_options().into_iter().collect();
		initial.retain(|option| self.can_use(&state, option, input));
		initial
	}

	/// Returns output
	fn r#use(
		&mut self,
		state: &mut GameState,
		option: Self::ModifierOption,
		input: AnyNumber,
	) -> Result<AnyNumber, CannotModify>;

	fn can_use(&self, state: &GameState, option: &Self::ModifierOption, input: AnyNumber) -> bool;
}

#[derive(Default, Clone)]
pub struct Copy {
	used: bool,
}

impl Modifier for Copy {
	type ModifierOption = ();
	fn all_options(&self) -> impl IntoIterator<Item = Self::ModifierOption> {
		[()]
	}

	fn r#use(
		&mut self,
		state: &mut GameState,
		_option: Self::ModifierOption,
		input: AnyNumber,
	) -> Result<AnyNumber, CannotModify> {
		if self.used {
			return Err(CannotModify::AlreadyUsed);
		}
		self.used = true;

		state.add_number_to_resolve_stack(input);
		Ok(input)
	}


	fn can_use(&self, state: &GameState, option: &Self::ModifierOption, input: AnyNumber) -> bool {
		let mut clone = self.clone();
		let mut state = state.clone();
		let option = option.clone();
		clone.r#use(&mut state, option, input).is_ok()
	}
}

#[derive(thiserror::Error, Debug)]
pub enum CannotModify {
	#[error("modifier already used")]
	AlreadyUsed,
}

#[derive(Default, Clone)]
pub struct Squish {
	used: bool,
}

#[derive(Clone)]
pub enum SquishOptions {
	/// +1
	Bump,
	/// -1
	Chip,
}

impl Modifier for Squish {
	type ModifierOption = SquishOptions;
	fn all_options(&self) -> impl IntoIterator<Item = Self::ModifierOption> {
		[SquishOptions::Bump, SquishOptions::Chip]
	}
	fn r#use(&mut self, _state: &mut GameState, option: Self::ModifierOption, input: AnyNumber) -> Result<AnyNumber, CannotModify> {
		if self.used {
			return Err(CannotModify::AlreadyUsed);
		}
		self.used = true;

		match option {
			SquishOptions::Bump => Ok(input + 1),
			SquishOptions::Chip => Ok(input - 1),
		}
	}


	fn can_use(&self, state: &GameState, option: &Self::ModifierOption, input: AnyNumber) -> bool {
		let mut clone = self.clone();
		let mut state = state.clone();
		let option = option.clone();
		clone.r#use(&mut state, option, input).is_ok()
	}
}

#[derive(Default, Clone)]
pub struct Morph {
	used: bool
}

#[derive(Clone)]
pub enum MorphOptions {
	Bump,
	/// +2
	DoubleBump,
	Chip,
	/// -2
	DoubleChip,
}

impl Modifier for Morph {
	type ModifierOption = MorphOptions;

	fn all_options(&self) -> impl IntoIterator<Item = Self::ModifierOption> {
		[MorphOptions::Bump, MorphOptions::DoubleBump, MorphOptions::Chip, MorphOptions::DoubleChip]
	}

	fn r#use(&mut self, _state: &mut GameState, option: Self::ModifierOption, input: AnyNumber) -> Result<AnyNumber, CannotModify> {
		if self.used {
			return Err(CannotModify::AlreadyUsed);
		}
		self.used = true;

		match option {
			MorphOptions::Bump => Ok(input + 1),
			MorphOptions::DoubleBump => Ok(input + 2),
			MorphOptions::Chip => Ok(input - 1),
			MorphOptions::DoubleChip => Ok(input - 2),
		}
	}


	fn can_use(&self, state: &GameState, option: &Self::ModifierOption, input: AnyNumber) -> bool {
		let mut clone = self.clone();
		let mut state = state.clone();
		let option = option.clone();
		clone.r#use(&mut state, option, input).is_ok()
	}
}
