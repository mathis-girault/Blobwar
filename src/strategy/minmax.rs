//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;
/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

fn search_next_move(reel_depth: u8, state: &Configuration, depth: u8, player: bool) -> (i8, Option<Movement>) {
    if depth == 0 {
        //print!("Deapth vaut zéro\n");
        return ((-1 as i8).pow(reel_depth as u32)*state.value(), None);
    }
    let mut value = if player {i8::MIN} else {i8::MAX};
    let mut next_mov = None;

    if player {
        for movement in state.movements() {
            let new_state:Configuration = state.play(&movement);
            //print!("mov avant recherche récursive mov: {movement:?}\n");
            let new_value = search_next_move(reel_depth, &new_state, depth - 1, false);
            //print!("best_mov: {next_mov:?} mov: {movement:?}, best_val: {value} val: {}\n", new_value.0);
            if new_value.0 > value {
                value = new_value.0;
                next_mov = Some(movement);
            }
            // print!("best_mov: {next_mov:?} next mov: {movement:?}, best_val: {value} val: {}\n", new_value.0);
        }
    }
    else {
        for movement in state.movements() {
            let new_state:Configuration = state.play(&movement);
            // print!("mov avant recherche récursive mov: {movement:?}\n");
            let new_value = search_next_move( reel_depth, &new_state, depth - 1, true);
            // print!("best_mov: {next_mov:?} next mov: {movement:?}, best_val: {value} val: {}\n", new_value.0);
            if new_value.0 < value {
                value = new_value.0;
                next_mov = Some(movement);
            }
        }
    };

    return (value, next_mov);
}


impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        
        let next_mov = search_next_move(self.0, state, self.0 + 1, true).1;
        //print!("{next_mov:?}");
        return next_mov;
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMax(depth).compute_next_move(state));
    }
}
