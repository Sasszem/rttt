use super::board::Board;
use super::enums::Symbol;
use rand::Rng;

pub trait AI {
    fn do_move(&self, board: &mut Board, player: Symbol);

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

mod random;
pub use self::random::RandomAI;

mod dumb;
pub use self::dumb::DumbAI;
mod smart;
pub use self::smart::SmartAI;

mod player;
pub use self::player::Player;

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
            return Box::new(DumbAI {});
        }
        3 => {
            return Box::new(SmartAI {});
        }
        _ => {
            panic!("Invalid AI choice somehow!");
        }
    }
}
