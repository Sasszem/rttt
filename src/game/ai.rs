use super::board::Board;
use super::enums::Symbol;
use rand::Rng;

pub trait AI {
    fn do_move(&self, board: &mut Board, player: Symbol);
}

struct RandomAI {}

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

struct SmarterAI {}

impl AI for SmarterAI {
    fn do_move(&self, board: &mut Board, player: Symbol) {
        // todo: if we are winning, do that move
        // if player is winning, block it
        // do random move
    }
}

struct SmartAI {}

impl AI for SmartAI {
    fn do_move(&self, board: &mut Board, player: Symbol) {
        // todo: win if can
        // block if must
        // pick in priority order: corner, edge, middle
    }
}

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

pub fn get_ai() -> Box<dyn AI> {
    println!("Please select the difficulty!");
    println!("1) Easy");
    println!("2) Normal");
    println!("3) Hard");
    println!("0) Random (default)");

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: could not read user input!");
    let mut choice: u32 = line.trim().parse().unwrap_or(0);
    if choice == 0 || choice > 3 {
        choice = rand::thread_rng().gen_range(1, 4);
    }

    match choice {
        1 => {
            return Box::new(RandomAI {});
        }
        2 => {
            return Box::new(SmarterAI {});
        }
        3 => {
            return Box::new(SmartAI {});
        }
        _ => {
            panic!("Invalid AI choice somehow!");
        }
    }
}
