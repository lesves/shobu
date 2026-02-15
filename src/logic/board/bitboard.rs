use super::square::{Square, Board};


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BitBoard(pub(super) u64);

impl BitBoard {
    #[inline]
    pub const fn new() -> Self {
        BitBoard(0)
    }

    #[inline]
    pub const fn from_u64(bitset: u64) -> Self {
        BitBoard(bitset)
    }

    #[inline]
    pub fn get(&self, pos: Square) -> bool {
        self.0 & 1 << pos.0 != 0
    }

    #[inline]
    pub fn set(&mut self, pos: Square) {
        self.0 |= 1 << pos.0;
    }

    #[inline]
    pub fn clear(&mut self, pos: Square) {
        self.0 &= !(1 << pos.0);
    }

    #[inline]
    pub fn flip(&mut self, pos: Square) {
        self.0 ^= 1 << pos.0;
    }

    #[inline]
    pub fn empty(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn any(&self) -> bool {
        self.0 != 0
    }

    #[inline]
    pub fn population(&self) -> u32 {
        self.0.count_ones()
    }

    #[inline]
    pub fn only_one(&self) -> bool {
        self.lscan() == self.rscan() && self.any()
    }

    #[inline]
    pub fn lscan(&self) -> Option<Square> {
        if self.0.trailing_zeros() == 64 {
            None
        } else {
            Some(Square(self.0.trailing_zeros() as u8))
        }
    }

    #[inline]
    pub fn rscan(&self) -> Option<Square> {
        if self.0.leading_zeros() == 64 {
            None
        } else {
            Some(Square(63-self.0.leading_zeros() as u8))
        }
    }

    pub fn iter(&self) -> BitBoardIter {
        BitBoardIter(self.0)
    }
}

pub struct BitBoardIter(u64);

impl Iterator for BitBoardIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let b = self.0 & (!self.0+1);
            self.0 ^= b;
            Some(Square(63 - b.leading_zeros() as u8))
        }
    }
}

impl std::ops::Index<Square> for BitBoard {
    type Output = bool;

    #[inline]
    fn index(&self, pos: Square) -> &Self::Output {
        match self.get(pos) {
            true => &true,
            false => &false,
        }
    }
}

impl std::ops::BitAnd for BitBoard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for BitBoard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAndAssign for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0)
    }
}

impl std::ops::Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl std::fmt::Debug for BitBoard {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for pair in [[Board::TopLeft, Board::TopRight], [Board::BottomLeft, Board::BottomRight]] {
			for row in 0..4 {
				for board in pair {
					for col in 0..4 {
						write!(f, "{}", if self.get(Square::from_local(board, row, col).unwrap()) {
							"x"
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

impl Board {
	pub fn mask(self) -> BitBoard {
		BitBoard(0xFFFF << (self as u8*16))
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bitboard() {
        use super::{BitBoard, Square};

        let mut bb = BitBoard::new();

        bb.set(Square(0));
        bb.set(Square(2));
        bb.flip(Square(1));
        bb.flip(Square(2));
        bb.clear(Square(63));

        assert!(bb[Square(0)]);
        assert!(!bb[Square(2)]);
        assert!(bb[Square(1)]);
        assert!(!bb[Square(63)]);

        bb.set(Square(63));
        assert!(bb[Square(63)]);
        bb.clear(Square(63));
        assert!(!bb[Square(63)]);
    }

    #[test]
    fn test_bitscan() {
        use super::{BitBoard, Square};

        let mut bb = BitBoard::new();
        bb.set(Square(32));
        for x in 20..0 {
            bb.set(Square(x));
            assert_eq!(bb.lscan(), Some(Square(x)));
        }
        bb = BitBoard::new();

        bb.set(Square(11));
        for x in 40..64 {
            bb.set(Square(x));
            assert_eq!(bb.rscan(), Some(Square(x)));
        }

        assert_eq!(BitBoard::new().rscan(), None);
        assert_eq!(BitBoard::new().lscan(), None);
    }

    #[test]
    fn test_bitboarditer() {
        use super::{BitBoard, Square};

        let mut it = BitBoard::from_u64(9223372036921884690).iter();

        assert_eq!(it.next(), Some(Square(1)));
        assert_eq!(it.next(), Some(Square(4)));
        assert_eq!(it.next(), Some(Square(26)));
        assert_eq!(it.next(), Some(Square(63)));
        assert_eq!(it.next(), None);
    }
}
