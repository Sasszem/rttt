use crate::board::Board;
use crate::symbol::Symbol;
use crate::ai;
use crate::ai::AI;
use crate::gameresult::GameResult;
use crate::ai::Player;

pub struct Game {
    ai: Box<dyn AI>,
    board: Board,
    player: Symbol
}

impl Game {
    pub fn new() -> Game {
        Game{
            ai: ai::get_ai(),
            board: Board::new(),
            player: Symbol::random(),
        }
    }

    pub fn run(&mut self) {
        if self.player == Symbol::O {
            self.ai.do_move(&mut self.board, self.player);
        }
        
        println!("You are {}", self.player.to_str());
    
        let player = Player{};

        loop {
            let result = self.board.result(self.player);
            if result!=GameResult::Running {
                break;
            }
            self.board.print();
            player.do_move(&mut self.board, self.player);
            let result = self.board.result(self.player);
            if result!=GameResult::Running {
                break;
            }
            self.ai.do_move(&mut self.board, self.player);
        }
        self.board.print();
    
        let result = self.board.result(self.player);
        match result {
            GameResult::Won => {
                println!("You won!");
            },
            GameResult::Lost => {
                println!("Haha loser!");
            },
            GameResult::Draw => {
                println!("Equally noobs!");
            },
            _ => {}
        }
    }
}
