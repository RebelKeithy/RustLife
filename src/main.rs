use std::thread;
use std::time;

mod utils {
    pub mod points;
    pub mod rotation;
    pub mod grid;
}

use utils::rotation::Rotation;
use utils::grid::{Grid,WIDTH,HEIGHT};

mod patterns {
    pub mod pattern;
    pub mod library;
}

use patterns::library::{r_pentomino,glider};

fn main() {
    let mut grid = Grid::new();
    let mut buffer = Grid::new();

    // grid.randomize();

    let glider = glider();
    for i in 0..40 {
        glider.add_to_grid(&mut grid, i * 5, 0, Rotation::None);
    }
    r_pentomino().add_to_grid(&mut grid, (WIDTH/2) as i32, (HEIGHT/2) as i32, Rotation::CCW);
    
    let mut generation = 0;
    loop {
        generation += 1;
        grid.print(generation);
        grid.generate_next_state(&mut buffer);
        std::mem::swap(&mut grid, &mut buffer);
        thread::sleep(time::Duration::from_millis(100))
    }
}