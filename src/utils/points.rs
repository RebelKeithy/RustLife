use std::ops::{Add,Mul,Sub,Div};
use super::rotation::Rotation;

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn rotate_around(&self, rotation: &Rotation, origin: &Point) -> Self {
        origin.clone() + ((self.clone() - origin.clone()) * rotation.clone())
    }
}

impl Mul<Rotation> for Point {
    type Output = Self;

    fn mul(self, rotation: Rotation) -> Self::Output {
        match rotation {
            Rotation::None => self,
            Rotation::Half => Point { x: -self.x, y: -self.y },
            Rotation::CCW => Point { x: self.y, y: -self.x },
            Rotation::CW => Point { x: -self.y, y: self.x },
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<i32> for Point {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}