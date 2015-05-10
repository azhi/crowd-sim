use utils::linelg::Point;
use utils::linelg::Line;

pub trait DistanceTo<T> {
    fn nearest_point(&self, other: &T) -> Point;
    fn distance_sqr(&self, other: &T) -> f64;
    fn distance(&self, other: &T) -> f64;
}

impl DistanceTo<Point> for Point {
    fn nearest_point(&self, other: &Point) -> Point {
        other.clone()
    }

    fn distance_sqr(&self, a: &Point) -> f64 {
        (*self - *a).length_sqr()
    }

    fn distance(&self, a: &Point) -> f64 {
        (*self - *a).length()
    }
}

impl DistanceTo<Line> for Point {
    fn nearest_point(&self, line: &Line) -> Point {
        let line_length = line.length_sqr();
        if line_length == 0f64 {
            line.from.clone()
        } else {
            let t = ::utils::linelg::dot(&(*self - line.from), &(line.to - line.from)) / line_length;
            if t < 0f64 {
                line.from.clone()
            } else if t > 1f64 {
                line.to.clone()
            } else {
                line.from + t * (line.to - line.from)
            }
        }
    }

    fn distance_sqr(&self, line: &Line) -> f64 {
        let nearest_point = self.nearest_point(line);
        self.distance_sqr(&nearest_point)
    }

    fn distance(&self, line: &Line) -> f64 {
        let nearest_point = self.nearest_point(line);
        self.distance(&nearest_point)
    }
}

#[test]
fn test_distance_points_sqr() {
    let a = Point::new(0f64, 0f64);
    let b = Point::new(3f64, 4f64);
    assert!(b.distance_sqr(&a) == 25f64);
}

#[test]
fn test_distance_from_point_to_vertical_line() {
    let p = Point::new(1f64, 0f64);
    let l = Line::new_from_raw(3f64, -4f64, 3f64, 4f64);
    let nearest_point = p.nearest_point(&l);
    assert!((nearest_point.x - 3f64).abs() < ::utils::linelg::EPS);
    assert!((nearest_point.y - 0f64).abs() < ::utils::linelg::EPS);
    assert!((p.distance_sqr(&l) - 4f64).abs() < ::utils::linelg::EPS);
    assert!((p.distance(&l) - 2f64).abs() < ::utils::linelg::EPS);
}

#[test]
fn test_distance_from_point_to_horizontal_line() {
    let p = Point::new(0f64, 4f64);
    let l = Line::new_from_raw(-4f64, 5f64, 4f64, 5f64);
    let nearest_point = p.nearest_point(&l);
    assert!((nearest_point.x - 0f64).abs() < ::utils::linelg::EPS);
    assert!((nearest_point.y - 5f64).abs() < ::utils::linelg::EPS);
    assert!((p.distance_sqr(&l) - 1f64).abs() < ::utils::linelg::EPS);
    assert!((p.distance(&l) - 1f64).abs() < ::utils::linelg::EPS);
}
