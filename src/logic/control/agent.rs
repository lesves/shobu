use crate::logic::game::{Color, GameState};
use crate::logic::movegen::{Move, MoveGen};
use rand::seq::SliceRandom;


pub trait Agent {
	fn act(&mut self, movegen: &mut MoveGen, state: GameState) -> Option<Move>;
}

pub struct RandomAgent;

impl Agent for RandomAgent {
	fn act(&mut self, gen: &mut MoveGen, state: GameState) -> Option<Move> {
		gen.moves(state).choose(&mut rand::thread_rng()).copied()
	}
}

use super::evaluation::{Evaluation, Score};
use std::cmp::{min, max};

pub struct MinimaxAgent {
	depth: u32,
	evaluate: Evaluation,
}

impl MinimaxAgent {
	pub fn new(depth: u32, evaluate: Evaluation) -> MinimaxAgent {
		MinimaxAgent {
			depth,
			evaluate,
		}
	}

	fn search(&self, gen: &mut MoveGen, state: GameState, depth: u32, mut alpha: Score, mut beta: Score) -> Score {
		if depth == 0 || state.winner().is_some() {
			return (self.evaluate)(state);
		}

		let children = gen.moves(state).into_iter().map(|mv| mv.apply(state).expect("move generator failed"));
		match state.current_side {
			Color::Black => {
				// Maximize
				let mut val = Score::MIN+1;

				for child in children {
					val = max(val, self.search(gen, child, depth-1, alpha, beta));
					if val > beta {
						break;
					}
					alpha = max(alpha, val);
				}

				val
			},
			Color::White => {
				// Minimize
				let mut val = Score::MAX;

				for child in children {
					val = min(val, self.search(gen, child, depth-1, alpha, beta));
					if val < alpha {
						break;
					}
					beta = min(beta, val);
				}

				val
			},
		}
	}
}

impl Agent for MinimaxAgent {
	fn act(&mut self, gen: &mut MoveGen, state: GameState) -> Option<Move> {
		gen.moves(state).into_iter().max_by_key(|mv| {
			let score = self.search(gen, mv.apply(state).expect("move generator failed"), self.depth-1, Score::MIN, Score::MAX);
			match state.current_side {
				Color::Black => score,
				Color::White => -score,
			}
		})
	}
}
