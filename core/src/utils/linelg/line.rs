use std::f64;
use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};
use utils::linelg::Point;
use utils::linelg::distance::DistanceTo;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line {
    pub from: Point,
    pub to: Point,
}

impl Line {
    pub fn new(from: Point, to: Point) -> Line {
        Line { from: from, to: to }
    }

    pub fn new_from_raw(x0 : f64, y0 : f64, x1 : f64, y1 : f64) -> Line {
        let from = Point{ x: x0, y: y0 };
        let to = Point{ x: x1, y: y1 };
        Line::new(from, to)
    }

    pub fn length_sqr(&self) -> f64 {
        self.to.distance_sqr(&self.from)
    }

    pub fn length(&self) -> f64 {
        self.to.distance(&self.from)
    }
}

impl Neg for Line {
    type Output = Line;

    fn neg(self) -> Line {
        Line { from: self.to, to: self.from }
    }
}

#[test]
fn test_len_sqr() {
    let l = Line::new_from_raw(0f64, 0f64, 4f64, 0f64);
    assert!(l.length_sqr() == 16f64);
}
