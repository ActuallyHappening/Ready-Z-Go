use rand::seq::IteratorRandom;
use ready_z_go::{
	ALWAYS_VALID_SINK, AnyNumber, BasicGame, Score, card::{
		modifier::{Modifier as _, SquishOptions},
		sink::Sink,
	}, roll,
};
use tracing::*;

fn main() -> color_eyre::Result<()> {
	ready_z_go::app_tracing::init_debug_tools("ready_z_go=trace").unwrap();
	debug!("Logging started");
	info!("Hello, world!");
	let rng = &mut rand::rng();

	let mut game = BasicGame::new();
	// what is the average value using no modifiers?
	// run N games randomly and average scores, assuming 2 your-turns and assuming random slot filling
	const N: usize = 10;
	let mut scores = Vec::<Score>::with_capacity(N);

	let num = roll() as AnyNumber;

	for _ in 0..12 {
		let state = game.state();

		let sink = game
			.card
			.sinks
			.iter_mut()
			.filter(|sink| sink.can_fill(&state, num).is_ok())
			.choose(rng)
			.expect(ALWAYS_VALID_SINK);

		sink.fill(&state, num)?;

		game.next_turn();
	}

	Ok(())
}
