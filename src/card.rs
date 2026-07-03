use crate::card::sink::bingo::SinkBingo;

pub mod modifier;
pub mod sink;

#[derive(Default)]
pub struct Morfi {
	pub modifiers: ModifiersMorfi,
	pub sinks: SinksMorfi,
}

#[derive(Default)]
pub struct ModifiersMorfi {
	pub copy: modifier::Copy,
	pub squish: modifier::Squish,
	pub morph: modifier::Morph,
}

pub struct SinksMorfi {
	pub bingo: SinkBingo,
	pub value: SinksValueMorfi,
}

impl Default for SinksMorfi {
	fn default() -> Self {
		Self {
			bingo: SinkBingo::new([[1, 2, 3], [8, 0, 4], [7, 6, 5]]),
			value: SinksValueMorfi::default(),
		}
	}
}

#[derive(Default)]
pub struct SinksValueMorfi {
	pub any: [sink::Any; 2],
	pub two_x_your_turn: sink::TwoXYourTurn,
	pub max_four: sink::MaxFour,
	pub set_one: [sink::SetOne; 4],
}

impl SinksValueMorfi {
	// pub fn iter_mut(&self) -> impl Iterator<Item = &mut sink::Any> {
	// 	todo!()
	// }
}
