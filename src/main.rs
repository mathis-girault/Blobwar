extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{Greedy, Human, MinMax};
use std::time::{Instant};

fn main() {
    let start = Instant::now();

    // let board = Board::load("x").expect("failed loading board");
    let board = Default::default();
    let mut game = Configuration::new(&board);
    // game.battle(MinMax(1), Greedy());
    // game.battle(Greedy(), MinMax(1));
    game.battle(MinMax(1), MinMax(3));

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
