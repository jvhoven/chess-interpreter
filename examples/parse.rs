use lib::game::Game;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("examples/game.pgn").expect("Something went wrong reading the file");
    let game = Game::from_string(&contents);

    println!("{:?}", game);
}
