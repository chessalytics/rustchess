use crate::error::ChessError;

use std::fmt;

/// Implementation of a file on the chess board (vertically from 0 to 7).
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd)]
pub struct File(pub u8);

impl TryFrom<char> for File {
    type Error = ChessError;

    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        let f: u8 = match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => {
                return Err(ChessError::UnknownSquare(c.to_string()));
            }
        };
        Ok(File(f))
    }
}

impl TryFrom<&str> for File {
    type Error = ChessError;

    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
        let f: u8 = match s.to_lowercase().as_str() {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            "h" => 7,
            _ => {
                return Err(ChessError::UnknownSquare(s.to_string()));
            }
        };
        Ok(File(f))
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &str = match self.0 {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => {
                return Err(fmt::Error);
            }
        };
        write!(f, "{}", s)
    }
}

/// Implementation of a rank on the chess board (horizontally from 0 to 7).
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd)]
pub struct Rank(pub u8);

impl TryFrom<char> for Rank {
    type Error = ChessError;

    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        let r: u8 = match c {
            '1' => 7,
            '2' => 6,
            '3' => 5,
            '4' => 4,
            '5' => 3,
            '6' => 2,
            '7' => 1,
            '8' => 0,
            _ => {
                return Err(ChessError::UnknownSquare(c.to_string()));
            }
        };
        Ok(Rank(r))
    }
}

impl TryFrom<&str> for Rank {
    type Error = ChessError;

    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
        let r: u8 = match s {
            "1" => 7,
            "2" => 6,
            "3" => 5,
            "4" => 4,
            "5" => 3,
            "6" => 2,
            "7" => 1,
            "8" => 0,
            _ => {
                return Err(ChessError::UnknownSquare(s.to_string()));
            }
        };
        Ok(Rank(r))
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1 + self.0)
    }
}

/// Chess square implementation using an unsigned char ([`u8`]).
///
/// You can either create a [`Square`] by providing its representative board index
/// (0-63) or by supplying a string which follows the standard chess notation.
///
/// The squares on the chess board are organized accordingly:
///
///  a8 b8 c8 d8 e8 f8 g8 h8
///  a7 b7 c7 d7 e7 f7 g7 h7
///  a6 b6 c6 d6 e6 f6 g6 h6
///  a5 b5 c5 d5 e5 f5 g5 h5
///  a4 b4 c4 d4 e4 f4 g4 h4
///  a3 b3 c3 d3 e4 f4 g4 h4
///  a2 b2 c2 d2 e2 f2 g2 h2
///  a1 b1 c1 d1 r1 f1 g1 h1
///
///  where a8 has index 0 and h1 has index 63, i.e., indices go from top to bottom + left to right.
///
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Square(pub u8);

impl Square {
    /// Create a new [`Square`] instance from an unsigned char ([`u8`]).
    ///
    /// # Note
    /// Any value > 63 will be truncated to 63 and thus represent the h1 square.
    pub fn new(b: u8) -> Self {
        Square(b & 63)
    }

    /// Create a new [`Square`] instance from a [`usize`].
    ///
    /// # Note
    /// This simply casts the usize to a u8 and truncates any values > 63.
    /// Intended to be used for more efficient table lookups.
    pub fn from_index(i: usize) -> Self {
        Square((i as u8) & 63)
    }

    /// Get the file of the square as an unsigned char ([`u8`]).
    pub fn file_as_u8(&self) -> u8 {
        self.0 % 8
    }

    /// Get the rank of the square as an unsigned char ([`u8`]).
    pub fn rank_as_u8(&self) -> u8 {
        7 - (self.0 / 8)
    }

    /// Get the file of the square as a [`File`].
    pub fn file(&self) -> File {
        File(self.0 % 8)
    }

    /// Get the rank of the square as a [`Rank`].
    pub fn rank(&self) -> Rank {
        Rank(7 - (self.0 / 8))
    }

    /// Get the squares index value as a [`usize`].
    pub fn index(&self) -> usize {
        self.0 as usize
    }

    /// Create a new [`Square`] instance from a string.
    ///
    /// # Panics
    /// Iff the string was not a valid chess square.
    pub fn from_str(s: &str) -> Self {
        Square::try_from(s).unwrap()
    }
}

