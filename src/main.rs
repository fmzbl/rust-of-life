use std::{thread, time::Duration};

pub mod game;
pub mod game_grid;

use game::Game;
use macroquad::window::next_frame;

pub const GRID_SIZE: usize = 80;

#[macroquad::main("Game of Life")]
async fn main() {
    let mut game = Game::new();
    loop {
        game.draw();
        game.tick();
        thread::sleep(Duration::from_millis(100));
	next_frame().await
    }
}
