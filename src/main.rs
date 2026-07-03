use rand::seq::IteratorRandom;
use ready_z_go::{
	card::{
		modifier::{Modifier as _, SquishOptions},
		sink::Sink,
	},
	roll, AnyNumber, BasicGame, Score, ALWAYS_VALID_SINK,
};
use tracing::*;

fn main() -> color_eyre::Result<()> {
	ready_z_go::app_tracing::init_debug_tools("ready_z_go=trace").unwrap();
	debug!("Logging started");
	info!("Hello, world!");
	let rng = &mut rand::rng();

	// what is the average value using no modifiers?
	// run N games randomly and average scores,
	// - assuming 2 your-turns
	// - assuming random slot filling
	// - assuming no modifiers
	// - assuming taking no action is always allowed
	const N: usize = 100;
	let mut scores = Vec::<Score>::with_capacity(N);

	for _n in 0..N {
		let mut game = BasicGame::new();

		for _ in 0..12 {
			let num = roll(rng) as AnyNumber;
			let state = game.state();
			let sink = game
				.card
				.sinks
				.iter_mut()
				.filter(|sink| sink.can_fill(&state, num).is_ok())
				.choose(rng);
			let Some(sink) = sink else {
				// assumption of doing nothing
				game.next_turn();
				continue;
			};
			sink.fill(&state, num)?;
			game.next_turn();
		}

		scores.push(game.card.score());
		debug!(score = %game.card.score(), "Game simulated");
	}

	let average = scores.into_iter().sum::<i16>() as f32 / N as f32;
	info!(%average);

	Ok(())
}
