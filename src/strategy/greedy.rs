//! Dumb greedy algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use std::{fmt, u64::MAX};

/// Dumb algorithm.
/// Amongst all possible movements return the one which yields the configuration with the best
/// immediate value.
pub struct Greedy();

impl fmt::Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greedy")
    }
}

impl Strategy for Greedy {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        
        let mut best_movement = None;

        if state.movements().peekable().peek().is_some() {
            // On inverse car value renvoie de combien il perd
            let mut max_value = i8::MIN;

            for movement in state.movements() {
                let next_state = state.play(&movement);
                let new_value = next_state.value();
                // println!(
                //     "{} : max = {}, calcul = {} / mouvement : {:?}",
                //     ["red", "blue"][state.current_player as usize], max_value, new_value, movement
                // );
                if new_value > max_value {
                    max_value = new_value;
                    best_movement = Some(movement);
                }
            }
        }

        best_movement
    }
}
