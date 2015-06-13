use ::utils::linelg::Vector;
use ::utils::linelg::Point;

use ::simulation::forces::PersonForcesParams;
use ::simulation::scene::Path;

#[derive(Debug)]
pub struct Person {
    pub coordinates: Point,
    pub heading: f64,
    pub path_id: u8,
    pub current_target_point: Point,
    pub current_target_index: u16,
    pub forces_params: PersonForcesParams,
}

impl Person {
    pub fn move_by(&mut self, total_force: Vector, t: f64) {
        let INSTANT_HEADING_CHANGE_THRESHOLD: f64 = 10_f64.to_radians();
        let TURN_RATE: f64 = 5_f64.to_radians();

        let mut new_heading = total_force.y.atan2(total_force.x);
        if new_heading < 0_f64 {
            new_heading += 2_f64 * ::std::f64::consts::PI;
        }
        let mut heading_change = new_heading - self.heading;
        if heading_change > ::std::f64::consts::PI {
            heading_change = - 2_f64 * ::std::f64::consts::PI + heading_change;
        } else if heading_change < - ::std::f64::consts::PI {
            heading_change = 2_f64 * ::std::f64::consts::PI + heading_change;
        }
        let adjusted_total_force = if heading_change.abs() > INSTANT_HEADING_CHANGE_THRESHOLD {
            new_heading = if heading_change > 0_f64 {
                self.heading + TURN_RATE
            } else {
                self.heading - TURN_RATE
            };
            if new_heading < 0_f64 {
                new_heading += 2_f64 * ::std::f64::consts::PI;
            }
            Vector::new(
                new_heading.cos(),
                new_heading.sin()
            ) * total_force.length().min(4_f64)
        } else {
            total_force
        };
        self.coordinates = self.coordinates + adjusted_total_force * t;
        self.heading = new_heading;
    }

    pub fn fov_coeff(&self, source: Point) -> f64 {
        const SIDE_FOV: f64 = 2_f64;

        let direction = source - self.coordinates;
        let angle = direction.y.atan2(direction.x);
        let ellipse_coeff = if angle > 0_f64 {
            ::utils::linelg::ellipse_sqr_radius_at_angle(SIDE_FOV, self.forces_params.forward_fov, angle).sqrt()
        } else {
            ::utils::linelg::ellipse_sqr_radius_at_angle(SIDE_FOV, self.forces_params.backward_fov, -angle).sqrt()
        };
        let normalization_coeff = SIDE_FOV.max(self.forces_params.forward_fov).max(self.forces_params.backward_fov);
        let fov_coeff = ellipse_coeff / normalization_coeff / 2_f64;
        fov_coeff
    }

    pub fn reached_destination(&self, path: &Path) -> bool {
        let target = &path.target_areas[self.current_target_index as usize];
        self.coordinates.x > target.p0.x && self.coordinates.x < target.p1.x &&
            self.coordinates.y > target.p0.y && self.coordinates.y < target.p1.y
    }
}
