use std::collections::HashSet;

use rand::seq::IteratorRandom;
use ready_z_go::{
	card::{
		self,
		modifier::{Modifier as _, SquishOptions},
		sink::{Sink, CAN_FILL_GUARENTEES_FILL_SUCCEEDS},
	},
	roll, AnyNumber, BasicGame, Score, ALWAYS_VALID_SINK,
};
use tracing::*;

fn main() -> color_eyre::Result<()> {
	ready_z_go::app_tracing::init_debug_tools("ready_z_go=debug")?;
	debug!("Logging started");
	info!("Hello, world!");

	count_possibilities()
}

/// Count ALL possible distinct end game states.
/// Distinct means what is written in the end, not just score,
/// and irrelevant of order.
///
/// - Assume no modifiers
/// - Assume 2 your-turns, first and last
/// - Assume taking no action is only allowed when no slots are permitted,
/// and the number of no-actions should be recorded
fn count_possibilities() -> color_eyre::Result<()> {
	let rng = &mut rand::rng();

	let genesis = BasicGame::new();
	let start = { let mut start = HashSet::with_capacity(1); start.insert(genesis); start };

	let mut by_turn: Vec<HashSet<BasicGame>> = Vec::with_capacity(12);
	for turn_num in 0..12 {
		// init with upper bound
		if turn_num == 0 {
			by_turn.push(HashSet::with_capacity(9));
		} else {
			// upper bound for 6th turn is max 9-6=3 possibilities per existing state
			// let capacity_upper_bound = (9 - turn_num) ^ by_turn[turn_num - 1].len();
			// by_turn.push(HashSet::with_capacity(capacity_upper_bound));
			by_turn.push(HashSet::new());
		}
		let previous_turn: &HashSet<BasicGame> = if turn_num == 0 { &start } else { &by_turn[turn_num - 1].clone() };
		let this_turn: &mut HashSet<BasicGame> = &mut by_turn[turn_num];

		for game in previous_turn {
			this_turn.extend(possibilities(game.clone()));
		}
		info!(%turn_num, "possibilities: {}", this_turn.len());
	}

	Ok(())
}

/// Takes every possible turn
fn possibilities(game: BasicGame) -> HashSet<BasicGame> {
	// with_capacity probably worsens performance
	let mut possibilities = HashSet::new();

	for num in 1..=8 {
		possibilities.extend(possibilities_num(game.clone(), num));
	}

	possibilities
}

fn possibilities_num(game: BasicGame, num: AnyNumber) -> HashSet<BasicGame> {
	let mut possibilities = HashSet::new();

	let state = game.state();
	let num_valid_sinks = game
		.card
		.sinks
		.iter()
		.filter(|sink| sink.can_fill(&state, num).is_ok())
		.count();

	for i in 0..num_valid_sinks {
		let mut sim_game = game.clone();
		let sim_state = sim_game.state();
		let sink = sim_game
			.card
			.sinks
			.iter_mut()
			.filter(|sink| sink.can_fill(&sim_state, num).is_ok())
			.nth(i)
			.expect("cloned games to have identical sinks");

		sink
			.fill(&sim_state, num)
			.expect(CAN_FILL_GUARENTEES_FILL_SUCCEEDS);
		sim_game.next_turn();

		possibilities.insert(sim_game);
	}

	possibilities
}

fn basic_random() -> color_eyre::Result<()> {
	let rng = &mut rand::rng();
	// what is the average value using no modifiers?
	// run N games randomly and average scores,
	// - assuming 2 your-turns
	// - assuming random slot filling
	// - assuming no modifiers
	// - assuming taking no action is allowed if no slots are permitted
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
