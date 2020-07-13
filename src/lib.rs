mod game {
    mod ai;
    mod board;
    mod enums;
    mod game;

    pub use game::Game;
}

use game::Game;
pub fn run_game() {
    let mut game = Game::new();
    game.run();
}
