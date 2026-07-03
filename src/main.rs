use std::any::Any;

use rand::seq::IteratorRandom;
pub(crate) use rapidhash::{HashSetExt as _, RapidHashSet as HashSet};
use ready_z_go::{
	card::{
		self,
		modifier::{Modifier as _, SquishOptions},
		sink::{self, Sink, CAN_FILL_GUARENTEES_FILL_SUCCEEDS},
	},
	heuristics::{self, SinkHeuristic},
	roll, AnyNumber, BasicGame, Score, ALWAYS_VALID_SINK,
};
use tracing::*;

fn main() -> color_eyre::Result<()> {
	ready_z_go::app_tracing::init_debug_tools("ready_z_go=debug")?;
	debug!("Logging started");
	info!("Hello, world!");

	count_possibilities()
}

type Container<T> = rapidhash::RapidHashSet<T>;

/// Count ALL possible distinct end game states.
/// Distinct means what is written in the end, not just score,
/// and irrelevant of order.
///
/// - Assume no modifiers
/// - Assume 2 your-turns, first and last
/// - Assume taking no action is only allowed when no slots are permitted,
/// and the number of no-actions should be recorded
fn count_possibilities() -> color_eyre::Result<()> {
	let genesis = BasicGame::new();
	let start = {
		let mut start = Container::with_capacity(1);
		start.insert(genesis);
		start
	};

	let mut by_turn: Vec<Container<BasicGame>> = Vec::with_capacity(12);
	for turn_num in 0..12 {
		// init with upper bound
		let capacity_bounds = [
			40, 690, 6804, 43071, 187068, 582428, 1338920, 2316303, 3046912, 3054898, 2320836, 1314705,
		];
		by_turn.push(Container::with_capacity(capacity_bounds[turn_num]));
		// by_turn.push(Container::new());

		let previous_turn = if turn_num == 0 {
			&start
		} else {
			&by_turn[turn_num - 1].clone()
		};
		let this_turn = &mut by_turn[turn_num];

		for game in previous_turn {
			this_turn.extend(possibilities(game.clone()));
		}
		info!(%turn_num, "possibilities: {}", this_turn.len());
	}

	let last = by_turn.into_iter().last().unwrap();
	let score_total: u128 = last
		.iter()
		.fold(0, |acc, game| acc + game.card.score() as u128);

	let last_num = HashSet::<BasicGame>::from_iter(last).len();
	info!("Final number: {}", last_num);

	let average = score_total as f32 / last_num as f32;
	info!("Average score: {}", average);

	Ok(())
}

type Container2<T> = Vec<T>;

/// Takes every possible turn
fn possibilities(game: BasicGame) -> Container2<BasicGame> {
	// with_capacity probably worsens performance
	let mut possibilities = Container2::new();

	for num in 1..=8 {
		possibilities.extend(possibilities_num(game.clone(), num));
	}

	possibilities
}

fn possibilities_num(game: BasicGame, num: AnyNumber) -> Container2<BasicGame> {
	let mut possibilities = Container2::new();

	let mut heuristic = Some(heuristics::PreferSink::<sink::bingo::SinkBingoMorfi>::new(
		0..=8,
	));
	heuristic = None;

	let state = game.state();
	let initial_valid_sinks = game
		.card
		.sinks
		.iter()
		.filter(|sink| sink.can_fill(&state, num).is_ok())
		.collect::<Vec<_>>();
	let mut num_valid_sinks = initial_valid_sinks.len();

	{
		let after_heuristic = initial_valid_sinks
			.iter()
			.filter(|sink| heuristic.filter_sink(&game, **sink));
		let count = after_heuristic.count();
		if count != 0 {
			num_valid_sinks = count;
		} else {
			// heuristic too aggressive
			heuristic = None;
		}
	}

	for i in 0..num_valid_sinks {
		let mut sim_game = game.clone();
		let sim_state = sim_game.state();

		let sink = sim_game
			.card
			.sinks
			.iter_mut()
			.filter(|sink| sink.can_fill(&sim_state, num).is_ok())
			.filter(|sink| {
				heuristic.filter_sink(&game, *sink)
			})
			.nth(i)
			.expect("cloned games to have identical sinks");

		sink
			.fill(&sim_state, num)
			.expect(CAN_FILL_GUARENTEES_FILL_SUCCEEDS);
		sim_game.next_turn();

		possibilities.push(sim_game);
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
