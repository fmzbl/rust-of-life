use std::{thread, time::Duration};

mod game;

use game::Game;
use macroquad::window::next_frame;

#[macroquad::main("Game of Life")]
async fn main() {
    let mut game = Game::new();
    game.seed_glider_gun();
    loop {
        game.draw();
        game.tick();
        thread::sleep(Duration::from_millis(100));
	next_frame().await
    }
}
