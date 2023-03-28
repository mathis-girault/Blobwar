extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{Greedy, Human, MinMax};

fn main() {
    //let board = Board::load("x").expect("failed loading board");
    let board = Default::default();
    let mut game = Configuration::new(&board);
    // game.battle(MinMax(1), Greedy());
    //game.battle(Greedy(), MinMax(1));
    game.battle(MinMax(3), MinMax(1));
}
