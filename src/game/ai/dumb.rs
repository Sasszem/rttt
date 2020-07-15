use super::super::board::Board;
use super::super::enums::Symbol;
use super::AI;

pub struct DumbAI {}

impl AI for DumbAI {
    fn do_move(&self, board: &mut Board, player: Symbol) {
        // todo: if we are winning, do that move
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
