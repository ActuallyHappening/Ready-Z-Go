use ready_z_go::{AnyNumber, BasicGame, card::sink::Sink, roll};
use tracing::*;

fn main() -> color_eyre::Result<()> {
	ready_z_go::app_tracing::init_debug_tools("ready_z_go=debug").unwrap();
	debug!("Logging started");

	info!("Hello, world!");

	let mut game = BasicGame::new();
	let first_roll = roll();

	{
		let mut state = game.state();
		game.card.sinks.value.any[0].fill(state, first_roll as AnyNumber)?;
	}

	Ok(())
}
