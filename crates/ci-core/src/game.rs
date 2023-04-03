use crate::{file::File, piece::Piece, rank::Rank};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum GameResult {
    WhiteWins,
    BlackWins,
    Draw,
    /**
     * game still in progress, game abandoned, or result otherwise unknown
     */
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum Player {
    White,
    Black,
}

#[derive(Debug, PartialEq)]
pub struct Move {
    pub piece: Piece,
    pub turn: Player,
    pub origin: Option<(Option<File>, Option<Rank>)>,
    pub destination: Option<(File, Rank)>,
    pub move_type: MoveType,
    pub result: Option<MoveResult>,
}

#[derive(Debug, PartialEq)]
pub enum MoveType {
    Castle(CastleType),
    Capture,
    EnPassantCapture,
    Promotion(Piece),
    Move,
}

#[derive(Debug, PartialEq)]
pub enum MoveResult {
    Check,
    Checkmate,
}

#[derive(Debug, PartialEq)]
pub enum CastleType {
    Kingside,
    Queenside,
}

#[derive(Debug)]
pub struct Game {
    pub moves: Vec<Move>,
    pub tags: HashMap<String, String>,
    pub result: GameResult,
}
