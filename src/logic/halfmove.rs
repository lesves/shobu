use super::board::{BitBoard, Square};


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct HalfMove(Square, MoveVector);

impl HalfMove {
	pub fn new(from: Square, ve: MoveVector) -> Option<HalfMove> {
		ve.apply(from)?;
		Some(HalfMove(from, ve))
	}

	pub fn vector(&self) -> MoveVector {
		self.1
	}

	pub fn from(&self) -> Square {
		self.0
	}

	pub fn to(&self) -> Square {
		self.1.apply(self.0).unwrap()
	}

	pub fn is_valid_passive(&self, blockers: BitBoard) -> bool {
		match self.vector().size {
			MoveSize::One => !blockers.get(self.to()),
			MoveSize::Two => {
				let closer = self.vector().apply_with_size(self.from(), 1).unwrap();
				!blockers.get(closer) && !blockers.get(self.to())
			}
		}
	}

	pub fn is_valid_active(&self, friendly: BitBoard, enemy: BitBoard) -> bool {
		let both = friendly | enemy;
		let stone_at = |bb: BitBoard, dist: i8| self.vector().apply_with_size(self.from(), dist).map(|s| bb.get(s)).unwrap_or(false);

		if self.vector().size == MoveSize::One && stone_at(enemy, 1) && !stone_at(both, 2) {
			true
		} else if self.vector().size == MoveSize::Two && stone_at(enemy, 1) && !stone_at(both, 2) && !stone_at(both, 3) {
			true
		} else if !stone_at(both, 1) && stone_at(enemy, 2) && !stone_at(both, 3) {
			true
		} else {
			self.is_valid_passive(both)
		}
	}
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct MoveVector {
	pub size: MoveSize,
	pub dir: Direction,
}

impl MoveVector {
	pub fn new(size: MoveSize, dir: Direction) -> MoveVector {
		MoveVector { size, dir }
	}

	pub fn apply(&self, square: Square) -> Option<Square> {
		self.apply_with_size(square, self.size.value())
	}

	pub fn apply_with_size(&self, square: Square, size: i8) -> Option<Square> {
		let offset = self.dir.offset();

		let (board, row, col) = square.local();
		Square::from_local(board, (row as i8 + size*offset.0) as u8, (col as i8 + size*offset.1) as u8)
	}

	pub fn iter() -> impl Iterator<Item=Self> {
		MoveSize::iter().flat_map(|size| {
			Direction::iter().map(move |dir| MoveVector { size, dir })
		})
	}
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum MoveSize {
	One,
	Two,
}

impl MoveSize {
	pub fn value(&self) -> i8 {
		match self {
			Self::One => 1,
			Self::Two => 2,
		}
	}

	pub fn iter() -> impl Iterator<Item=Self> {
		[Self::One, Self::Two].iter().copied()
	}
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Direction {
	N,
	NW,
	W,
	SW,
	S,
	SE,
	E,
	NE,
}

impl Direction {
	pub fn iter() -> impl Iterator<Item=Self> {
		use Direction::*;
		[N, NW, W, SW, S, SE, E, NE].iter().copied()
	}

	pub fn offset(self) -> (i8, i8) {
		[
			(-1,  0), // N
			(-1, -1), // NW
			( 0, -1), //  W
			( 1, -1), // SW
			( 1,  0), // S
			( 1,  1), // SE
			( 0,  1), //  E
			(-1,  1), // NE
		][self as usize]
	}
}
