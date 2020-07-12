fn main() {
    let mut game = Game::new();
    game.run();
}
mod symbol;
mod ai;
mod board;
mod gameresult;
mod game;
use crate::game::Game;


// todo: remove debug print
// todo: do not allow overwrites