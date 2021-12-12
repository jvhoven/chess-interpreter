use crate::{parser::pgn, square::Square};
use std::{collections::HashMap};

#[derive(Debug, PartialEq)]
pub enum GameResult {
    WhiteWins,
    BlackWins,
    Draw,
    Unknown,
}

#[derive(Debug)]
pub enum Player {
    White,
    Black,
}

#[derive(Debug)]
pub struct Move {
    pub white: String,
    pub black: String,
    pub number: u16,
}

#[derive(Debug)]
pub struct Game {
    moves: Vec<Move>,
    tags: HashMap<String, String>,
    result: GameResult,
}

pub struct GameState {
    pub board: [[Square; 8]; 8],
}

impl Game {
    pub fn from_string(data: &str) -> Game {
        let result = pgn::parse(data);

        Game {
            moves: result.moves,
            tags: result.tags,
            result: result.result,
        }
    }
}