impl TryFrom<&str> for Square {
    type Error = ChessError;

    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
        if s.len() < 2 {
            return Err(ChessError::UnknownSquare(s.to_string()));
        }

        let file: File = File::try_from(s.to_lowercase().chars().nth(0).unwrap())?;
        let rank: Rank = Rank::try_from(s.chars().nth(1).unwrap())?;
        Ok(Square(rank.0 * 8 + file.0))
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_ok() {
        let h1 = Square::from_str("h1");
        let c8 = Square::from_str("c8");
        let e4 = Square::from_str("E4");
        let a3 = Square::from_str("a3");
        let b2 = Square::from_str("b2");
        let g5 = Square::from_str("g5");
        let d6 = Square::from_str("d6");

        assert_eq!(Square(63), h1);
        assert_eq!(Square::new(2), c8);
        assert_eq!(Square::from_index(36_usize), e4);
        assert_eq!(63_usize, h1.index());
        assert_eq!(2, a3.rank().0);
        assert_eq!(1, b2.file().0);
        assert_eq!(4, g5.rank().0);
        assert_eq!(3, d6.file().0);

        assert_eq!("h1".to_string(), h1.to_string());
        assert_eq!("c8".to_string(), c8.to_string());
        assert_eq!("e4".to_string(), e4.to_string());
        assert_eq!("a3".to_string(), a3.to_string());
        assert_eq!("b2".to_string(), b2.to_string());
        assert_eq!("g5".to_string(), g5.to_string());
        assert_eq!("d6".to_string(), d6.to_string());
    }

    #[test]
    #[should_panic]
    fn file_to_string_err() {
        File(10).to_string();
    }

    #[test]
    #[should_panic]
    fn from_str_err_too_short() {
        Square::from_str("a");
    }

    #[test]
    #[should_panic]
    fn from_str_err_unknown_rank() {
        Square::from_str("a9");
    }

    #[test]
    fn file_from_str_ok() {
        assert_eq!(0, File::try_from("a").unwrap().0);
        assert_eq!(1, File::try_from("b").unwrap().0);
        assert_eq!(2, File::try_from("c").unwrap().0);
        assert_eq!(3, File::try_from("d").unwrap().0);
        assert_eq!(4, File::try_from("e").unwrap().0);
        assert_eq!(5, File::try_from("f").unwrap().0);
        assert_eq!(5, File::try_from("F").unwrap().0);
        assert_eq!(6, File::try_from("g").unwrap().0);
        assert_eq!(7, File::try_from("h").unwrap().0);
        assert_eq!(7, File::try_from("H").unwrap().0);
    }

    #[test]
    #[should_panic]
    fn file_from_str_err() {
        File::try_from("q").unwrap();
    }

    #[test]
    fn rank_from_str_ok() {
        assert_eq!(7, Rank::try_from("1").unwrap().0);
        assert_eq!(6, Rank::try_from("2").unwrap().0);
        assert_eq!(5, Rank::try_from("3").unwrap().0);
        assert_eq!(4, Rank::try_from("4").unwrap().0);
        assert_eq!(3, Rank::try_from("5").unwrap().0);
        assert_eq!(2, Rank::try_from("6").unwrap().0);
        assert_eq!(1, Rank::try_from("7").unwrap().0);
        assert_eq!(0, Rank::try_from("8").unwrap().0);
    }

    #[test]
    #[should_panic]
    fn rank_from_str_err() {
        Rank::try_from("9").unwrap();
    }

    #[test]
    fn file_and_rank_ok() {
        let d7 = Square::from_str("d7");
        let f1 = Square::from_str("f1");

        assert_eq!(6u8, d7.rank_as_u8());
        assert_eq!(Rank(6), d7.rank());

        assert_eq!(5u8, f1.file_as_u8());
        assert_eq!(File(5), f1.file());

        let ds = d7.to_string();
        let fs = f1.to_string();
        assert_eq!('d', ds.chars().next().unwrap());
        assert_eq!('1', fs.chars().nth(1).unwrap());
    }

    #[test]
    #[should_panic]
    fn from_str_err_unknown_file() {
        Square::from_str("q4");
    }
}
