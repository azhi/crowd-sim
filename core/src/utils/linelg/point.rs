use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};
use utils::linelg::Vector;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    pub fn zero() -> Point {
        Point { x: 0f64, y: 0f64 }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        Vector { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Point {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point { x: self * rhs.x, y: self * rhs.y }
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, rhs: Point) -> Point {
        Point { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Point {
        Point { x: self.x / rhs, y: self.y / rhs }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point { x: -self.x, y: -self.y }
    }
}

impl Index<usize> for Point {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Invalid index into point"),
        }
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Invalid index into point"),
        }
    }
}
