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
        let mut new_heading = total_force.y.atan2(total_force.x);
        let mut heading_change = new_heading - self.heading;
        let adjusted_total_force = if heading_change.abs() > INSTANT_HEADING_CHANGE_THRESHOLD {
            new_heading = if heading_change > 0_f64 {
                self.heading + INSTANT_HEADING_CHANGE_THRESHOLD
            } else {
                self.heading - INSTANT_HEADING_CHANGE_THRESHOLD
            };
            Vector::new(
                new_heading.cos(),
                new_heading.sin()
            ) * total_force.length()
        } else {
            total_force
        };
        self.coordinates = self.coordinates + adjusted_total_force * t;
        self.heading = new_heading;
    }

    pub fn reached_destination(&self, path: &Path) -> bool {
        let target = &path.target_areas[self.current_target_index as usize];
        self.coordinates.x > target.p0.x && self.coordinates.x < target.p1.x &&
            self.coordinates.y > target.p0.y && self.coordinates.y < target.p1.y
    }
}
