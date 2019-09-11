use eight_puzzle::Game;

fn main() {
    env_logger::init();

    let mut game = Game::new();

    while let Some(()) = game.next() {}
}
