use std::f64;
use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector { x: x, y: y }
    }

    pub fn zero() -> Vector {
        Vector { x: 0f64, y: 0f64 }
    }

    pub fn length_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_sqr())
    }

    pub fn normalized(&self) -> Vector {
        let len = self.length();
        Vector { x: self.x / len, y: self.y / len }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        Vector { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector { x: self * rhs.x, y: self * rhs.y }
    }
}

impl Div for Vector {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Vector {
        Vector { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        Vector { x: self.x / rhs, y: self.y / rhs }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector { x: -self.x, y: -self.y }
    }
}

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Invalid index into vector"),
        }
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Invalid index into vector"),
        }
    }
}

#[test]
fn test_len_sqr() {
    let v = Vector::new(1f64, 2f64);
    assert!(v.length_sqr() == 1f64 + 4f64);
}

#[test]
fn test_idx() {
    let mut v = Vector::new(1f64, 2f64);
    assert!(v[0] == 1f64 && v[1] == 2f64);
    {
        let x = &mut v[1];
        *x = 5f64;
    }
    assert!(v[1] == 5f64);
}

