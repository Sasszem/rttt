//! Player prototype

use super::super::board::Board;
use super::super::enums::Symbol;
use super::AI;

/// Player AI
/// 
/// A special AI type for the player.
/// Does a move based on player input
pub struct Player {}

impl AI for Player {
    fn do_move(&self, board: &mut Board, player: Symbol) {
        // Loop until valid input
        loop {
            // Read in line
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Error reading string!");
            
            // Parse number and handle errors
            let num = input.trim().parse::<u32>();
            match num {
                // Verify range and handle errors
                Ok(n) => match n {
                    // Correct range
                    1..=9 => {
                        let n = n - 1; // off-by-one

                        // only alllow empty squares
                        if board.get(n / 3, n % 3) == Symbol::Nil {
                            board.set(n / 3, n % 3, player);
                            return;
                        } else {
                            println!("Sorry, that square is already occupied!");
                        }
                    }
                    // range error
                    _ => {
                        println!("Please only input numbers in the 1-9 range!");
                    }
                },
                // Handle invalid number
                _ => {
                    println!("Sorry, I did not understand! Please input a number!");
                }
            }
        }
    }
}
