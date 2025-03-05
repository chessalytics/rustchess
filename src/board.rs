use crate::CastlingStatus;
use crate::bitboard::{Bitboard, EMPTY};
use crate::castling_rights::{CastlingRights, NO_CASTLING_RIGHTS};
use crate::color::{Color, NUM_COLORS};
use crate::error::{ChessError, Result};
use crate::_move::Move;
use crate::piece::{NUM_PIECES, Piece};
use crate::square::Square;

use std::collections::HashMap;
use std::fmt;

/// The standard starting position in chess.
pub const DEFAULT_BOARD_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// Exhaustive enum of the states a chess board/game can be in.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum BoardStatus {
    Checkmate,
    Ongoing,
    Stalemate,
}

/// Implementation of a chess board.
#[derive(Debug)]
pub struct Board {
    bitboards: [Bitboard; NUM_PIECES * 2],
    pieces: HashMap<usize, (Piece, Color)>,
    side_to_move: Color,
    castling_rights: CastlingRights,
    en_passante_square: Option<Square>,
    halfmove_clock: usize,
    fullmove_number: usize,
    moves: Vec<Move>,
}

impl Board {
    /// Get the current bitboards representing the board position.
    pub fn bitboards(&self) -> &[Bitboard; NUM_PIECES * 2] {
        &self.bitboards
    }

    /// Get the current mapping of what pieces exist on what squares.
    pub fn pieces(&self) -> &HashMap<usize, (Piece, Color)> {
        &self.pieces
    }

    /// Get which players turn it is to make a move.
    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    /// Get the current castling rights for the position.
    pub fn castling_rights(&self) -> CastlingRights {
        self.castling_rights
    }

    /// Get the current castling rights for a specific color.
    pub fn castling_status_for(&self, c: Color) -> CastlingStatus {
        self.castling_rights.for_color(c)
    }

    /// Get the possible en passante square if there is one.
    pub fn en_passante_square(&self) -> Option<Square> {
        self.en_passante_square
    }

    /// Get the halfmove clock of the board position.
    pub fn halfmove_clock(&self) -> usize {
        self.halfmove_clock
    }

    /// Get the fullmove number of the board position.
    pub fn fullmove_number(&self) -> usize {
        self.fullmove_number
    }

    /// Get the moves made in this position from after initializing it.
    pub fn moves(&self) -> &Vec<Move> {
        &self.moves
    }

    /// Make a move on the chess board, this function is just an alias wrapper for [`Board::push`].
    pub fn make_move(&mut self, m: Move) {
        self.push(m);
    }

    /// Try and make a move on the chess board, this function is just an alias wrapper for [`Board::try_push`].
    pub fn try_make_move(&mut self, m: Move) -> Result<()> {
        self.try_push(m)
    }

    /// Make a move on the chess board.
    pub fn push(&mut self, m: Move) {
        self.try_push(m).unwrap()
    }

    /// Try and make a move on the chess board.
    pub fn try_push(&mut self, m: Move) -> Result<()> {
        let source: Square = m.source();
        let destination: Square = m.destination();

        let (moved_piece, moved_color): (Piece, Color) = match self.pieces.get(&source.index()) {
            Some((p, c)) => (*p, *c),
            None => {
                return Err(Box::new(ChessError::InvalidMove(m.to_string())));
            },
        };

        if moved_color != self.side_to_move() {
            return Err(Box::new(ChessError::InvalidMove(m.to_string())));
        }

        let piece_bb_idx: usize = moved_piece.as_index() + if moved_color == Color::Black { 6 } else { 0 };
        let piece_bb: Bitboard = self.bitboards[piece_bb_idx];

        println!("{piece_bb}");

        todo!()
    }

    /// Create a new [`Board`] that is completely empty.
    pub fn empty() -> Self {
        Board {
            bitboards: [EMPTY; NUM_PIECES * NUM_COLORS],
            pieces: HashMap::new(),
            side_to_move: Color::White,
            castling_rights: NO_CASTLING_RIGHTS,
            en_passante_square: None,
            halfmove_clock: 0,
            fullmove_number: 0,
            moves: Vec::new(),
        }
    }

    /// Alias for [`Board::empty()`] which creates a new empty board.
    ///
    /// If you want to create a board with the standard starting position,
    /// then use either of [`Board::default()`] or [`Board::standard()`].
    pub fn new() -> Self {
        Board::empty()
    }

