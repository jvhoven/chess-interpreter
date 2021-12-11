#![cfg(test)]
mod tests {
    use lib::game::GameResult;
    use lib::parser::pgn;
    use glob::glob;

    #[test]
    fn should_parse() {
        for entry in glob("tests/games/*.pgn").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    println!("{:?}", path.display());
                    let contents = std::fs::read_to_string(path).unwrap();
                    let game = pgn::parse(&contents);

                    assert!(game.result != GameResult::Unknown, "Game has unknown outcome: {:?}", game);
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
