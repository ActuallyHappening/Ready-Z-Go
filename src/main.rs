use ready_z_go::{AnyNumber, BasicGame, card::{modifier::{Modifier as _, SquishOptions}, sink::Sink}, roll};
use tracing::*;

fn main() -> color_eyre::Result<()> {
	ready_z_go::app_tracing::init_debug_tools("ready_z_go=debug").unwrap();
	debug!("Logging started");

	info!("Hello, world!");

	let mut game = BasicGame::new();
	let dice = roll() as AnyNumber;

	{
		let mut state = game.state();
		let dice = game.card.modifiers.squish.r#use(&mut state, SquishOptions::Bump, dice)?;
		game.card.sinks.value.any[0].fill(&state, dice)?;
	}

	Ok(())
}
