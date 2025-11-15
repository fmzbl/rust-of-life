use macroquad::prelude::*;

use crate::{GRID_SIZE, game_editor::PatternCoords};

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

type Grid = Vec<Vec<Cell>>;

#[derive(Debug, Clone)]
pub struct Cell {
    pub alive: bool,
    pub chaotic: bool,
}

#[derive(Debug)]
pub struct GameGrid {
    grid: Grid,
}

impl GameGrid {
    pub fn new() -> GameGrid {
        let grid = vec![vec![Cell {alive: false, chaotic: false}; GRID_SIZE]; GRID_SIZE];
        GameGrid { grid }
    }

    pub fn apply_pattern(&mut self, coords: PatternCoords, start_x: usize, start_y: usize, chaotic: bool) {
        for &(y, x) in coords {
            if y + start_y < GRID_SIZE && x + start_x < GRID_SIZE {
                self.grid[y + start_y][x + start_x] = Cell {alive: true, chaotic};
            }
        }
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        self.grid[y][x] = Cell{
	    alive: self.grid[y][x].alive,
	    chaotic: self.grid[y][x].chaotic,
	}
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
                                Some(grid_copy[y][x].alive)
                            }
                            (_, _) => None,
                        }
                    })
                    .filter(|&n| n)
                    .count();

                self.grid[cell_y][cell_x].alive = match (grid_copy[cell_y][cell_x].alive, alive_neighbors) {
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
