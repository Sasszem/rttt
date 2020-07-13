#[derive(PartialEq, Copy, Clone)]
pub enum Symbol {
    X,
    O,
    Nil,
}

use rand::Rng;

impl Symbol {
    pub fn to_str(&self) -> String {
        return match self {
            Symbol::X => String::from("X"),
            Symbol::O => String::from("O"),
            Symbol::Nil => String::from(" "),
        };
    }

    pub fn other(s: Symbol) -> Symbol {
        return match s {
            Symbol::X => Symbol::O,
            Symbol::O => Symbol::X,
            Symbol::Nil => Symbol::Nil,
        };
    }

    pub fn random() -> Symbol {
        return match rand::thread_rng().gen_range(0, 2) {
            0 => Symbol::O,
            1 => Symbol::X,
            _ => panic!("Invalid random player symbol number!"),
        };
    }
}
