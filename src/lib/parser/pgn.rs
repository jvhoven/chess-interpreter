use crate::game::{GameResult, Move};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ParseResult {
    pub moves: Vec<Move>,
    pub tags: HashMap<String, String>,
    pub result: GameResult,
}

pub fn parse(pgn_data: &str) -> ParseResult {
    let (moves, tags) = format(pgn_data);
    ParseResult {
        tags,
        moves: parse_moves(&moves),
        result: parse_result(&moves),
    }
}

fn format(data: &str) -> (String, HashMap<String, String>) {
    let mut moves = String::new();
    let mut metadata = HashMap::new();

    for line in data.lines() {
        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        if is_metadata(line) {
            let mut parts = line.split('\"');
            let key = parts
                .next()
                .unwrap()
                .trim_start_matches('[')
                .trim_end_matches(']')
                .to_string();
            let value = parts.next().unwrap().to_string();

            metadata.insert(key, value);
        } else {
            moves.push_str(format!("{} ", line).as_str());
        }
    }
    (moves, metadata)
}

fn parse_moves(data: &str) -> Vec<Move> {
    let re: Regex = Regex::new(r"(\d+.\s+\S+\s+\S+)").unwrap();
    re.captures_iter(data)
        .map(|capture| Move {
            white: capture[1].split(' ').nth(1).unwrap().to_string(),
            black: capture[1].split(' ').nth(2).unwrap().to_string(),
            number: capture[1]
                .split('.').next()
                .unwrap()
                .parse::<u16>()
                .expect("Could not parse move number"),
        })
        .collect::<Vec<Move>>()
}

fn parse_result(data: &str) -> GameResult {
    let re: Regex = Regex::new(r#"1/2|1-0|0-1"#).unwrap();
    match re
        .captures(data)
        .expect("Could not find match result in data")
        .get(0)
        .unwrap()
        .as_str()
    {
        "1/2" => GameResult::Draw,
        "1-0" => GameResult::WhiteWins,
        "0-1" => GameResult::BlackWins,
        _ => GameResult::Unknown,
    }
}

fn is_metadata(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[(.*?)\]").unwrap();
    }
    RE.is_match(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{GameResult, Move};

    #[test]
    fn test_parse_moves() {
        [
            (
                "",
                Box::new(|moves: Vec<Move>| assert!(moves.is_empty())) as Box<dyn Fn(Vec<Move>)>,
            ),
            (
                "1. e4 e5",
                Box::new(|moves: Vec<Move>| assert!(moves.len() == 1)) as Box<dyn Fn(Vec<Move>)>,
            ),
        ]
        .map(|(input, assertion)| {
            let moves = parse_moves(input);
            assertion(moves);
        });
    }

    #[test]
    fn test_parse_outcome() {
        [
            (
                "1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 4. a7 1/2-1/2",
                GameResult::Draw,
            ),
            (
                "1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 4. a7 1-0",
                GameResult::WhiteWins,
            ),
            (
                "1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 4. a7 0-1",
                GameResult::BlackWins,
            ),
            ("1-0", GameResult::WhiteWins),
            ("0-1", GameResult::BlackWins),
        ]
        .map(|(game, expected)| {
            assert!(parse_result(game) == expected);
        });
    }
}
