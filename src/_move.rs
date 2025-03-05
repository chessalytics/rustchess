use crate::piece::Piece;
use crate::square::Square;

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
    fn new(source: Square, destination: Square, promotion: Option<Piece>) -> Self {
        Move {
            source,
            destination,
            promotion,
        }
    }

    /// Get the source square for the move.
    fn source(&self) -> Square {
        self.source
    }

    /// Get the target square for the move.
    fn destination(&self) -> Square {
        self.destination
    }

    /// Get the piece the move promoted to if it was a promotion.
    fn promotion(&self) -> Option<Piece> {
        self.promotion
    }

    /// Get whether or not the move is a promotion.
    fn is_promotion(&self) -> bool {
        self.promotion.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_move_and_getters() {
        let m1 = Move { source: Square::from_str("a4"), destination: Square::from_str("a6"), promotion: None };
        let m2 = Move::new(Square::from_str("a8"), Square::from_index(8), Some(Piece::Queen));
        let m3 = Move::new(Square(0), Square::from_str("a7"), Some(Piece::Queen));

        assert_eq!(Square::from_str("a4"), m1.source());
        assert_eq!(Square::from_index(8), m2.destination());
        assert_eq!(None, m1.promotion);
        assert_eq!(Some(Piece::Queen), m2.promotion);
        assert_eq!(true, m3.is_promotion());
        assert_eq!(m2, m3);
    }
}
