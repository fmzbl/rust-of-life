use macroquad::prelude::*;
use std::{thread, time::Duration};

use crate::game_editor::PatternCoords;
use crate::{game_editor::GameEditor, game_grid::GameGrid};

use crate::GRID_SIZE;

const CELL_SIZE: f32 = 20.0; // Size of each cell in pixels

#[derive(Debug, Default)]
enum GameState {
    Running,
    #[default]
    Editing,
}

pub struct Game {
    game_grid: GameGrid,
    game_state: GameState,
    game_editor: GameEditor,
    chaos: bool,
}

impl Game {
    pub fn new() -> Game {
        let game_grid = GameGrid::new();
        let game_state = GameState::default();
        let game_editor = GameEditor::new();

        Game {
            game_grid,
            game_state,
            game_editor,
	    chaos: false,
        }
    }

    pub fn tick(&mut self) {
        match self.game_state {
            GameState::Running => {
                self.game_grid.apply_rules();

		if self.chaos {
		    self.apply_random();
		}

                thread::sleep(Duration::from_millis(100));
            }
            GameState::Editing => {
                self.handle_editing();
            }
        }

        self.apply_input_rules();
    }

    pub fn apply_random(&mut self) {

        let patterns = self.game_editor.get_editor_patterns();
        for pattern in patterns.iter() {
            let roll = rand::gen_range(1, 6) == 1;
            if roll {
                let n1 = rand::gen_range(1, GRID_SIZE);
                let n2 = rand::gen_range(1, GRID_SIZE);
                self.game_grid.apply_pattern(pattern.coords(), n1, n2);
            }
        }

        // cuantos, cuales, donde,
    }

    pub fn draw(&mut self) {
        // grid
        clear_background(BLACK);
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let color = if self.game_grid.get_ref()[y][x] {
                    WHITE
                } else {
                    DARKGRAY
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

        draw_text("click; click; space", 400.0, 20.0, 20.0, WHITE);

        // Draw status
        draw_text(
            match self.game_state {
                GameState::Running => {if self.chaos {"Running (chaos)"} else {"Running"}},
                GameState::Editing => "Editing",
            },
            220.0,
            20.0,
            20.0,
            GREEN,
        );

        if matches!(self.game_state, GameState::Editing) {
            if let Some(pattern) = self.game_editor.pattern_selected() {
                draw_text(
                    &format!("(escape) Selected: {}", pattern.name()),
                    220.0,
                    40.0,
                    20.0,
                    GREEN,
                );
            }
        }

        for (i, pattern) in self.game_editor.patterns().iter().enumerate() {
            let text = i.to_string() + ": " + pattern.name();
            draw_text(&text, 20.0, 20.0 * (i + 1) as f32, 20.0, PURPLE);
        }
    }

    fn handle_editing(&mut self) {
        let patterns_id: Vec<usize> = self.game_editor.patterns().iter().map(|p| p.id()).collect();

        for pattern_id in patterns_id {
            let key_code_option = match pattern_id {
                0 => Some(KeyCode::Key0),
                1 => Some(KeyCode::Key1),
                2 => Some(KeyCode::Key2),
                3 => Some(KeyCode::Key3),
                4 => Some(KeyCode::Key4),
                5 => Some(KeyCode::Key5),
                6 => Some(KeyCode::Key6),
                7 => Some(KeyCode::Key7),
                8 => Some(KeyCode::Key8),
                9 => Some(KeyCode::Key9),
                _ => None,
            };
            if let Some(key_code) = key_code_option {
                if is_key_pressed(key_code) {
                    let _ = self.game_editor.select_pattern(pattern_id);
                }
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            self.game_editor.unselect_pattern();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let grid_x = (mouse_x / CELL_SIZE) as usize;
            let grid_y = (mouse_y / CELL_SIZE) as usize;

            if let Some(pattern) = self.game_editor.pattern_selected() {
                // apply pattern
                self.game_grid
                    .apply_pattern(pattern.coords(), grid_x, grid_y);
            } else {
                // toogle cell
                if grid_x < GRID_SIZE && grid_y < GRID_SIZE {
                    self.game_grid.toggle_cell(grid_x, grid_y);
                }
            }
        }
    }

    fn apply_input_rules(&mut self) {
        match self.game_state {
            GameState::Running => {
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

	if is_key_pressed(KeyCode::C) {
	    self.chaos = !self.chaos;
	}
    }
}
