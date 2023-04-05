//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;
/// Min-Max algorithm with a given recursion depth.
pub struct MinMaxPar(pub u8);


impl Strategy for MinMaxPar {
    /** 
     * Fonction qui renvoie le meilleur mouvement possible pour une profondeur de récursion
     * donnée et en suivant l'algorithme de MinMax.
     */
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        
        let mut best_move: Option<Movement> = None;
        let mut best_score = i8::MIN;

        for movement in state.movements() {
            let next_state = state.play(&movement);
            let score = min_max_par(&next_state, self.0, 0 == (self.0 % 2), false);
            
            if score > best_score {
                best_score = score;
                best_move = Some(movement);
            }
        }
        
        best_move
    }
}



/** 
 * Fonction recursive qui implemente l'algorithme de MinMax pour le blobwar.
 * @params state l'etat de jeu actuel
 * @params depth la pronfondeur actuelle de récursion
 * @params even_depth la parité de pronfondeur de départ
 * @params max_player le joueur courant pour le calcul
 *
 * @returns elle renvoie le meilleur score pour la pronfondeur de récursion demandée
 * en suivant la méthode de MinMax.
 */
fn min_max_par(state: &Configuration, depth: u8, even_depth: bool, max_player: bool) -> i8 {
    
    if depth == 0 {
        // En fonction de la valeur de even_depth, on renvoie la valeur en négatif ou positif 
        // car le gain calculé dépend du joueur courant, et donc de la pronfondeur d'appel
        return if even_depth {state.value()} else {-state.value()};
    }

    let mut best_score = if max_player {i8::MIN} else {i8::MAX};

    for movement in state.movements() {
        let next_state = state.play(&movement);
        let score = min_max(&next_state, depth - 1, even_depth, !max_player);

        if max_player {
            best_score = std::cmp::max(best_score, score);
        } else {
            best_score = std::cmp::min(best_score, score);
        }
    }

    best_score
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
