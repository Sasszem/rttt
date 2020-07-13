mod game {
    mod ai;
    mod enums;
    mod board;
    mod game;

    pub use game::Game;
}

use game::Game;
pub fn run_game() {
    let mut game = Game::new();
    game.run();
}