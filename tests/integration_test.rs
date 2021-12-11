#![cfg(test)]
mod tests {
    use chess_interpreter::pgn_parser::{parse, Outcome};
    use glob::glob;

    #[test]
    fn should_parse() {
        for entry in glob("tests/games/*.pgn").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    println!("{:?}", path.display());
                    let contents = std::fs::read_to_string(path).unwrap();
                    let game = parse(&contents);

                    assert!(game.outcome != Outcome::Unknown, "Game has unknown outcome: {:?}", game);
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
