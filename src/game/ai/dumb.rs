//! Medium AI

use super::super::board::Board;
use super::super::enums::Symbol;
use super::AI;

/// Medium AI
///
/// This AI wants to win badly
///
/// It will do the winning move if possible
/// and a random one if not.
pub struct DumbAI {}

impl AI for DumbAI {
    fn do_move(&self, board: &mut Board, player: Symbol) {
        let symbol = Symbol::other(player);
        for i in 0..9 {
            if board.can_win(symbol, i / 3, i % 3) {
                board.set(i / 3, i % 3, symbol);
                return;
            }
        }
        self.place_random(board, symbol);
    }
}
