use crate::square::Square;

use std::fmt;
use std::ops;

/// A bitboard implementation using unsigned long long (u64).
/// One bit being set at a position indicates a piece placement there.
#[derive(Clone, Copy, Debug, Default, Eq, PartialOrd, Hash, PartialEq)]
pub struct Bitboard(pub u64);

pub const EMPTY: Bitboard = Bitboard(0u64);
pub const FULL: Bitboard = Bitboard(u64::MAX);

impl Bitboard {
    /// Create a new bitboard instance from a [`u64`].
    pub fn new(b: u64) -> Self {
        Bitboard(b)
    }

    /// Create a new bitboard instance from a [`Square`].
    pub fn from_square(s: Square) -> Self {
        Bitboard(1u64 << s.index())
    }
}

impl ops::BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, other: Self) -> Self::Output {
        Bitboard(self.0 & other.0)
    }
}

impl ops::BitAnd for &Bitboard {
    type Output = Bitboard;

    fn bitand(self, other: Self) -> Self::Output {
        Bitboard(self.0 & other.0)
    }
}

impl ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, other: Self) {
        self.0 &= other.0;
    }
}

impl ops::BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, other: Self) -> Self::Output {
        Bitboard(self.0 | other.0)
    }
}

impl ops::BitOr for &Bitboard {
    type Output = Bitboard;

    fn bitor(self, other: Self) -> Self::Output {
        Bitboard(self.0 | other.0)
    }
}

impl ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

impl ops::BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, other: Self) -> Self::Output {
        Bitboard(self.0 ^ other.0)
    }
}

impl ops::BitXor for &Bitboard {
    type Output = Bitboard;

    fn bitxor(self, other: Self) -> Self::Output {
        Bitboard(self.0 ^ other.0)
    }
}

impl ops::BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, other: Self) {
        self.0 ^= other.0;
    }
}

impl ops::Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl ops::Not for &Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in 0..8 {
            for file in 0..8 {
                let mask: u64 = 1u64 << (rank * 8 + file);
                write!(f, " {} ", if self.0 & mask != 0 { '1' } else { '0' })?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_and() {
        let b1: Bitboard = Bitboard(1u64 << 4);
        let b2: Bitboard = Bitboard(1u64 << 32);

        assert_eq!(EMPTY, b1 & b2);

        let mut b3: Bitboard = Bitboard((1u64 << 5) - 1);
        b3 &= b1;

        assert_eq!(b1, b3);
    }

    #[test]
    fn op_or() {
        let b1: Bitboard = Bitboard(1u64 << 4);
        let b2: Bitboard = Bitboard(1u64 << 32);
        let b3: Bitboard = Bitboard((1u64 << 4) + (1u64 << 32));

        assert_eq!(b3, b1 | b2);

        let mut b4: Bitboard = Bitboard((1u64 << 8) - 1);
        let b5: Bitboard = Bitboard((1u64 << 16) - 1);
        b4 |= b5;

        assert_eq!(b4, b5);
    }

    #[test]
    fn op_xor() {
        let b1: Bitboard = Bitboard((1u64 << 4) - 1);
        let b2: Bitboard = Bitboard(1u64 << 1);
        let b3: Bitboard = Bitboard(((1u64 << 4) - 1) - (1u64 << 1));

        assert_eq!(b3, b1 ^ b2);

        let mut b4: Bitboard = Bitboard((1u64 << 5) - 1);
        let b5: Bitboard = Bitboard((1u64 << 3) - 1);
        b4 ^= b5;
        let b6: Bitboard = Bitboard(((1u64 << 5) - 1) - ((1u64 << 3) - 1));

        assert_eq!(b6, b4);
    }

    #[test]
    fn op_not() {
        let b1: Bitboard = EMPTY;
        let b2: Bitboard = FULL;

        assert_eq!(b2, !b1);
        assert_eq!(b1, !b2);
    }

    #[test]
    fn display() {
        let bb: Bitboard = Bitboard::new(4);
        let s: String = bb.to_string();

        assert_eq!('1', s.chars().nth(7).unwrap());
    }
}
