use ::utils::linelg::Vector;
use ::utils::linelg::Point;

use ::simulation::forces::PersonForcesParams;
use ::simulation::scene::Path;

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
        self.coordinates = self.coordinates + total_force * t;
        self.heading = total_force.y.atan2(total_force.x);
    }

    pub fn reached_destination(&self, path: &Path) -> bool {
        let target = &path.target_areas[self.current_target_index as usize];
        self.coordinates.x > target.p0.x && self.coordinates.x < target.p1.x &&
            self.coordinates.y > target.p0.y && self.coordinates.y < target.p1.y
    }
}
