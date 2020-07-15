use super::AI;
use super::super::board::Board;
use super::super::enums::Symbol;
use rand::Rng;

pub struct SmartAI {}

impl AI for SmartAI {
    fn do_move(&self, board: &mut Board, player: Symbol) {
        let sym = Symbol::other(player);
        
        // win if can
        for i in 0..9 {
            if board.can_win(sym, i/3, i%3) {
                board.set(i/3, i%3, sym);
                return;
            }
        }

        // block if must
        for i in 0..9 {
            if board.can_win(player, i/3, i%3) {
                board.set(i/3, i%3, sym);
                return;
            }
        }

        // pick in priority order: corner, edge, middle
        
        // corners
        if CORNERS.iter().map(|x| {board.get(x.0, x.1)}).filter(|s| {*s==Symbol::Nil}).count() > 0 {
            loop {
                let (i, j) = CORNERS[rand::thread_rng().gen_range(0, 4)];
                if board.get(i, j) == Symbol::Nil {
                    board.set(i, j, sym);
                    return;
                }
            }
        }

        // edges
        if EDGES.iter().map(|x| {board.get(x.0, x.1)}).filter(|s| {*s==Symbol::Nil}).count() > 0 {
            loop {
                let (i, j) = EDGES[rand::thread_rng().gen_range(0, 4)];
                if board.get(i, j) == Symbol::Nil {
                    board.set(i, j, sym);
                    return;
                }
            }
        }

        // center
        board.set(1, 1, sym);
    }
}

const CORNERS: [(u32, u32); 4] = [(0, 0), (0, 2), (2, 0), (2, 2)];
const EDGES: [(u32, u32); 4] = [(0, 1), (1, 0), (1, 2), (2, 1)];