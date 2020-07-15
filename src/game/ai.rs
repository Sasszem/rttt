//! AI trait, implementations and selection function

use super::board::Board;
use super::enums::Symbol;
use rand::Rng;

/// AI trait defines common AI behaviour
pub trait AI {
    /// Do a single move
    ///
    /// Must be implemented
    fn do_move(&self, board: &mut Board, player: Symbol);

    /// Place a symbol randomly
    ///
    /// Only places to free (`Symbol::Nil`) squares
    ///
    /// ## Warning
    ///
    /// Can hang up a program via an infinite loop if `board` is fully filled!
    fn place_random(&self, board: &mut Board, sym: Symbol) {
        loop {
            let target = rand::thread_rng().gen_range(0, 9);
            if board.get(target / 3, target % 3) == Symbol::Nil {
                board.set(target / 3, target % 3, sym);
                return;
            }
        }
    }
}

// AI types
mod random;
pub use self::random::RandomAI;
mod dumb;
pub use self::dumb::DumbAI;
mod smart;
pub use self::smart::SmartAI;
mod player;
pub use self::player::Player;

/// Get an AI based on user input
///
/// Shows select menu, and return an AI implementation
pub fn get_ai() -> Box<dyn AI> {
    //  print menu
    println!("Please select the difficulty!");
    println!("1) Easy");
    println!("2) Normal");
    println!("3) Hard");
    println!("0) Random (default)");

    // read in line
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: could not read user input!");
    let mut choice: u32 = line.trim().parse().unwrap_or(0);

    // pick random on default or 0
    if choice == 0 || choice > 3 {
        choice = rand::thread_rng().gen_range(1, 4);
    }

    // match implementations to menu choices
    match choice {
        1 => {
            return Box::new(RandomAI {});
        }
        2 => {
            return Box::new(DumbAI {});
        }
        3 => {
            return Box::new(SmartAI {});
        }
        _ => {
            // this should be impossible to reach
            panic!("Invalid AI choice somehow!");
        }
    }
}
