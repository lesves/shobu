use crate::enumindex::AsIndex;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Board {
	TopLeft,
	TopRight,
	BottomLeft,
	BottomRight,
}

impl Board {
	pub fn coord(&self) -> (u8, u8) {
		use Board::*;
		match self {
			TopLeft => (0, 0),
			TopRight => (0, 1),
			BottomLeft => (1, 0),
			BottomRight => (1, 1),
		}
	}

	pub fn iter() -> impl Iterator<Item=Self> {
		use Board::*;
		[TopLeft, TopRight, BottomLeft, BottomRight].iter().copied()
	}
}

impl AsIndex for Board {
	#[inline]
	fn to_idx(&self) -> usize {
		*self as usize
	}
}

impl From<u8> for Board {
	fn from(item: u8) -> Board {
		match item {
			0 => Board::TopLeft,
			1 => Board::TopRight,
			2 => Board::BottomLeft,
			3 => Board::BottomRight,
			_ => panic!("unable to select board with id {}", item) // TODO: rewrite
		}
	}
}


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Square(pub(super) u8);

impl Square {
	pub fn from_global(pos: u8) -> Option<Square> {
		if pos < 64 {
			Some(Square(pos))
		} else {
			None
		}
	}

	pub fn from_local(board: Board, row: u8, col: u8) -> Option<Square> {
		if row < 4 && col < 4 {
			Some(Square(board as u8*16 + row*4 + col))
		} else {
			None
		}
	}

	pub fn global(&self) -> u8 {
		self.0
	}

	pub fn board(&self) -> Board {
		(self.0/16).into()
	}

	pub fn row(&self) -> u8 {
		(self.0%16) / 4
	}

	pub fn col(&self) -> u8 {
		self.0 % 4
	}

	pub fn local(&self) -> (Board, u8, u8) {
		(self.board(), self.row(), self.col())
	}
}

impl AsIndex for Square {
	#[inline]
	fn to_idx(&self) -> usize {
		self.global() as usize
	}
}
