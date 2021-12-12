use crate::parser::pgn;
use std::{collections::HashMap, convert::TryInto};

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

#[derive(Debug)]
pub enum Variant {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug)]
pub struct Piece {
    variant: Variant,
    control: Player,
}

pub struct GameState {
    pub board: Vec<Square>,
}

#[derive(Debug)]

pub struct Square {
    pub x: u8,
    pub y: u8,
    piece: Option<Piece>,
}

impl Square {
    pub fn new(x: u8, y: u8, piece: Option<Piece>) -> Square {
        Square { x, y, piece }
    }
}

impl GameState {
    pub fn new() -> GameState {
        let mut board: Vec<Square> = Vec::with_capacity(64);

        for x in 0..8 {
            for y in 0..8 {
                board[x + y] =
                    Square::new(x.try_into().unwrap(), y.try_into().unwrap(), None);
            }
        }

        // Add pawns
        for x in 0..8 {
            board[x + 8] = Square::new(x.try_into().unwrap(), 8, Some(Piece {
                variant: Variant::Pawn,
                control: Player::White,
            }));

            board[x + 48] = Square::new(x.try_into().unwrap(), 8, Some(Piece {
                variant: Variant::Pawn,
                control: Player::Black,
            }));

            
        }

        println!("{:?}", board);

        GameState { board }
    }
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
