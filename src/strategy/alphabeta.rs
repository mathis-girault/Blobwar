//! Alpha - Beta algorithm.
use std::fmt;

use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBeta(pub u8);



impl Strategy for AlphaBeta {
    /** 
     * Fonction qui renvoie le meilleur mouvement possible pour une profondeur de récursion
     * donnée et en suivant l'algorithme de alpha beta.
     */
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        
        let mut best_move: Option<Movement> = None;
        let mut best_score = i8::MIN;
        let mut alpha = i8::MIN;
        let beta = i8::MAX;

        for movement in state.movements() {
            let next_state = state.play(&movement);
            let score = alpha_beta(&next_state, self.0, 0 == (self.0 % 2), false, alpha, beta);
            
            if score > best_score {
                best_score = score;
                best_move = Some(movement);
            }
            alpha = std::cmp::max(alpha, best_score);
        }
        
        best_move
    }
}

/** 
 * Fonction recursive qui implemente l'algorithme de alpha-beta pour le blobwar.
 * @params state l'etat de jeu actuel
 * @params depth la pronfondeur actuelle de récursion
 * @params even_depth la parité de pronfondeur de départ
 * @params max_player le joueur courant pour le calcul
 * @params alpha et beta, les deux paramètres pour l'algorithme alpha-beta
 *
 * @returns elle renvoie le meilleur score pour la pronfondeur de récursion demandée
 * en suivant la méthode de alpha-beta.
 */
fn alpha_beta(state: &Configuration, depth: u8, even_depth: bool, max_player: bool, mut alpha: i8, beta: i8) -> i8 {
    
    if depth == 0 {
        // En fonction de la valeur de even_depth, on renvoie la valeur en négatif ou positif 
        // car le gain calculé dépend du joueur courant, et donc de la pronfondeur d'appel
        return if even_depth {state.value()} else {-state.value()};
    }

    let mut best_score = if max_player {i8::MIN} else {i8::MAX};

    for movement in state.movements() {
        let next_state = state.play(&movement);
        let score = alpha_beta(&next_state, depth - 1, even_depth, !max_player, alpha, beta);
        
        if max_player {
            best_score = std::cmp::max(best_score, score);
            alpha = std::cmp::max(alpha, best_score);
        } else {
            best_score = std::cmp::min(best_score, score);
            beta = std::cmp::min(beta, best_score);
        }

        if alpha >= beta {
            break;
        }
    }

    best_score
}


impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        unimplemented!("implementer alpha beta")
    }
}
