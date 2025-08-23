use macroquad::prelude::*;

use crate::{game_editor::PatternCoords, GRID_SIZE};

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

type Grid = Vec<Vec<bool>>;

pub struct GameGrid {
    grid: Grid,
}

impl GameGrid {
    pub fn new() -> GameGrid {
        let grid = vec![vec![false; GRID_SIZE]; GRID_SIZE];
        GameGrid { grid }
    }

    pub fn apply_pattern(&mut self, coords: PatternCoords, start_x: usize, start_y: usize) {
        for &(y, x) in coords {
            if y < GRID_SIZE && x < GRID_SIZE {
                self.grid[y + start_y][x + start_x] = true;
            }
        }
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        self.grid[y][x] = !self.grid[y][x]
    }

    pub fn apply_rules(&mut self) {
        let grid_copy = self.grid.clone();

        for cell_y in 0..grid_copy.len() {
            for cell_x in 0..grid_copy.len() {
                let alive_neighbors = NEIGHBORS
                    .iter()
                    .filter_map(|n| {
                        let y = cell_y as i32 + n.0;
                        let x = cell_x as i32 + n.1;

                        let x = usize::try_from(x);
                        let y = usize::try_from(y);

                        // guards against out of bounds and failed conversions of cords
                        match (x, y) {
                            (Ok(x), Ok(y)) if y < grid_copy.len() && x < grid_copy[y].len() => {
                                Some(grid_copy[y][x])
                            }
                            (_, _) => None,
                        }
                    })
                    .filter(|&n| n)
                    .count();

                self.grid[cell_y][cell_x] = match (grid_copy[cell_y][cell_x], alive_neighbors) {
                    (true, 2 | 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
    }

    pub fn get_ref(&self) -> &Grid {
        return &self.grid;
    }
}

impl Default for GameGrid {
    fn default() -> Self {
        Self::new()
    }
}
