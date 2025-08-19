use std::{thread, time::Duration};

const GRID_SIZE: usize = 60;
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

    fn seed_glider_gun(&mut self) {
        // Gosper Glider Gun pattern coordinates
        let glider_gun: &[(usize, usize)] = &[
            (1, 25), (2, 23), (2, 25), (3, 13), (3, 14), (3, 21), (3, 22), (3, 35), (3, 36),
            (4, 12), (4, 16), (4, 21), (4, 22), (4, 35), (4, 36), (5, 1), (5, 2), (5, 11),
            (5, 17), (5, 21), (5, 22), (6, 1), (6, 2), (6, 11), (6, 15), (6, 17), (6, 18),
            (6, 23), (6, 25), (7, 11), (7, 17), (7, 25), (8, 12), (8, 16), (9, 13), (9, 14),
        ];

        for row in self.grid.iter_mut() {
            for cell in row.iter_mut() {
                *cell = false;
            }
        }

        for &(y, x) in glider_gun {
            if y < GRID_SIZE && x < GRID_SIZE {
                self.grid[y][x] = true;
            }
        }
    }

    fn seed_random(&mut self) {
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
    game.seed_glider_gun();
    loop {
        print!("{CLEAR_STR}");
        game.display();
        game.tick();
        thread::sleep(Duration::from_millis(300));
    }
}
