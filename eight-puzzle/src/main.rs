use eight_puzzle::Game;

fn main() {
    let mut game = Game::new();

    while let Some(()) = game.next() {}
}
