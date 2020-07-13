use super::enums::GameResult;
use super::enums::Symbol;

pub struct Board {
    board: Vec<Vec<Symbol>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: vec![
                vec![Symbol::Nil, Symbol::Nil, Symbol::Nil],
                vec![Symbol::Nil, Symbol::Nil, Symbol::Nil],
                vec![Symbol::Nil, Symbol::Nil, Symbol::Nil],
            ],
        }
    }

    fn format_row(row: &Vec<Symbol>) -> String {
        return format!(
            "{} | {} | {}",
            row[0].to_str(),
            row[1].to_str(),
            row[2].to_str()
        );
    }

    pub fn print(&self) {
        println!("{}", Board::format_row(&self.board[2]));
        println!("--+---+--");
        println!("{}", Board::format_row(&self.board[1]));
        println!("--+---+--");
        println!("{}", Board::format_row(&self.board[0]));
    }

    pub fn result(&self, player: Symbol) -> GameResult {
        if self.won_by(player) {
            return GameResult::Won;
        }
        if self.won_by(Symbol::other(player)) {
            return GameResult::Lost;
        }
        if self.is_draw() {
            return GameResult::Draw;
        }
        return GameResult::Running;
    }

    fn is_draw(&self) -> bool {
        // warning!
        // This only checks for fileld boards
        // run won_by before this to eliminate the possibility of a false result!
        self.board
            .iter()
            .filter(|row| row.iter().filter(|s| **s != Symbol::Nil).count() == 3)
            .count()
            == 3
    }

    fn won_by(&self, s: Symbol) -> bool {
        // thiis function checks for multiple conditions
        // and uses early return

        // check rows
        if self
            .board
            .iter()
            .filter(|row| row.iter().filter(|sym| **sym == s).count() == 3)
            .count()
            > 0
        {
            return true;
        }

        // columns
        for i in 0..3 {
            if self
                .board
                .iter()
                .map(|row| row[i])
                .filter(|sym| *sym == s)
                .count()
                == 3
            {
                return true;
            }
        }

        // diagonal 1
        if (0..3)
            .map(|i| self.board[i][i])
            .filter(|sym| *sym == s)
            .count()
            == 3
        {
            return true;
        }

        // diagonal 2
        if (0..3)
            .map(|i| self.board[i][2 - i])
            .filter(|sym| *sym == s)
            .count()
            == 3
        {
            return true;
        }

        return false;
    }

    pub fn get(&self, i: u32, j: u32) -> Symbol {
        return self.board[i as usize][j as usize];
    }

    pub fn set(&mut self, i: u32, j: u32, s: Symbol) {
        self.board[i as usize][j as usize] = s;
    }
}
