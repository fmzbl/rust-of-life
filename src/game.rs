use macroquad::prelude::*;

use crate::game_grid::GameGrid;
use macroquad::prelude::*;

use crate::GRID_SIZE;

const CELL_SIZE: f32 = 10.0; // Size of each cell in pixels

#[derive(Debug, Default)]
enum GameState {
    Running,
    #[default]
    Editing,
}

pub struct Game {
    game_grid: GameGrid,
    game_state: GameState,
}

impl Game {
    pub fn new() -> Game {
        let mut game_grid = GameGrid::new();
        game_grid.seed_glider_gun();

        let game_state = GameState::default();

        Game {
            game_grid,
            game_state,
        }
    }

    pub fn tick(&mut self) {

        match self.game_state {
            GameState::Running => {
                self.game_grid.apply_rules();
            }
            GameState::Editing => {
		self.handle_editing();
	    }
        }

        self.apply_input_rules();
	dbg!(&self.game_state);
    }

    pub fn draw(&self) {
        clear_background(BLACK);
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let color = if self.game_grid.get_ref()[y][x] {
                    WHITE
                } else {
                    GRAY
                };
                draw_rectangle(
                    x as f32 * CELL_SIZE,
                    y as f32 * CELL_SIZE,
                    CELL_SIZE - 1.0, // -1 to avoid gaps
                    CELL_SIZE - 1.0,
                    color,
                );
            }
        }
    }

    fn handle_editing(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            let grid_x = (mouse_x / CELL_SIZE) as usize;
            let grid_y = (mouse_y / CELL_SIZE) as usize;

            if grid_x < GRID_SIZE && grid_y < GRID_SIZE {
		self.game_grid.toggle_cell(grid_x, grid_y);
            }
        }
    }

    fn apply_input_rules(&mut self) {
        match self.game_state {
            GameState::Running => {
                self.game_grid.apply_rules();
                if is_key_pressed(KeyCode::Space) {
                    self.game_state = GameState::Editing;
                }
            }
            GameState::Editing => {
                if is_key_pressed(KeyCode::Space) {
                    self.game_state = GameState::Running;
                }
            }
        }
    }
}
