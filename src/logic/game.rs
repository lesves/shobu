use super::board::{BitBoard, Board, Square};


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Color {
	Black,
	White,
}

impl Color {
	pub fn home(&self) -> [Board; 2] {
		match self {
			Color::Black => [Board::TopLeft, Board::TopRight],
			Color::White => [Board::BottomLeft, Board::BottomRight],
		}
	}

	pub fn home_mask(&self) -> BitBoard {
		match self {
			Color::Black => BitBoard::from_u64(0x00000000ffffffff),
			Color::White => BitBoard::from_u64(0xffffffff00000000),
		}
	}
}

impl std::ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
        	Self::Black => Self::White,
        	Self::White => Self::Black,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct GameState {
	pub current_side: Color,
	pub blacks: BitBoard,
	pub whites: BitBoard,
}

impl GameState {
	pub fn initial() -> GameState {
		GameState {
			current_side: Color::Black,
			blacks: BitBoard::from_u64(0x000f000f000f000f),
			whites: BitBoard::from_u64(0xf000f000f000f000),
		}
	}

	pub fn from_relative(current_side: Color, friendly: BitBoard, enemy: BitBoard) -> GameState {
		match current_side {
			Color::Black => GameState { current_side, blacks: friendly, whites: enemy },
			Color::White => GameState { current_side, whites: friendly, blacks: enemy },
		}
	}

	pub fn friendly(&self) -> BitBoard {
		match self.current_side {
			Color::Black => self.blacks,
			Color::White => self.whites,
		}
	}

	pub fn enemy(&self) -> BitBoard {
		match self.current_side {
			Color::Black => self.whites,
			Color::White => self.blacks,
		}
	}

	pub fn pieces(&self) -> BitBoard {
		self.blacks|self.whites
	}

	pub fn winner(&self) -> Option<Color> {
		let mut res = None;

		for board in Board::iter() {
			if (self.whites & board.mask()).empty() {
				assert!(res.is_none());
				res = Some(Color::Black);
			}
			if (self.blacks & board.mask()).empty() {
				assert!(res.is_none());
				res = Some(Color::White);
			}
		}
		res
	}
}

impl std::fmt::Debug for GameState {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		writeln!(f, "\n|==========================")?;
		writeln!(f, "| GameState: {} to move.", if self.current_side == Color::Black {"black"} else {"white"})?;
		for pair in [[Board::TopLeft, Board::TopRight], [Board::BottomLeft, Board::BottomRight]] {
			for row in 0..4 {
				write!(f, "| ")?;
				for board in pair {
					for col in 0..4 {
						let square = Square::from_local(board, row, col).unwrap();
						write!(f, "{}", if self.blacks.get(square) {
							"b"
						} else if self.whites.get(square) {
							"w"
						} else {
							"."
						})?;
					}
					write!(f, " ")?;
				}
				writeln!(f, "")?;
			}
			writeln!(f, "")?;
		}
		Ok(())
	}
}
