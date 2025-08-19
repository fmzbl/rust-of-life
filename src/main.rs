use std::{thread, time::Duration};

const GRID_SIZE: usize = 40;
const CLEAR_STR: &str = "\x1B[2J\x1B[H";
const ALIVE_STR: &str = "■ ";
const DEAD_STR: &str = "□ ";

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

struct Game {
    grid: Vec<Vec<bool>>,
}

impl Game {
    fn new() -> Game {
        let grid = vec![vec![false; GRID_SIZE]; GRID_SIZE];
        Game { grid }
    }

    fn seed(&mut self) {
        for row in self.grid.iter_mut() {
            for cell in row.iter_mut() {
                let random_state = rand::random_bool(0.1);
                *cell = random_state;
            }
        }
    }

    fn display(&self) {
        let mut grid_string = String::new();

        for row in self.grid.iter() {
            for cell in row.iter() {
                match cell {
                    true => {
                        grid_string.push_str(ALIVE_STR);
                    }
                    false => {
                        grid_string.push_str(DEAD_STR);
                    }
                }
            }
            grid_string.push('\n');
        }

        print!("{grid_string}");
    }

    fn tick(&mut self) {
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
}

fn main() {
    let mut game = Game::new();
    game.seed();
    loop {
        print!("{CLEAR_STR}");
        game.display();
        game.tick();
        thread::sleep(Duration::from_millis(500));
    }
}
