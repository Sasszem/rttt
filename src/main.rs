fn main() {
    println!("Hello, world!");
    let mut board = make_board();

    let mut result = game_result(&board);
    while result == Symbol::Nil {
        print_board(&board);
        player_move(&mut board);
        ai_move(&mut board);
        result = game_result(&board);
    }
    print_board(&board);
    match result {
        Symbol::X => {
            println!("You won!");
        },
        Symbol::O => {
            println!("Haha loser!");
        }
        _ => {}
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Symbol {
    X,
    O,
    Nil
}

impl Symbol{
    fn to_str(s: Symbol) -> String {
        return match s {
            Symbol::X => String::from("X"),
            Symbol::O => String::from("O"),
            Symbol::Nil => String::from(" ")
        };
    }
}

type Board = Vec<Vec<Symbol>>;

fn make_board() -> Board {
    vec!(
        vec!(Symbol::Nil, Symbol::Nil, Symbol::Nil),
        vec!(Symbol::Nil, Symbol::Nil, Symbol::Nil),
        vec!(Symbol::Nil, Symbol::Nil, Symbol::Nil)
    )
}

fn print_row(row: &Vec<Symbol>) -> String {
    return format!("{} | {} | {}", Symbol::to_str(row[0]), Symbol::to_str(row[1]), Symbol::to_str(row[2]));
}

fn print_board(board: &Board) {
    println!("{}", print_row(&board[0]));
    println!("--+---+--");
    println!("{}", print_row(&board[1]));
    println!("--+---+--");
    println!("{}", print_row(&board[2]));
}

fn game_result(board: &Board) -> Symbol {
    if won_by(Symbol::X, board) {
        return Symbol::X;
    }
    if won_by(Symbol::O, board) {
        return Symbol::O;
    }
    return Symbol::Nil;
}

fn player_move(board: &mut Board) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let target: u32 = input.trim().parse().expect("Input a number please!");
    board[(target/3) as usize][(target%3) as usize] = Symbol::X;
}

use rand::Rng;

fn ai_move(board: &mut Board) {
    loop {
        let target = rand::thread_rng().gen_range(0, 8);
        if board[(target/3) as usize][(target%3) as usize] == Symbol::Nil {
            board[(target/3) as usize][(target%3) as usize] = Symbol::O;
            return;
        }
    }
}

fn won_by(s: Symbol, board: &Board) -> bool {

    println!("Wonby({:?})", s);
    // check rows
    if board.iter().filter(|row| {row.iter().filter(|sym| {**sym==s}).count() == 3}).count() > 0 {
        return true;
    }

    for i in 0..3 {
        if board.iter().map(|row| {row[i]}).filter(|sym| {*sym==s}).count()==3 {
            return true;
        }
    }

    if (0..3).map(|i| {board[i][i]}).filter(|sym| {*sym==s}).count()==3 {
        return true;
    }

    if (0..3).map(|i| {board[i][2-i]}).filter(|sym| {*sym==s}).count()==3 {
        return true;
    }

    return false;
}