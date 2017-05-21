use utils::linelg::vector::Vector;

pub fn vector_heading(v: Vector) -> f64 {
    let mut heading = v.y.atan2(v.x);
    if heading < 0_f64 {
        heading += 2_f64 * ::std::f64::consts::PI;
    }
    return heading;
}

pub fn heading_diff(h1: f64, h2: f64) -> f64 {
    let mut diff = h1 - h2;
    if diff > ::std::f64::consts::PI {
        diff = - 2_f64 * ::std::f64::consts::PI + diff;
    } else if diff < - ::std::f64::consts::PI {
        diff = 2_f64 * ::std::f64::consts::PI + diff;
    }
    return diff;
}
