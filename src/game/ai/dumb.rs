use super::AI;
use rand::Rng;
use super::super::board::Board;
use super::super::enums::Symbol;


pub struct DumbAI {}

impl AI for DumbAI {
    fn do_move(&self, board: &mut Board, player: Symbol) {
        // todo: if we are winning, do that move
        // if player is winning, block it
        // do random move
    }
}