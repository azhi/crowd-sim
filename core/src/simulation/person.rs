use ::utils::linelg::Vector;
use ::utils::linelg::Point;

use ::simulation::forces::PersonForcesParams;
use ::simulation::scene::Path;
use ::simulation::scene::Area;

#[derive(Debug,Clone)]
pub struct Person {
    pub id: u64,
    pub coordinates: Point,
    pub heading: f64,
    pub path_id: u8,
    pub current_target_area: Area,
    pub current_target_index: u16,
    pub panic_level: f64,
    pub forces_params: PersonForcesParams,
}

impl Person {
    #[allow(non_snake_case)]
    pub fn move_by(&mut self, total_force: Vector, t: f64) {
        let MAX_HEADING_DIFF_TO_TARGET: f64 = 180_f64.to_radians();
        let INSTANT_HEADING_CHANGE_THRESHOLD: f64 = 45_f64.to_radians();
        let TURN_RATE: f64 = 10_f64.to_radians();

        let target_vector = self.current_target_point() - self.coordinates;
        let target_heading = ::utils::headings::vector_heading(target_vector);

        let mut new_heading = ::utils::headings::vector_heading(total_force);
        let heading_change = ::utils::headings::heading_diff(new_heading, self.heading);

        let adjusted_total_force = if heading_change.abs() > INSTANT_HEADING_CHANGE_THRESHOLD {
            new_heading = if heading_change > 0_f64 {
                self.heading + TURN_RATE
            } else {
                self.heading - TURN_RATE
            };
            if new_heading < 0_f64 {
                new_heading += 2_f64 * ::std::f64::consts::PI;
            }

            let mut weaken_coeff = (heading_change.abs() - INSTANT_HEADING_CHANGE_THRESHOLD).cos();
            let heading_diff_to_target = ::utils::headings::heading_diff(target_heading, new_heading);
            if heading_diff_to_target.abs() > MAX_HEADING_DIFF_TO_TARGET {
                // cap max rotation at MAX_HEADING_DIFF_TO_TARGET
                new_heading = heading_diff_to_target.signum() * MAX_HEADING_DIFF_TO_TARGET;
                let chop_diff = heading_diff_to_target.abs() - MAX_HEADING_DIFF_TO_TARGET;
                weaken_coeff = weaken_coeff * chop_diff.cos() / 1000_f64;
            }
            Vector::new(new_heading.cos(), new_heading.sin()) * (total_force.length() * weaken_coeff).min(4_f64)
        } else {
            total_force
        };
        self.coordinates = self.coordinates + adjusted_total_force * t;
        self.heading = new_heading;
    }

    pub fn current_target_point(&self) -> Point {
        self.current_target_area.nearest_point(&self.coordinates)
    }

    pub fn fov_coeff(&self, source: Point) -> f64 {
        const SIDE_FOV: f64 = 2_f64;

        let direction = source - self.coordinates;
        let source_heading = ::utils::headings::vector_heading(direction);
        let heading_diff = ::utils::headings::heading_diff(source_heading, self.heading);
        let ellipse_coeff = if heading_diff.abs() < ::std::f64::consts::PI / 2_f64 {
            ::utils::linelg::ellipse_sqr_radius_at_angle(SIDE_FOV, self.forces_params.forward_fov, heading_diff).sqrt()
        } else {
            ::utils::linelg::ellipse_sqr_radius_at_angle(SIDE_FOV, self.forces_params.backward_fov, heading_diff).sqrt()
        };
        let normalization_coeff = SIDE_FOV.max(self.forces_params.forward_fov).max(self.forces_params.backward_fov);
        let fov_coeff = ellipse_coeff / normalization_coeff / 2_f64;
        fov_coeff
    }

    pub fn reached_destination(&self, path: &Path) -> bool {
        let target = &path.target_areas[self.current_target_index as usize];
        let person_radius = ::simulation::scene::APPROX_PERSON_RADIUS;
        self.coordinates.x + person_radius > target.p0.x && self.coordinates.x - person_radius < target.p1.x &&
            self.coordinates.y + person_radius > target.p0.y && self.coordinates.y - person_radius < target.p1.y
    }
}
