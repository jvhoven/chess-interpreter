use std::str::FromStr;

use ci_core::{
    game::{
        CastleType,
        MoveType::{self, Promotion},
    },
    piece::Piece,
};

#[derive(Debug, PartialEq, Eq)]
struct UnknownMoveTypeError;

struct PNGMoveType(MoveType);

impl FromStr for PNGMoveType {
    type Err = UnknownMoveTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "O-O-O" => Ok(PNGMoveType(MoveType::Castle(CastleType::Queenside))),
            "O-O" => Ok(PNGMoveType(MoveType::Castle(CastleType::Kingside))),
            "x" => Ok(PNGMoveType(MoveType::Capture)),
            "=Q" => Ok(PNGMoveType(Promotion(Piece::Queen))),
            "=N" => Ok(PNGMoveType(Promotion(Piece::Knight))),
            "=B" => Ok(PNGMoveType(Promotion(Piece::Bishop))),
            "=R" => Ok(PNGMoveType(Promotion(Piece::Rook))),
            _ => Err(UnknownMoveTypeError),
        }
    }
}
