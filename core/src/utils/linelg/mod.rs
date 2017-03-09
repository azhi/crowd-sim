#![allow(dead_code)]
use std::ops::Index;

// Re-export the linalg types from the internal modules
pub use self::vector::Vector;
pub use self::point::Point;
pub use self::line::Line;

pub mod vector;
pub mod point;
pub mod line;
pub mod distance;

static EPS : f64 = 0.00001f64;

pub fn dot<A: Index<usize, Output = f64>, B: Index<usize, Output = f64>>(a: &A, b: &B) -> f64 {
    a[0] * b[0] + a[1] * b[1]
}

pub fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
    if x < min { min } else if x > max { max } else { x }
}

pub fn ellipse_sqr_radius_at_angle(r_x: f64, r_y: f64, angle: f64) -> f64 {
    (r_x * r_y).powi(2) / (r_x.powi(2) * angle.sin().powi(2) + r_y.powi(2) * angle.cos().powi(2))
}

#[test]
fn test_dot() {
    let a = Vector::new(1f64, 2f64);
    let b = Vector::new(4f64, 5f64);
    assert!(dot(&a, &b) == 1f64 * 4f64 + 2f64 * 5f64);
}
