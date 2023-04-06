use rand::Rng;

pub const WIDTH: usize = 209;
pub const HEIGHT: usize = 55;

#[derive(Copy,Clone,PartialEq,Default)]
pub enum State {
    Alive,
    #[default]
    Dead,
}

impl State {
    fn get_char(&self) -> &str {
        match self {
            State::Alive => "X",
            State::Dead => " ",
        }
    }
}

pub struct Grid(pub [[State; WIDTH]; HEIGHT]);

impl Grid {
    pub fn new() -> Grid {
        Grid([[State::default(); WIDTH]; HEIGHT])
    }

    pub fn generate_next_state(&self, buffer: &mut Grid) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let mut neighbors = 0;
                for dx in -1isize..2 {
                    for dy in -1isize..2 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let neighbor_x = (x as isize) + dx;
                        let neighbor_y = (y as isize) + dy;
                        if (neighbor_x as usize) < WIDTH && neighbor_x >= 0 && 
                                (neighbor_y as usize) < HEIGHT && neighbor_y >= 0 && 
                                self.0[neighbor_y as usize][neighbor_x as usize] == State::Alive {
                            neighbors += 1;
                        }
                    }
                }
                buffer.0[y][x] = match neighbors {
                    0 => State::Dead,
                    1 => State::Dead,
                    2 => self.0[y][x],
                    3 => State::Alive,
                    4 => State::Dead,
                    5 => State::Dead,
                    6 => State::Dead,
                    7 => State::Dead,
                    8 => State::Dead,
                    _ => self.0[y][x]
                }
                
            }
        }
    }

    pub fn print(&self, generation: i32) {
        println!("Generation: {}", generation);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", self.0[y][x].get_char());
            }
            println!();
        }
    }
    
    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let roll = rng.gen_range(0..2);
                self.0[y][x] = match roll {
                    0 => State::Alive,
                    _ => State::Dead
                }
            }
        }
    }
}