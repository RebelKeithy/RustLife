use crate::utils::points::Point;
use crate::utils::rotation::Rotation;
use crate::utils::grid::Grid;
use crate::utils::grid::State;
use std::cmp;

pub struct Pattern {
    points: Vec<Point>,
    corner_min: Point,
    corner_max: Point
}

impl Pattern {
    pub fn new() -> Pattern {
        Pattern {
            points: vec![],
            corner_max: Point::new(),
            corner_min: Point::new()
        }
    }

    pub fn from_vec(points: Vec<Point>) -> Pattern {
        let mut corner_min = Point::new();
        let mut corner_max = Point::new();
        for point in &points {
            corner_min.x = cmp::min(corner_min.x, point.x);
            corner_min.y = cmp::min(corner_min.y, point.y);
            corner_max.x = cmp::max(corner_max.x, point.x);
            corner_max.y = cmp::max(corner_max.y, point.y);
        }

        Pattern {
            points,
            corner_max,
            corner_min
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.corner_min.x = cmp::min(self.corner_min.x, point.x);
        self.corner_min.y = cmp::min(self.corner_min.y, point.y);
        self.corner_max.x = cmp::max(self.corner_max.x, point.x);
        self.corner_max.y = cmp::max(self.corner_max.y, point.y);
        self.points.push(point);
    }

    pub fn add_to_grid(&self, grid: &mut Grid, x_offset: i32, y_offset: i32, rotation: Rotation) {
        let origin = (self.corner_max.clone() - self.corner_min.clone())/2;
        for point in &self.points {
            let rotated_point = point.rotate_around(&rotation, &origin);
            grid.0[(rotated_point.y + y_offset) as usize][(rotated_point.x + x_offset) as usize] = State::Alive;
        }
    }
}