    /// Create a new [`Board`] with the default starting position.
    pub fn standard() -> Self {
        Board::default()
    }

    /// Create a new [`Board`] from a user specified Forsyth-Edwards-Notation (FEN) string.
    ///
    /// # Panics
    /// Iff the user provided an invalid FEN string.
    pub fn from_fen(fen: &str) -> Self {
        BoardBuilder::from_fen(fen).build()
    }

    /// Try and create a new [`Board`] from a user specified Forsyth-Edwards-Notation (FEN) string.
    ///
    /// # Errors
    /// Iff the user provided an invalid FEN string.
    pub fn try_from_fen(fen: &str) -> Result<Self> {
        BoardBuilder::try_from_fen(fen)?.try_build()
    }
}

impl Default for Board {
    fn default() -> Self {
        BoardBuilder::from_fen(DEFAULT_BOARD_FEN).build()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "   +------------------------+")?;
        for rank in 0..8 {
            for field in 0..8 {
                if field == 0 {
                    write!(f, " {} |", 8 - rank)?;
                }

                if let Some((piece, color)) = self.pieces.get(&(rank * 8 + field)) {
                    write!(f, " {} ", piece.to_string(*color))?;
                } else {
                    write!(f, " . ")?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "   +------------------------+")?;
        writeln!(f, "     a  b  c  d  e  f  g  h")?;
        writeln!(
            f,
            "\n       To move: {}  ({}, {})",
            self.side_to_move, self.halfmove_clock, self.fullmove_number
        )
    }
}

/// A helper struct for building an instance of a [`Board`] struct.
#[derive(Debug, Default)]
pub struct BoardBuilder {
    bitboards: Option<[Bitboard; NUM_PIECES * NUM_COLORS]>,
    pieces: HashMap<usize, (Piece, Color)>,
    side_to_move: Option<Color>,
    castling_rights: Option<CastlingRights>,
    en_passante_square: Option<Square>,
    halfmove_clock: usize,
    fullmove_number: usize,
}

impl BoardBuilder {
    /// Create a new empty [`BoardBuilder`] instance.
    pub fn new() -> Self {
        BoardBuilder::default()
    }

    /// Create a new [`Board`]  from the current [`BoardBuilder`] instance.
    ///
    /// # Panics
    /// Iff not all required fields had been set.
    pub fn build(self) -> Board {
        self.try_build().unwrap()
    }

    /// Try and create a new [`Board`] from the current [`BoardBuilder`] instance.
    ///
    /// # Errors
    /// Iff not all required fields had been set.
    pub fn try_build(self) -> Result<Board> {
        let bitboards: [Bitboard; NUM_PIECES * NUM_COLORS] = self.bitboards.ok_or_else(|| {
            Box::new(ChessError::BoardSetup(
                "Bitboards not initialized".to_string(),
            ))
        })?;

        let side_to_move: Color = self.side_to_move.ok_or_else(|| {
            Box::new(ChessError::BoardSetup(
                "Side to move not initialized".to_string(),
            ))
        })?;

        let castling_rights: CastlingRights = self
            .castling_rights
            .ok_or_else(|| Box::new(ChessError::BoardSetup("".to_string())))?;

        Ok(Board {
            bitboards,
            pieces: self.pieces,
            side_to_move,
            castling_rights,
            en_passante_square: self.en_passante_square,
            halfmove_clock: self.halfmove_clock,
            fullmove_number: self.fullmove_number,
            moves: Vec::new(),
        })
    }

    /// Set up a board state from a provided FEN string.
    ///
    /// # Panics
    /// If the provided FEN string was invalid.
    ///
    /// # Details
    /// Forsyth–Edwards Notation (FEN) is a standard notation for describing a
    /// particular board position of a chess game. The purpose of FEN is to
    /// provide all the necessary information to restart a game from a particular
    /// position [wikipedia link](https://en.wikipedia.org/wiki/Forsyth–Edwards_Notation).
    ///
    /// Below you can see the FEN for the starting position:
    /// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    pub fn from_fen(fen: &str) -> BoardBuilder {
        BoardBuilder::try_from_fen(fen).unwrap()
    }

