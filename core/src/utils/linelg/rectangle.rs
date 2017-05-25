use std::f64;
use utils::linelg::Point;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub p4: Point,
}

impl Rectangle {
    pub fn new(p1: Point, p2: Point, p3: Point, p4: Point) -> Rectangle {
        Rectangle { p1: p1, p2: p2, p3: p3, p4: p4 }
    }

    pub fn new_from_raw(x0 : f64, y0 : f64, x1 : f64, y1 : f64) -> Rectangle {
        let p1 = Point{ x: x0, y: y0 };
        let p2 = Point{ x: x0, y: y1 };
        let p3 = Point{ x: x1, y: y1 };
        let p4 = Point{ x: x1, y: y0 };
        Rectangle::new(p1, p2, p3, p4)
    }
}
