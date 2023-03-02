//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;
use std::u64::MIN;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        
        let mut best_move: Option<Movement> = None;
        let mut best_score = i8::MAX; // On peut forcement faire mieux que la situation actuelle

        for movement in state.movements() {
            let next_state = state.play(&movement);
            let score = min_max(&next_state, self.0, true);
            // println!(
            //     "{} : max = {}, calcul = {} / mouvement : {:?}",
            //     ["red", "blue"][state.current_player as usize], best_score, score, movement
            // );
            if score < best_score {
                best_score = score;
                best_move = Some(movement);
            }
        }
        println!(
            "coup final de {} : score = {} / mouvement : {:?}",
            ["red", "blue"][state.current_player as usize], best_score, best_move
        );
        best_move
    }
}


fn min_max(state: &Configuration, depth: u8, max_player: bool) -> i8 {
    if depth == 0 {
        return state.value();
    }
    let mut best_score = state.value();

    for movement in state.movements() {
        let next_state = state.play(&movement);
        let score = min_max(&next_state, depth - 1, !max_player);
        
        if max_player {
            best_score = std::cmp::max(best_score, score);
        } else {
            best_score = std::cmp::min(best_score, score);
        }
    }

    best_score
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
