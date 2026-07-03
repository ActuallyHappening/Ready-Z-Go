pub mod modifier;
pub mod sink;

pub struct CardMorfi {
	modifiers: ModifiersMorfi,
	sinks: SinksMorfi,
}

pub struct ModifiersMorfi {
	copy: modifier::Copy,
	squish: modifier::Squish,
	morph: modifier::Morph,
}

pub struct SinksMorfi {
	// bingo: SinkBingoMorfi,
	value: SinksValueMorfi,
}

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
