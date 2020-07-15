//! Implement the board struct

use super::enums::GameResult;
use super::enums::Symbol;

/// Board struct holds [Symbols](../enums/symbol/enum.Symbol.html)
pub struct Board {
    board: Vec<Vec<Symbol>>,
}

impl Board {
    /// Create new board filled with Nil
    pub fn new() -> Board {
        Board {
            board: vec![
                vec![Symbol::Nil, Symbol::Nil, Symbol::Nil],
                vec![Symbol::Nil, Symbol::Nil, Symbol::Nil],
                vec![Symbol::Nil, Symbol::Nil, Symbol::Nil],
            ],
        }
    }

    /// print the provided row formatted
    fn print_row(row: &Vec<Symbol>) {
        println!(
            "{} | {} | {}",
            row[0].to_str(),
            row[1].to_str(),
            row[2].to_str()
        );
    }

    /// print the whole board formatted
    pub fn print(&self) {
        Board::print_row(&self.board[2]);
        println!("--+---+--");
        Board::print_row(&self.board[1]);
        println!("--+---+--");
        Board::print_row(&self.board[0]);
    }

    /// Calculate result for a given symbol
    /// 
    /// If game is not won, lost or a draw, then it's still in progress
    /// (e.g running)
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

    /// determine if the current board is a draw
    /// ## warning
    /// This only checks for fully filled boards
    /// run won_by before this to eliminate the possibility of a false result!
    fn is_draw(&self) -> bool {
        self.board
            .iter()
            .filter(|row| row.iter().filter(|s| **s != Symbol::Nil).count() == 3)
            .count()
            == 3
    }


    /// Check if the board is won by a given symbol
    /// 
    /// Checks rows, columns and diagonals
    fn won_by(&self, s: Symbol) -> bool {
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

    /// Check if the symbol can win the game on the square (i, j)
    /// 
    /// Ignores non-nil cells (so it's safe to blindly call it without checks)
    pub fn can_win(&self, symbol: Symbol, i: u32, j: u32) -> bool {
        // check for Nil
        if self.get(i, j) != Symbol::Nil {
            return false;
        }

        // check row
        if self.board[i as usize]
            .iter()
            .filter(|x| **x == symbol)
            .count()
            == 2
        {
            return true;
        }

        // check column
        if self
            .board
            .iter()
            .map(|row| row[j as usize])
            .filter(|x| *x == symbol)
            .count()
            == 2
        {
            return true;
        }

        // check diag. 1 (/)
        if i == j {
            if (0..3)
                .map(|x| self.get(x, x))
                .filter(|x| *x == symbol)
                .count()
                == 2
            {
                return true;
            }
        }

        // check diag. 2 (\)
        if i + j == 2 {
            if (0..3)
                .map(|x| self.get(x, 2 - x))
                .filter(|x| *x == symbol)
                .count()
                == 2
            {
                return true;
            }
        }

        return false;
    }

    /// Get a copy of a given cell
    pub fn get(&self, i: u32, j: u32) -> Symbol {
        return self.board[i as usize][j as usize];
    }

    /// Set a cell
    /// 
    /// WARNING! It blindly overrides, so check for Nil before calling!
    pub fn set(&mut self, i: u32, j: u32, s: Symbol) {
        self.board[i as usize][j as usize] = s;
    }
}