    /// Try to set up a board state from a provided FEN string.
    ///
    /// # Errors
    /// If the provided FEN string was invalid.
    ///
    /// # Details
    /// Forsyth–Edwards Notation (FEN) is a standard notation for describing a
    /// particular board position of a chess game. The purpose of FEN is to
    /// provide all the necessary information to restart a game from a particular
    /// position [wikipedia link](https://en.wikipedia.org/wiki/Forsyth–Edwards_Notation).
    ///
    /// Below you can see the FEN for the starting position:
    /// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    pub fn try_from_fen(fen: &str) -> Result<BoardBuilder> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 4 {
            return Err(Box::new(ChessError::InvalidFen(fen.to_string())));
        }

        // Initialize board state as empty.
        let mut bitboards: [Bitboard; NUM_PIECES * NUM_COLORS] = [EMPTY; NUM_PIECES * NUM_COLORS];
        let mut pieces: HashMap<usize, (Piece, Color)> = HashMap::new();

        // In reality we are actually going from the 8th rank to the 1st rank,
        // but we calculate this counter backwards because its easier with indices...
        let mut rank: usize = 0;
        let mut file: usize = 0;

        let piece_placement_str: &str = parts[0];
        let active_color_str: &str = parts[1];
        let castling_rights_str: &str = parts[2];
        let en_passant_square_str: &str = parts[3];

        for c in piece_placement_str.chars() {
            if c == '/' {
                rank += 1;
                file = 0;
                continue;
            }

            let s: Square = Square::from_index(rank * 8 + file);
            let bb_idx: usize;

            let piece: Piece;
            let color: Color;

            match c {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    // We need to subtract 48 here because the char '1' byte value is 49.
                    // See the ASCII table for more details: https://www.ascii-code.com
                    file += (c as usize) - 48;
                    continue;
                }
                'P' => {
                    bb_idx = 0;
                    piece = Piece::Pawn;
                    color = Color::White;
                }
                'N' => {
                    bb_idx = 1;
                    piece = Piece::Knight;
                    color = Color::White;
                }
                'B' => {
                    bb_idx = 2;
                    piece = Piece::Bishop;
                    color = Color::White;
                }
                'R' => {
                    bb_idx = 3;
                    piece = Piece::Rook;
                    color = Color::White;
                }
                'Q' => {
                    bb_idx = 4;
                    piece = Piece::Queen;
                    color = Color::White;
                }
                'K' => {
                    bb_idx = 5;
                    piece = Piece::King;
                    color = Color::White;
                }
                'p' => {
                    bb_idx = 6;
                    piece = Piece::Pawn;
                    color = Color::Black;
                }
                'n' => {
                    bb_idx = 7;
                    piece = Piece::Knight;
                    color = Color::Black;
                }
                'b' => {
                    bb_idx = 8;
                    piece = Piece::Bishop;
                    color = Color::Black;
                }
                'r' => {
                    bb_idx = 9;
                    piece = Piece::Rook;
                    color = Color::Black;
                }
                'q' => {
                    bb_idx = 10;
                    piece = Piece::Queen;
                    color = Color::Black;
                }
                'k' => {
                    bb_idx = 11;
                    piece = Piece::King;
                    color = Color::Black;
                }
                _ => {
                    return Err(Box::new(ChessError::InvalidFen(fen.to_string())));
                }
            }

            bitboards[bb_idx] |= Bitboard::from_square(s);
            pieces.insert(rank * 8 + file, (piece, color));
            file += 1;
        }

        let side_to_move: Color = Color::try_from_str(active_color_str)?;

        let castling_rights: CastlingRights = CastlingRights::try_from(castling_rights_str)?;

        let en_passante_square: Option<Square> = match en_passant_square_str {
            "-" => None,
            _ => Some(Square::try_from(en_passant_square_str)?),
        };

        let mut halfmove_clock: usize = 0;
        let mut fullmove_number: usize = 0;

        if parts.len() == 6 {
            halfmove_clock = parts[4].parse()?;
            fullmove_number = parts[5].parse()?;
        }

        Ok(BoardBuilder {
            bitboards: Some(bitboards),
            pieces,
            side_to_move: Some(side_to_move),
            castling_rights: Some(castling_rights),
            en_passante_square,
            halfmove_clock,
            fullmove_number,
        })
    }
}
