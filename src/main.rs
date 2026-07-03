use rand::seq::IteratorRandom;
use ready_z_go::{AnyNumber, BasicGame, card::{modifier::{Modifier as _, SquishOptions}, sink::Sink}, roll};
use tracing::*;

fn main() -> color_eyre::Result<()> {
	ready_z_go::app_tracing::init_debug_tools("ready_z_go=trace").unwrap();
	debug!("Logging started");

	info!("Hello, world!");

	let rng = &mut rand::rng();

	let mut game = BasicGame::new();
	let num = roll() as AnyNumber;
	info!("Score: {}", game.card.score());

	{
		let mut state = game.state();
		let num = game.card.modifiers.squish.r#use(&mut state, SquishOptions::Bump, num)?;
		debug!("New dice: {}", num);

		// game.card.sinks.value.any[0].fill(&state, dice)?;
		let sink = game.card.sinks.iter_mut().choose(rng).expect("A sink");
		if sink.can_fill(&state, num).is_ok() {
			sink.fill(&state, num)?;

			info!("Score: {}", game.card.score());
			game.next_turn();
		} else {
			warn!("Couldn't fill randomly chosen sink");
		}
	}

	Ok(())
}
