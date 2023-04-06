use super::pattern::Pattern;
use crate::utils::points::Point;

pub fn r_pentomino() -> Pattern {
    Pattern::from_vec(vec![
        Point { x: 1, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 1, y: 1 },
        Point { x: 1, y: 2 }
    ])
}

pub fn glider() -> Pattern {
    Pattern::from_vec(vec![
        Point { x: 1, y: 3},
        Point { x: 2, y: 3},
        Point { x: 3, y: 3},
        Point { x: 3, y: 2},
        Point { x: 2, y: 1},
    ])
} 