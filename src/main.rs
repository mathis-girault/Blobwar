extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{Greedy, Human, MinMax, AlphaBeta, MinMaxPar};
use std::time::{Instant};

fn main() {
    let start = Instant::now(); // permet de calculer le temps d'éxecution

    // let board = Board::load("x").expect("failed loading board");
    let board = Default::default();
    let mut game = Configuration::new(&board);
    // game.battle(MinMax(1), Greedy());
    // game.battle(Greedy(), MinMax(1));
    game.battle(MinMaxPar(2), MinMax(2));

    let duration = start.elapsed(); // permet de calculer le temps d'éxecution
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
