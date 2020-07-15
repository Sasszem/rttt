use super::AI;
use super::super::board::Board;
use super::super::enums::Symbol;


pub struct SmartAI {}

impl AI for SmartAI {
    fn do_move(&self, board: &mut Board, player: Symbol) {
        // todo: win if can
        // block if must
        // pick in priority order: corner, edge, middle
    }
}
