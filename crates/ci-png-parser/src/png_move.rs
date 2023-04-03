use std::str::FromStr;

use ci_core::{
    file::File,
    game::{Move, Player},
    rank::Rank,
};
use regex::Regex;

use crate::piece::PNGPiece;

pub struct PNGMove(Move);

impl PNGMove {
    // ! Transforms PNG SAN notation to concrete move data.
    // !
    // ! # Examples
    // ! ```
    // ! use ci_png_parser::{png_move::PNGMove};
    // ! use ci_core::{file::File, game::Move, game::MoveType, game::Player, piece::Piece, rank::Rank};
    // !
    // ! let notation = "e4";
    // ! let chess_move = PNGMove::from_notation(notation, Player::White);
    // !
    // ! assert!(chess_move == Some(Move {
    // !   piece: Piece::Pawn,
    // !   turn: Player::White,
    // !   move_type: MoveType::Move,
    // !   destination: Some((File::E, Rank::Four)),
    // !   result: None,
    // ! }))
    // ! ```
    pub fn from_notation(notation: &str, turn: Player) -> Option<Move> {
        let re = Regex::new("^([KQRBNP])?([a-h])([1-8])$").unwrap();
        if let Some(captures) = re.captures(notation) {
            let [_match, piece, file, rank]: &[&str] = &captures
                .iter()
                .map(|capture| capture.as_ref().map(|m| m.as_str()).unwrap_or(""))
                .collect::<Vec<&str>>()[..] else {
                    panic!("Did not receive valid PNG SAN notation {}", notation);
                };

            dbg!(piece, file, rank);
            // TODO: castling, captures, checks, checkmates, as well as skip the result of the game
            // idea: do parse the result of the game on move level, instead of game level, keeps the games spoiler free!
            return Some(Move {
                piece: PNGPiece::from_str(piece).expect("Unknown piece").get(),
                turn,
                origin: None,
                destination: Some((
                    File::from_str(file).expect("Unknown file"),
                    Rank::from_str(rank).expect("Unknown rank"),
                )),
                move_type: ci_core::game::MoveType::Move,
                result: None,
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::PNGMove;
    use ci_core::{
        file::File,
        game::CastleType,
        game::Move,
        game::Player,
        game::{MoveResult, MoveType},
        piece::Piece,
        rank::Rank,
    };

    #[test]
    pub fn test() {
        [
            (
                "e4", // Pawn moves to e4
                Some(Move {
                    piece: Piece::Pawn,
                    turn: Player::White,
                    move_type: MoveType::Move,
                    origin: None,
                    destination: Some((File::E, Rank::Four)),
                    result: None,
                }),
            ),
            (
                "Re6", // Rook moves to e6
                Some(Move {
                    piece: Piece::Rook,
                    turn: Player::White,
                    move_type: MoveType::Move,
                    origin: None,
                    destination: Some((File::E, Rank::Six)),
                    result: None,
                }),
            ),
            (
                "exd5", // Pawn on the E file captures piece on d5
                Some(Move {
                    piece: Piece::Pawn,
                    turn: Player::White,
                    move_type: MoveType::Capture,
                    origin: None,
                    destination: Some((File::D, Rank::Five)),
                    result: None,
                }),
            ),
            (
                "Rdf8", // Rook on the D file moves to f8, this is required if both rooks are eligible to make the move to f8
                Some(Move {
                    piece: Piece::Rook,
                    turn: Player::White,
                    move_type: MoveType::Move,
                    origin: Some((Some(File::D), None)),
                    destination: Some((File::F, Rank::Eight)),
                    result: None,
                }),
            ),
            (
                "exd6 e.p", // Pawn on the E file captures en passant and moves to d6
                Some(Move {
                    piece: Piece::Pawn,
                    turn: Player::White,
                    move_type: MoveType::EnPassantCapture,
                    origin: Some((Some(File::E), None)),
                    destination: Some((File::D, Rank::Six)),
                    result: None,
                }),
            ),
            (
                "O-O", // Castle kingside
                Some(Move {
                    piece: Piece::King,
                    turn: Player::White,
                    move_type: MoveType::Castle(CastleType::Kingside),
                    origin: None,
                    destination: None,
                    result: None,
                }),
            ),
            (
                "O-O-O", // Castle queenside
                Some(Move {
                    piece: Piece::King,
                    turn: Player::White,
                    move_type: MoveType::Castle(CastleType::Queenside),
                    origin: None,
                    destination: None,
                    result: None,
                }),
            ),
            (
                "Rxe1+", // Rook captures on E1 and puts opponent in check position
                Some(Move {
                    piece: Piece::Rook,
                    turn: Player::White,
                    move_type: MoveType::Capture,
                    origin: None,
                    destination: Some((File::E, Rank::One)),
                    result: Some(MoveResult::Check),
                }),
            ),
            (
                "e8=Q", // Pawn moves to E8 and promotes to queen
                Some(Move {
                    piece: Piece::Pawn,
                    turn: Player::White,
                    move_type: MoveType::Promotion(Piece::Queen),
                    origin: None,
                    destination: Some((File::E, Rank::Eight)),
                    result: None,
                }),
            ),
            (
                "Kxe4#", // Knight captures on e4 and checkmates the opponent
                Some(Move {
                    piece: Piece::Knight,
                    turn: Player::White,
                    move_type: MoveType::Capture,
                    origin: None,
                    destination: Some((File::E, Rank::Four)),
                    result: Some(MoveResult::Checkmate),
                }),
            ),
        ]
        .map(|(notation, expected)| {
            let result = PNGMove::from_notation(notation, ci_core::game::Player::White);
            assert_eq!(result, expected);
        });
    }
}
