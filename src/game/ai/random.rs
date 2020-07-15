//! Easy AI

use super::super::board::Board;
use super::super::enums::Symbol;
use super::AI;

/// Easy AI
///
/// Always places at random
pub struct RandomAI {}

impl AI for RandomAI {
    fn do_move(&self, board: &mut Board, player: Symbol) {
        self.place_random(board, Symbol::other(player));
    }
}
