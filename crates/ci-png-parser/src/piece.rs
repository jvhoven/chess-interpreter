use std::str::FromStr;

use ci_core::piece::Piece;

#[derive(Debug)]
pub struct PNGPiece(Piece);

impl PNGPiece {
    pub fn get(&self) -> Piece {
        self.0
    }
}

impl FromStr for PNGPiece {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // The SAN (Standard Algebraic Notation) format omits the letter for a pawn
            "P" => Ok(PNGPiece(Piece::Pawn)),
            "N" => Ok(PNGPiece(Piece::Knight)),
            "B" => Ok(PNGPiece(Piece::Bishop)),
            "R" => Ok(PNGPiece(Piece::Rook)),
            "Q" => Ok(PNGPiece(Piece::Queen)),
            "K" => Ok(PNGPiece(Piece::King)),
            _ => Ok(PNGPiece(Piece::Pawn)),
        }
    }
}
