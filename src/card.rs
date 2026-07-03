pub mod modifier;
pub mod sink;

#[derive(Default)]
pub struct Morfi {
	modifiers: ModifiersMorfi,
	sinks: SinksMorfi,
}

#[derive(Default)]
pub struct ModifiersMorfi {
	copy: modifier::Copy,
	squish: modifier::Squish,
	morph: modifier::Morph,
}

#[derive(Default)]
pub struct SinksMorfi {
	// bingo: SinkBingoMorfi,
	value: SinksValueMorfi,
}

#[derive(Default)]
pub struct SinksValueMorfi {
	any: [sink::Any; 2],
	two_x_your_turn: sink::TwoXYourTurn,
	max_four: sink::MaxFour,
	set_one: [sink::SetOne; 4],
}

impl SinksValueMorfi {
	// pub fn iter_mut(&self) -> impl Iterator<Item = &mut sink::Any> {
	// 	todo!()
	// }
}
