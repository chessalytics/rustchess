use crate::color::Color;
use crate::error::{ChessError, Result};

/// Exhaustive enum of the castling availability status for a color.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum CastlingStatus {
    NotAvailable = 0,
    Kingside = 1,
    Queenside = 2,
    Both = 3,
}

impl CastlingStatus {
    ///
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    pub fn from_u8(b: u8) -> Self {
        CastlingStatus::try_from_u8(b).unwrap()
    }

    ///
    pub fn try_from_u8(b: u8) -> Result<Self> {
        match b {
            0 => Ok(CastlingStatus::NotAvailable),
            1 => Ok(CastlingStatus::Queenside),
            2 => Ok(CastlingStatus::Kingside),
            3 => Ok(CastlingStatus::Both),
            _ => Err(Box::new(ChessError::UnknownCastlingRights(b.to_string()))),
        }
    }
}

///
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd)]
pub struct CastlingRights(pub u8);

pub const NO_CASTLING_RIGHTS: CastlingRights = CastlingRights(0u8);
pub const FULL_CASTLING_RIGHTS: CastlingRights = CastlingRights(15u8);

impl CastlingRights {
    ///
    pub fn from_str(s: &str) -> Self {
        CastlingRights::try_from(s).unwrap()
    }

    ///
    pub fn for_color(&self, c: Color) -> CastlingStatus {
        match c {
            Color::White => CastlingStatus::from_u8((self.0 & 12) >> 2),
            Color::Black => CastlingStatus::from_u8(self.0 & 3),
        }
    }
}

impl TryFrom<&str> for CastlingRights {
    type Error = ChessError;

    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
        let mut b: u8 = 0;
        for c in s.chars() {
            match c {
                'K' => b |= 1u8 << 3,
                'Q' => b |= 1u8 << 2,
                'k' => b |= 1u8 << 1,
                'q' => b |= 1u8 << 0,
                _ => return Err(ChessError::InvalidFen(s.to_string())),
            }
        };
        Ok(CastlingRights(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_ok() {
        let cr1: CastlingRights = CastlingRights::from_str("Qk");
        assert_eq!(CastlingRights(6), cr1);

        let cr2: CastlingRights = CastlingRights::from_str("q");
        assert_eq!(CastlingRights(1), cr2);

        let cr3: CastlingRights = CastlingRights::from_str("");
        assert_eq!(CastlingRights::default(), cr3);
    }

    #[test]
    #[should_panic]
    fn from_str_err() {
        CastlingRights::from_str("abc");
    }

    #[test]
    fn try_from_str_ok() {
        assert_eq!(CastlingRights(9), CastlingRights::try_from("Kq").unwrap());
    }

    #[test]
    #[should_panic]
    fn try_from_str_err() {
        CastlingRights::try_from("KQb").unwrap();
    }

    #[test]
    fn for_color() {
        let cr1 = CastlingRights::from_str("KQkq");
        let wr1 = cr1.for_color(Color::White);
        let br1 = cr1.for_color(Color::Black);

        assert_eq!(CastlingStatus::Both, wr1);
        assert_eq!(CastlingStatus::Both, br1);

        let cr2 = CastlingRights::from_str("k");
        let wr2 = cr2.for_color(Color::White);
        let br2 = cr2.for_color(Color::Black);

        assert_eq!(CastlingStatus::NotAvailable, wr2);
        assert_eq!(CastlingStatus::Kingside, br2);

        let cr3 = CastlingRights::from_str("Qq");
        let wr3 = cr3.for_color(Color::White);
        let br3 = cr3.for_color(Color::Black);

        assert_eq!(CastlingStatus::Queenside, wr3);
        assert_eq!(CastlingStatus::Queenside, br3);
    }
}
