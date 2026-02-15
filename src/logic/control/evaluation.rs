use crate::logic::board::Board;
use crate::logic::game::{Color, GameState};


pub type Score = i64; // higher score => better position for black
pub type Evaluation = fn(GameState) -> Score;//dyn Fn(GameState) -> Score;


pub fn boardwise_squaring(state: GameState) -> Score {
	match state.winner() {
		Some(clr) => if clr == Color::Black { i64::MAX } else { i64::MIN+1 },
		None => {
			let mut res = 0i64;
			for board in Board::iter() {
				let diff = (board.mask() & state.blacks).population() as i64 - 
					(board.mask() & state.whites).population() as i64;
				res += diff*diff*diff.signum();
			}
			res
		}
	}
}

pub fn boardwise(state: GameState) -> Score {
	match state.winner() {
		Some(clr) => if clr == Color::Black { i64::MAX } else { i64::MIN+1 },
		None => {
			let mut res = 0i64;
			for board in Board::iter() {
				res += (board.mask() & state.blacks).population() as i64 - 
					(board.mask() & state.whites).population() as i64;
			}
			res
		}
	}
}

pub fn simple(state: GameState) -> Score {
	match state.winner() {
		Some(clr) => if clr == Color::Black { i64::MAX } else { i64::MIN+1 },
		None => state.blacks.population() as i64 - state.whites.population() as i64
	}
}

