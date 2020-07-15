use super::AI;
use super::super::board::Board;
use super::super::enums::Symbol;


pub struct Player {}

impl AI for Player {
    fn do_move(&self, board: &mut Board, player: Symbol) {
        loop {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Error reading string!");
            let num = input.trim().parse::<u32>();
            match num {
                Ok(n) => match n {
                    1..=9 => {
                        let n = n - 1;
                        if board.get(n / 3, n % 3) == Symbol::Nil {
                            board.set(n / 3, n % 3, player);
                            return;
                        } else {
                            println!("Sorry, that square is already occupied!");
                        }
                    }
                    _ => {
                        println!("Please only input numbers in the 1-9 range!");
                    }
                },
                _ => {
                    println!("Sorry, I did not understand! Please input a number!");
                }
            }
        }
    }
}
