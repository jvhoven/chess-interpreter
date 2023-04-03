use crate::{piece::Piece, square::Square};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Color {
    White,
    Black,
}

struct Squares {
    data: [Option<(Piece, Color)>; 64],
}

impl Hash for Squares {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (idx, item) in self.data.iter().enumerate() {
            state.write_u8(idx as u8);

            if let Some((piece, color)) = item {
                piece.hash(state);
                color.hash(state);
            }
        }
    }
}

pub enum Perspective {
    White,
    Black,
}

pub struct Board {
    pub squares: [Option<(Piece, Color)>; 64],
    pub hash: u64,
    pub perspective: Perspective,
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [None; 64],
            hash: 0,
            perspective: Perspective::White,
        }
    }

    pub fn with_pieces<'a>(pieces: impl IntoIterator<Item = &'a (Square, Piece, Color)>) -> Board {
        let mut board = Board::new();

        for piece in pieces.into_iter() {
            board.squares[piece.0.to_index()] = Some((piece.1, piece.2));
        }

        board.hash = Board::calculate_hash(board.squares);
        board
    }

    pub fn calculate_hash<'a>(squares: [Option<(Piece, Color)>; 64]) -> u64 {
        let mut hasher = DefaultHasher::new();
        squares.hash(&mut hasher);

        hasher.finish()
    }
}

impl Default for Board {
    fn default() -> Board {
        Board::with_pieces(&[
            (Square::A1, Piece::Rook, Color::White),
            (Square::B1, Piece::Knight, Color::White),
            (Square::C1, Piece::Bishop, Color::White),
            (Square::D1, Piece::Queen, Color::White),
            (Square::E1, Piece::King, Color::White),
            (Square::F1, Piece::Bishop, Color::White),
            (Square::G1, Piece::Knight, Color::White),
            (Square::H1, Piece::Rook, Color::White),
            (Square::A2, Piece::Pawn, Color::White),
            (Square::B2, Piece::Pawn, Color::White),
            (Square::C2, Piece::Pawn, Color::White),
            (Square::D2, Piece::Pawn, Color::White),
            (Square::E2, Piece::Pawn, Color::White),
            (Square::F2, Piece::Pawn, Color::White),
            (Square::G2, Piece::Pawn, Color::White),
            (Square::H2, Piece::Pawn, Color::White),
            (Square::A7, Piece::Pawn, Color::Black),
            (Square::B7, Piece::Pawn, Color::Black),
            (Square::C7, Piece::Pawn, Color::Black),
            (Square::D7, Piece::Pawn, Color::Black),
            (Square::E7, Piece::Pawn, Color::Black),
            (Square::F7, Piece::Pawn, Color::Black),
            (Square::G7, Piece::Pawn, Color::Black),
            (Square::H7, Piece::Pawn, Color::Black),
            (Square::A8, Piece::Rook, Color::Black),
            (Square::B8, Piece::Knight, Color::Black),
            (Square::C8, Piece::Bishop, Color::Black),
            (Square::D8, Piece::Queen, Color::Black),
            (Square::E8, Piece::King, Color::Black),
            (Square::F8, Piece::Bishop, Color::Black),
            (Square::G8, Piece::Knight, Color::Black),
            (Square::H8, Piece::Rook, Color::Black),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let board = Board::new();

        assert_eq!(board.squares[0], None);
        assert_eq!(board.squares[63], None);
    }

    #[test]
    fn test_with_pieces() {
        let board = Board::with_pieces(&[(Square::A1, Piece::Pawn, Color::White)]);

        assert_eq!(board.squares[0], Some((Piece::Pawn, Color::White)));
    }
}
