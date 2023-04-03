use ci_core::game::{Game, GameResult, Move, Player};
use lazy_static::lazy_static;
use png_move::PNGMove;
use regex::Regex;
use std::collections::HashMap;

mod move_type;
mod piece;
pub mod png_move;

lazy_static! {
    static ref METADATA: Regex = Regex::new(r"\[(.*?)\]").unwrap();
    static ref GAME_LINE: Regex = Regex::new(r"(\d+.\s+\S+\s+\S+)").unwrap();
    static ref RESULT: Regex = Regex::new(r#"1/2|1-0|0-1"#).unwrap();
}

struct PNGParser<'a> {
    data: &'a str,
}

impl<'a> PNGParser<'_> {
    // TODO: Should probably not be static, as we may want to load multiple
    // PNGs during the lifetime of the program
    pub fn from_str(data: &'static str) -> Self {
        Self { data }
    }

    fn parse(&self) -> Game {
        let mut tags: HashMap<String, String> = HashMap::new();
        let mut moves: Vec<Move> = Vec::new();

        for line in self.data.lines() {
            if line.is_empty() {
                continue;
            }

            match line {
                metadata if self.is_metadata(line) => {
                    let mut parts = metadata.split('\"');
                    tags.insert(
                        parts
                            .next()
                            .expect("Could not find tag name")
                            .trim_start_matches('[')
                            .trim_end_matches(']')
                            .to_string(),
                        parts.next().expect("Could not find tag value").to_string(),
                    );
                }
                game_line if self.is_game_line(line) => {
                    for capture in GAME_LINE.captures_iter(game_line) {
                        let mut parts = capture[0].split_whitespace();
                        while let Some(_index) = parts.next() {
                            if let (Some(white_move), Some(black_move)) =
                                (parts.next(), parts.next())
                            {
                                let w_move = PNGMove::from_notation(white_move, Player::White);
                                let b_move = PNGMove::from_notation(black_move, Player::Black);

                                dbg!(white_move, w_move, black_move, b_move);
                            }
                        }
                    }
                }
                _ => {
                    println!("No idea");
                }
            }
        }

        Game {
            moves: Vec::new(),
            tags,
            result: self.get_result(),
        }
    }

    fn get_result(&self) -> GameResult {
        if let Some(result) = RESULT.captures(self.data) {
            if result.len() > 1 {
                println!("Found multiple results, that's a bit odd");
            }

            if let Some(first_result) = result.get(0) {
                return match first_result.as_str() {
                    "1/2" => GameResult::Draw,
                    "1-0" => GameResult::WhiteWins,
                    "0-1" => GameResult::BlackWins,
                    "*" | _ => GameResult::Unknown,
                };
            }
        }

        GameResult::Unknown
    }

    fn is_metadata(&self, line: &str) -> bool {
        METADATA.is_match(line)
    }

    fn is_game_line(&self, line: &str) -> bool {
        GAME_LINE.is_match(line)
    }
}

#[cfg(test)]
mod tests {
    use crate::PNGParser;

    // #[test]
    // fn test() {
    //     let parser = PNGParser::from_str(
    //         r#"
    //     [Event "F/S Return Match"]
    //     [Site "Belgrade, Serbia JUG"]
    //     [Date "1992.11.04"]
    //     [Round "29"]
    //     [White "Fischer, Robert J."]
    //     [Black "Spassky, Boris V."]
    //     [Result "1/2-1/2"]

    //     1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 {This opening is called the Ruy Lopez.}
    //     4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 9. h3 Nb8 10. d4 Nbd7
    //     11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15. Nb1 h6 16. Bh4 c5 17. dxe5
    //     Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21. Nc4 Nxc4 22. Bxc4 Nb6
    //     23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 28. Qxg5
    //     hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33. f3 Bc8 34. Kf2 Bf5
    //     35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6
    //     Nf2 42. g4 Bd3 43. Re6 1/2-1/2
    //     "#,
    //     );

    //     parser.parse();
    // }
}
