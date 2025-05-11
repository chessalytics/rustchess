use crate::error::{ChessError, Result};
use crate::piece::Piece;
use crate::square::Square;

use std::fmt;

/// Representation of a move.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct Move {
    source: Square,
    destination: Square,
    promotion: Option<Piece>,
}

impl Move {
    /// Create a new [`Move`] given a source square, destination square, and optional piece to
    /// promote to.
    pub fn new(source: Square, destination: Square, promotion: Option<Piece>) -> Self {
        Move {
            source,
            destination,
            promotion,
        }
    }

    /// Get the source square for the move.
    pub fn source(&self) -> Square {
        self.source
    }

    /// Get the target square for the move.
    pub fn destination(&self) -> Square {
        self.destination
    }

    /// Get the piece the move promoted to if it was a promotion.
    pub fn promotion(&self) -> Option<Piece> {
        self.promotion
    }

    /// Get whether or not the move is a promotion.
    pub fn is_promotion(&self) -> bool {
        self.promotion.is_some()
    }

    /// Create a new move from a UCI string.
    pub fn from_uci(uci: &str) -> Self {
        Move::try_from_uci(uci).unwrap()
    }

    /// Try and create a new move from a UCI string.
    pub fn try_from_uci(uci: &str) -> Result<Self> {
        let source_str: &str = uci
            .get(0..2)
            .ok_or_else(|| Box::new(ChessError::UnknownUciMove(uci.to_string())))?;
        let destination_str: &str = uci
            .get(2..4)
            .ok_or_else(|| Box::new(ChessError::UnknownUciMove(uci.to_string())))?;

        let mut promotion_piece: Option<Piece> = None;
        if uci.len() == 5 {
            promotion_piece =
                Some(Piece::try_from(uci.chars().last().ok_or_else(|| {
                    Box::new(ChessError::UnknownPiece(uci.to_string()))
                })?)?);
        }

        Ok(Move {
            source: Square::try_from(source_str)?,
            destination: Square::try_from(destination_str)?,
            promotion: promotion_piece,
        })
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.source(), self.destination(),)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_move_and_getters() {
        let m1 = Move {
            source: Square::from_str("a4"),
            destination: Square::from_str("a6"),
            promotion: None,
        };
        let m2 = Move::new(
            Square::from_str("a8"),
            Square::from_index(8),
            Some(Piece::Queen),
        );
        let m3 = Move::new(Square(0), Square::from_str("a7"), Some(Piece::Queen));

        assert_eq!(Square::from_str("a4"), m1.source());
        assert_eq!(Square::from_index(8), m2.destination());
        assert_eq!(None, m1.promotion);
        assert_eq!(Some(Piece::Queen), m2.promotion);
        assert_eq!(true, m3.is_promotion());
        assert_eq!(m2, m3);
    }

    #[test]
    fn from_uci_ok() {
        let m1 = Move::from_uci("a6b8");
        let m2 = Move::from_uci("b5f2");
        let m3 = Move::from_uci("a7b8n");

        assert_eq!(Square::from_str("a6"), m1.source());
        assert_eq!(Square::from_str("f2"), m2.destination());
        assert_eq!(None, m2.promotion());
        assert_eq!(Some(Piece::Knight), m3.promotion());
        assert_eq!(false, m1.is_promotion());
    }

    #[test]
    #[should_panic]
    fn from_uci_err() {
        Move::try_from_uci("q7b6").unwrap();
    }
}
