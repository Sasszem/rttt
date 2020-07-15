//! Implement the main game loop
//!
//! No need to import other modules in order to run the game

use super::ai;
use super::ai::Player;
use super::ai::AI;
use super::board::Board;
use super::enums::GameResult;
use super::enums::Symbol;

/// Game mainloop
///
/// Handles object creation, game loop, result checking and printing
pub fn run() {
    // variable initializations
    let mut board = Board::new();
    let psym = Symbol::random();
    let ai = ai::get_ai();
    let player = Player {};

    // do first move by AI
    if psym == Symbol::O {
        ai.do_move(&mut board, psym);
    }

    // print symbol message
    println!("You are {}", psym.to_str());

    // game loop
    loop {
        // check results
        let result = board.result(psym);
        if result != GameResult::Running {
            break;
        }

        // print board
        board.print();

        // player move
        player.do_move(&mut board, psym);

        // check result again
        let result = board.result(psym);
        if result != GameResult::Running {
            break;
        }

        // do AI move
        ai.do_move(&mut board, psym);
    }
    // game loop ends here
    // everythings after this is after win / lose

    board.print();

    // print game result
    let result = board.result(psym);
    match result {
        GameResult::Won => {
            println!("You won!");
        }
        GameResult::Lost => {
            println!("Haha loser!");
        }
        GameResult::Draw => {
            println!("Equally noobs!");
        }
        _ => {}
    }
}
