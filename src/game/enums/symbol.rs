/// Represent the different states a square can be
#[derive(PartialEq, Copy, Clone)]
pub enum Symbol {
    X,
    O,
    Nil,
}

use rand::Rng;

impl Symbol {
    /// Convert the `Symbol` to a `String`
    pub fn to_str(&self) -> String {
        return match self {
            Symbol::X => String::from("X"),
            Symbol::O => String::from("O"),
            Symbol::Nil => String::from(" "),
        };
    }

    /// Return the other Symbol
    /// 
    /// Return `X` for `O`, and `O` for `X`
    /// 
    /// Returns `Nil` for `Nil`
    pub fn other(s: Symbol) -> Symbol {
        return match s {
            Symbol::X => Symbol::O,
            Symbol::O => Symbol::X,
            Symbol::Nil => Symbol::Nil,
        };
    }

    /// Return a random (`X` or `O`) Symbol
    pub fn random() -> Symbol {
        return match rand::thread_rng().gen_range(0, 2) {
            0 => Symbol::O,
            1 => Symbol::X,
            // this case should never be reached
            _ => panic!("Invalid random player symbol number!"),
        };
    }
}
