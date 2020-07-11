fn main() {
    let mut board = make_board();

    let player = Symbol::X;

    let mut result = game_result(&board, &player);
    while result == GameResult::Running {
        print_board(&board);
        player_move(&mut board);
        ai_move(&mut board);
        result = game_result(&board, &player);
    }
    print_board(&board);
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

fn game_result(board: &Board, player: &Symbol) -> GameResult {
    if won_by(player, board) {
        return GameResult::Won;
    }
    if won_by(player, board) {
        return GameResult::Lost;
    }
    if is_draw(board) {
        return GameResult::Draw;
    }
    return GameResult::Running;
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

#[derive(PartialEq)]
enum GameResult {
    Won,
    Lost,
    Draw,
    Running
}

fn is_draw(board: &Board) -> bool {
    board.iter().filter(|row| {
        row.iter().filter(|s| {
            **s != Symbol::Nil
        }).count() == 3
    }).count() == 3
}

fn won_by(s: &Symbol, board: &Board) -> bool {
    println!("Wonby({:?})", s);
    // check rows
    if board.iter().filter(|row| {row.iter().filter(|sym| {*sym==s}).count() == 3}).count() > 0 {
        return true;
    }

    for i in 0..3 {
        if board.iter().map(|row| {row[i]}).filter(|sym| {sym==s}).count()==3 {
            return true;
        }
    }

    if (0..3).map(|i| {board[i][i]}).filter(|sym| {sym==s}).count()==3 {
        return true;
    }

    if (0..3).map(|i| {board[i][2-i]}).filter(|sym| {sym==s}).count()==3 {
        return true;
    }

    return false;
}