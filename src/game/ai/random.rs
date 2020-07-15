use super::AI;
use rand::Rng;
use super::super::board::Board;
use super::super::enums::Symbol;


pub struct RandomAI {}

impl AI for RandomAI {
    fn do_move(&self, board: &mut Board, player: Symbol) {
        loop {
            let target = rand::thread_rng().gen_range(0, 9);
            if board.get(target / 3, target % 3) == Symbol::Nil {
                board.set(target / 3, target % 3, Symbol::other(player));
                return;
            }
        }
    }
}