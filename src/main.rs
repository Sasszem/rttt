fn main() {
    let mut board = Board::new();

    let player_sym = match rand::thread_rng().gen_range(0,2) {
        0 => Symbol::O,
        1 => Symbol::X,
        _ => panic!("Invalid random player symbol number!")
    };

    let ai = RandomAI{};
    let player = Player{};

    if player_sym == Symbol::O {
        ai.do_move(&mut board, player_sym);
    }
    println!("You are {}", player_sym.to_str());

    loop {
        let result = board.result(player_sym);
        if result!=GameResult::Running {
            break;
        }
        board.print();
        player.do_move(&mut board, player_sym);
        let result = board.result(player_sym);
        if result!=GameResult::Running {
            break;
        }
        ai.do_move(&mut board, player_sym);
    }
    board.print();

    let result = board.result(player_sym);
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

#[derive(PartialEq, Copy, Clone)]
enum Symbol {
    X,
    O,
    Nil
}

impl Symbol{
    fn to_str(&self) -> String {
        return match self {
            Symbol::X => String::from("X"),
            Symbol::O => String::from("O"),
            Symbol::Nil => String::from(" ")
        };
    }

    fn other(&self) -> Symbol {
        return match self {
            Symbol::X => Symbol::O,
            Symbol::O => Symbol::X,
            Symbol::Nil => Symbol::Nil
        }
    }
}

trait AI {
    fn do_move(&self, board: &mut Board, player: Symbol);
}

struct RandomAI {}

impl AI for RandomAI {
    fn do_move(&self, board: &mut Board, player: Symbol) {
        loop {
            let target = rand::thread_rng().gen_range(0, 9);
            if board.get(target/3, target%3) == Symbol::Nil {
                board.set(target/3, target%3, player.other());
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

struct Player {}

impl AI for Player {
    fn do_move(&self, board: &mut Board, player: Symbol) {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Error reading string!");
            let num = input.trim().parse::<u32>();
            match num {
                Ok(n) => {
                    if n<=9 && n>=1 {
                        let n = n - 1;
                        board.set(n/3, n%3, player);
                        return;
                    }
                }
                _ => {}
            }
            println!("Sorry, I did not understand! Please input a number in the range 1-9!");
        }
    }
}


struct Board {
    board: Vec<Vec<Symbol>>, 
}

impl Board {
    fn new() -> Board {
        Board{
            board: vec!(
                vec!(Symbol::Nil, Symbol::Nil, Symbol::Nil),
                vec!(Symbol::Nil, Symbol::Nil, Symbol::Nil),
                vec!(Symbol::Nil, Symbol::Nil, Symbol::Nil)
            )
        }
    }

    
    fn format_row(row: &Vec<Symbol>) -> String {
        return format!("{} | {} | {}", row[0].to_str(), row[1].to_str(), row[2].to_str());
    }

    fn print(&self) {
        println!("{}", Board::format_row(&self.board[2]));
        println!("--+---+--");
        println!("{}", Board::format_row(&self.board[1]));
        println!("--+---+--");
        println!("{}", Board::format_row(&self.board[0]));
    }
    
    fn result(&self, player: Symbol) -> GameResult {
        if self.won_by(player) {
            return GameResult::Won;
        }
        if self.won_by(player.other()) {
            return GameResult::Lost;
        }
        if self.is_draw() {
            return GameResult::Draw;
        }
        return GameResult::Running;
    }

    fn is_draw(&self) -> bool {
        self.board.iter().filter(|row| {
            row.iter().filter(|s| {
                **s != Symbol::Nil
            }).count() == 3
        }).count() == 3
    }
    
    fn won_by(&self, s: Symbol) -> bool {
        // check rows
        if self.board.iter().filter(|row| {row.iter().filter(|sym| {**sym==s}).count() == 3}).count() > 0 {
            return true;
        }
    
        for i in 0..3 {
            if self.board.iter().map(|row| {row[i]}).filter(|sym| {*sym==s}).count()==3 {
                return true;
            }
        }
    
        if (0..3).map(|i| {self.board[i][i]}).filter(|sym| {*sym==s}).count()==3 {
            return true;
        }
    
        if (0..3).map(|i| {self.board[i][2-i]}).filter(|sym| {*sym==s}).count()==3 {
            return true;
        }
    
        return false;
    }

    fn get(&self, i:u32, j:u32) -> Symbol {
        return self.board[i as usize][j as usize];
    }

    fn set(&mut self, i:u32, j:u32, s: Symbol) {
        println!("Board.set({}, {}, {})", i, j, s.to_str());
        self.board[i as usize][j as usize] = s;
    }
}

use rand::Rng;

#[derive(PartialEq)]
enum GameResult {
    Won,
    Lost,
    Draw,
    Running
}


