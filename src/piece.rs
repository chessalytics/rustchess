use crate::color::Color;

/// Exhaustive enum of all available piece types.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// The number of different pieces in the game of chess.
pub const NUM_PIECES: usize = 6;

/// An [`array`] containing the pieces with placement corresponding to their respective index.
pub const PIECES: [Piece; NUM_PIECES] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];

impl Piece {
    /// Get the [`usize`] index of the piece.
    ///
    /// This is usually used for efficient table lookups.
    pub fn as_index(&self) -> usize {
        *self as usize
    }

    pub fn to_string(&self, color: Color) -> String {
        let s: &str = match self {
            Piece::Pawn => "P",
            Piece::Knight => "N",
            Piece::Bishop => "B",
            Piece::Rook => "R",
            Piece::Queen => "Q",
            Piece::King => "K",
        };

        match color {
            Color::White => s.to_string(),
            Color::Black => s.to_lowercase(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_index() {
        assert_eq!(0, Piece::Pawn.as_index());
        assert_eq!(1, Piece::Knight.as_index());
        assert_eq!(2, Piece::Bishop.as_index());
        assert_eq!(3, Piece::Rook.as_index());
        assert_eq!(4, Piece::Queen.as_index());
        assert_eq!(5, Piece::King.as_index());
    }

    #[test]
    fn index_lookup() {
        assert_eq!(Piece::Pawn, PIECES[0]);
        assert_eq!(Piece::Rook, PIECES[Piece::Rook.as_index()]);
    }

    #[test]
    fn to_string() {
        assert_eq!("P".to_string(), Piece::Pawn.to_string(Color::White));
        assert_eq!("N".to_string(), Piece::Knight.to_string(Color::White));
        assert_eq!("B".to_string(), Piece::Bishop.to_string(Color::White));
        assert_eq!("R".to_string(), Piece::Rook.to_string(Color::White));
        assert_eq!("Q".to_string(), Piece::Queen.to_string(Color::White));
        assert_eq!("K".to_string(), Piece::King.to_string(Color::White));

        assert_eq!("p".to_string(), Piece::Pawn.to_string(Color::Black));
        assert_eq!("n".to_string(), Piece::Knight.to_string(Color::Black));
        assert_eq!("b".to_string(), Piece::Bishop.to_string(Color::Black));
        assert_eq!("r".to_string(), Piece::Rook.to_string(Color::Black));
        assert_eq!("q".to_string(), Piece::Queen.to_string(Color::Black));
        assert_eq!("k".to_string(), Piece::King.to_string(Color::Black));
    }
}
