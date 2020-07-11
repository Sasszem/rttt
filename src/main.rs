fn main() {
    let mut board = make_board();

    let player = Symbol::X;

    loop {
        let result = game_result(&board, &player);
        if result!=GameResult::Running {
            break;
        }
        print_board(&board);
        player_move(&mut board);
        let result = game_result(&board, &player);
        if result!=GameResult::Running {
            break;
        }
        ai_move(&mut board);
    }
    print_board(&board);

    let result = game_result(&board, &player);
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
    fn to_str(s: Symbol) -> String {
        return match s {
            Symbol::X => String::from("X"),
            Symbol::O => String::from("O"),
            Symbol::Nil => String::from(" ")
        };
    }

    fn other(s: &Symbol) -> Symbol {
        return match s {
            Symbol::X => Symbol::O,
            Symbol::O => Symbol::X,
            Symbol::Nil => Symbol::Nil
        }
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
    if won_by(&Symbol::other(player), board) {
        return GameResult::Lost;
    }
    if is_draw(board) {
        return GameResult::Draw;
    }
    return GameResult::Running;
}

fn player_move(board: &mut Board) {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error reading string!");
        let num = input.trim().parse::<u32>();
        match num {
            Ok(n) => {
                if n<=9 && n>=1 {
                    let n = n - 1;
                    board[(n/3) as usize][(n%3) as usize] = Symbol::X;
                    return;
                }
            }
            _ => {}
        }
        println!("Sorry, I did not understand! Please input a number in the range 1-9!");
    }
}

use rand::Rng;

fn ai_move(board: &mut Board) {
    loop {
        let target = rand::thread_rng().gen_range(0, 9);
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