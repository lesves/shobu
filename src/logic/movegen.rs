use super::board::Board;
use super::game::{Color, GameState};
use super::halfmove::{HalfMove, MoveVector};
use std::collections::HashMap;

// Move data structure

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Move {
	pub passive: HalfMove,
	pub active: HalfMove,
}

impl Move {
	pub fn is_valid(&self, state: GameState) -> bool {
		let passive_board = self.passive.from().board();
		let active_board = self.active.from().board();

		self.passive.is_valid_passive(state.friendly() | state.enemy()) && 
		self.active.is_valid_active(state.friendly(), state.enemy()) && (
			passive_board.other_board(Color::White) == active_board ||
			passive_board.other_board(Color::Black) == active_board
		)
	}

	pub fn apply(&self, state: GameState) -> Option<GameState> {
		if !self.is_valid(state) {
			None
		} else {
			let mut friendly = state.friendly();
			let mut enemy = state.enemy();

			friendly.clear(self.passive.from());
			friendly.set(self.passive.to());

			let from = self.active.from();
			let to = self.active.to();
			friendly.clear(from);

			let v = self.active.vector();
			let in_path = v.apply_with_size(from, 1).unwrap();
			let enemy_in_path = v.size.value() == 2 && enemy.get(in_path);

			if enemy.get(to) || enemy_in_path {
				if enemy_in_path {
					enemy.clear(in_path);
				} else {
					enemy.clear(to);
				}
				match v.apply_with_size(from, v.size.value()+1) {
					Some(landing) => enemy.set(landing),
					None => {},
				}
			}
			friendly.set(to);

			Some(GameState::from_relative(
				!state.current_side,
				enemy,
				friendly,
			))
		}
	}
}

impl Board {
	pub fn other_board(&self, side: Color) -> Board {
		use Color::*;
		use Board::*;

		match (self, side) {
			(TopLeft, Black) => TopRight,
			(TopLeft, White) => BottomRight,
			(TopRight, Black) => TopLeft,
			(TopRight, White) => BottomLeft,

			(BottomLeft, White) => BottomRight,
			(BottomLeft, Black) => TopRight,
			(BottomRight, White) => BottomLeft,
			(BottomRight, Black) => TopLeft,
		}
	}
}

// Move generator

#[derive(Clone)]
pub struct MoveGen {
	cache: HashMap<GameState, Vec<Move>>,
}

impl MoveGen {
	pub fn new() -> MoveGen {
		MoveGen { cache: HashMap::new() }
	}

	pub fn moves(&mut self, state: GameState) -> Vec<Move> {
		if self.cache.contains_key(&state) {
			return self.cache[&state].clone();
		}
		let mut res = vec![];

		if state.winner().is_some() {
			return res;
		}

		for passive_board in state.current_side.home() {
			for active_board in [Color::White, Color::Black].map(|clr| passive_board.other_board(clr)) {
				let passive_pieces = passive_board.mask() & state.friendly();
				let active_pieces = active_board.mask() & state.friendly();

				for ve in MoveVector::iter() {
					for passive_piece in passive_pieces.iter() {
						for active_piece in active_pieces.iter() {
							match try {
								let passive = HalfMove::new(passive_piece, ve)?;
								let active = HalfMove::new(active_piece, ve)?;

								Move { passive, active }
							} {
								Some(mv) if mv.is_valid(state) => res.push(mv),
								_ => {}
							};
						}
					}
				}
			}
		}

		self.cache.insert(state, res.clone());
		res
	}

	pub fn states(&mut self, state: GameState) -> Vec<GameState> {
		self.moves(state).iter().map(|mv| mv.apply(state).unwrap()).collect()
	}
}