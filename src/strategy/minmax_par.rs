//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;
use rayon::iter::ParallelIterator;
/// Min-Max algorithm with a given recursion depth.
pub struct MinMaxPar(pub u8);

impl Strategy for MinMaxPar {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let mut best_move: Option<Movement> = None;
        let mut best_score = i8::MIN;

        let movements = state.movements().into_par_iter();

        movements.for_each(|movement| {
            let next_state = state.play(&movement);
            let score = min_max_par(&next_state, self.0, 0 == (self.0 % 2), false);

            if score > best_score {
                best_score = score;
                best_move = Some(movement);
            }
        });

        best_move
    }
}

fn min_max_par(state: &Configuration, depth: u8, even_depth: bool, max_player: bool) -> i8 {
    if depth == 0 {
        return if even_depth {state.value()} else {-state.value()};
    }

    let movements = state.movements().into_par_iter().map(|m| {
        let next_state = state.play(&m);
        let score = min_max_par(&next_state, depth - 1, even_depth, !max_player);
        (score, max_player)
    });

    let (best_score, _) = if max_player {
        movements.max().unwrap_or((i8::MIN, max_player))
    } else {
        movements.min().unwrap_or((i8::MAX, max_player))
    };

    best_score
}

impl<'a> rayon::iter::ParallelIterator for &'a mut (dyn Iterator<Item = Movement> + 'a) {
    type Item = Movement;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        rayon::iter::plumbing::bridge_unindexed(self, consumer)
    }
}

impl fmt::Display for MinMaxPar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_par_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMaxPar(depth).compute_next_move(state));
    }
}
