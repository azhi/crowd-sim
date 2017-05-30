use utils::linelg::Line;
use utils::linelg::Point;

pub fn line_intersection(line1: &Line, line2: &Line) -> Option<Point> {
    let discr = (line1.from.x - line1.to.x) * (line2.from.y - line2.to.y) -
                (line1.from.y - line1.to.y) * (line2.from.x - line2.to.x);
    if discr != 0_f64 {
        let x = ((line1.from.x * line1.to.y - line1.from.y * line1.to.x) * (line2.from.x - line2.to.x) -
                 (line2.from.x * line2.to.y - line2.from.y * line2.to.x) * (line1.from.x - line1.to.x)) / discr;
        let y = ((line1.from.x * line1.to.y - line1.from.y * line1.to.x) * (line2.from.y - line2.to.y) -
                 (line2.from.x * line2.to.y - line2.from.y * line2.to.x) * (line1.from.y - line1.to.y)) / discr;
        Some(Point{x: x, y: y})
    } else {
        None
    }
}
