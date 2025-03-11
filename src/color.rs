use crate::error::{ChessError, Result};

use std::fmt;
use std::ops;

/// Exhaustive enum of the available colors in chess.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Color {
    White,
    Black,
}

/// The number of different colors available.
pub const NUM_COLORS: usize = 2;
/// An [`array`] containing the colors with placement corresponding to their respective index.
pub const COLORS: [Color; NUM_COLORS] = [Color::White, Color::Black];

impl Color {
    /// Get the [`usize`] index of the color.
    ///
    /// This is usually used for efficient table lookups.
    pub fn as_index(&self) -> usize {
        *self as usize
    }

    /// Create a new [`Color`] from a given string.
    ///
    /// # Panics
    /// If either the string is empty or the string does not contain any of the characters
    /// ('w', 'W', 'b', 'B') as the first character in the string.
    pub fn from_str(s: &str) -> Color {
        Color::try_from_str(s).unwrap()
    }

    /// Create a new [`Color`] from a given string.
    ///
    /// # Errors
    /// Returns a [`ChessError::UnknownColor`] error if either the string is empty or the string does not contain any of the characters
    /// ('w', 'W', 'b', 'B') as the first character in the string.
    pub fn try_from_str(s: &str) -> Result<Color> {
        match s.chars().next() {
            Some(c) => match c {
                'w' | 'W' => Ok(Color::White),
                'b' | 'B' => Ok(Color::Black),
                _ => Err(Box::new(ChessError::UnknownColor(s.to_string()))),
            },
            None => Err(Box::new(ChessError::UnknownColor(s.to_string()))),
        }
    }

    /// Get the opposite color.
    pub fn invert(self) -> Self {
        !self
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "W"),
            Color::Black => write!(f, "B"),
        }
    }
}

impl ops::Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_index() {
        assert_eq!(0, Color::White.as_index());
        assert_eq!(1, Color::Black.as_index());
    }

    #[test]
    fn index_lookup() {
        assert_eq!(Color::White, COLORS[0]);
        assert_eq!(Color::Black, COLORS[Color::Black.as_index()]);
    }

    #[test]
    fn try_from_str_ok() {
        assert_eq!(Color::White, Color::try_from_str("W").unwrap());
        assert_eq!(Color::White, Color::try_from_str("w").unwrap());
        assert_eq!(Color::Black, Color::try_from_str("B").unwrap());
        assert_eq!(Color::Black, Color::try_from_str("b").unwrap());
    }

    #[test]
    #[should_panic]
    fn try_from_str_err_empty() {
        Color::try_from_str("").unwrap();
    }

    #[test]
    #[should_panic]
    fn try_from_str_err() {
        Color::try_from_str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbbcccc").unwrap();
    }

    #[test]
    fn from_str() {
        assert_eq!(Color::White, Color::from_str("w"));
        assert_eq!(Color::Black, Color::from_str("B"));
    }

    #[test]
    #[should_panic]
    fn from_str_err() {
        Color::from_str("q");
    }

    #[test]
    fn invert() {
        assert_eq!(Color::Black, !Color::White);
        assert_eq!(Color::White, !Color::Black);
        assert_eq!(Color::White, Color::Black.invert());
    }
}
