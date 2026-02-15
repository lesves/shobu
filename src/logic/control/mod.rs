pub mod agent;
pub mod evaluation;

pub use agent::{Agent, MinimaxAgent, RandomAgent};
pub use evaluation::{Score, Evaluation};
use super::game::Color;

pub enum Player {
	Human,
	Computer(Box<dyn Agent>),
}

pub struct Players {
	pub black: Player,
	pub white: Player,
}

impl Players {
	pub fn get(&mut self, color: Color) -> &mut Player {
		match color {
			Color::Black => &mut self.black,
			Color::White => &mut self.white,
		}
	}

	pub fn computer_only(&self) -> bool {
		match self.black {
			Player::Human => false,
			_ => match self.white {
				Player::Human => false,
				_ => true,
			}
		}
	}
}
