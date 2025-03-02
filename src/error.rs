use thiserror::Error;

use std::error;
use std::result;

///
pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ChessifyError {
    #[error("board was not setup properly due to: `{0}`")]
    BoardSetup(String),

    #[error("`{0}` is an invalid FEN string")]
    InvalidFen(String),

    #[error("could not parse `{0}` as `{0}`")]
    ParsingError(String, String),

    #[error("could not parse `{0}` as castling rights")]
    UnknownCastlingRights(String),

    #[error("could not parse `{0}` as a color")]
    UnknownColor(String),

    #[error("could not parse `{0}` as a square")]
    UnknownSquare(String),

    #[error("unknown chessify error")]
    Unknown,
}
