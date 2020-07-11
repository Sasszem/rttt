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

type Board = Vec<Vec<Symbol>>;

fn make_board() -> Board {
    vec!(
        vec!(Symbol::Nil, Symbol::Nil, Symbol::Nil),
        vec!(Symbol::Nil, Symbol::Nil, Symbol::Nil),
        vec!(Symbol::Nil, Symbol::Nil, Symbol::Nil)
    )
}

fn print_board(board: &Board) {
    let mut strings: Vec<&str> = Vec::new();
    for row in board {
        for cell in row {
            let sym = match cell {
                Symbol::X => "X",
                Symbol::O => "O",
                Symbol::Nil => " "
            };
            strings.push(sym);
        }
    }
    println!("{} | {} | {}", strings[0], strings[1], strings[2]);
    println!("--+---+--");
    println!("{} | {} | {}", strings[3], strings[4], strings[5]);
    println!("--+---+--");
    println!("{} | {} | {}", strings[6], strings[7], strings[8]);
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

    for i in 0..2 {
        if board.iter().map(|row| {row[i]}).filter(|sym| {*sym==s}).count()==3 {
            return true;
        }
    }

    if (0..2).map(|i| {board[i][i]}).filter(|sym| {*sym==s}).count()==3 {
        return true;
    }

    if (0..2).map(|i| {board[i][2-i]}).filter(|sym| {*sym==s}).count()==3 {
        return true;
    }

    return false;
}