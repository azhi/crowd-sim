use ::std::fmt::Debug;

use ::simulation::forces::Force;
use ::simulation::forces::Forceable;

use ::simulation::person::Person;
use ::simulation::scene::Scene;

use ::configuration::DistributionValue;
use ::utils::linelg::distance::DistanceTo;
use ::utils::linelg::Vector;
use ::utils::linelg::Point;

#[derive(Debug)]
pub struct RepulsionForce;

impl RepulsionForce {
    fn repulsion_from_obstacle<T: Debug>(&self, person: &Person, obstacle: &T, scene_scale: f64) -> Vector where Point: DistanceTo<T> {
        // some magic numbers
        const DISTANCE_SQR_THRESHOLD: f64 = 25_f64;
        const REPULSION_ELLIPSE_R_X: f64 = 1.5_f64;
        const REPULSION_ELLIPSE_R_Y: f64 = 5_f64;

        let nearest_point = person.coordinates.nearest_point(obstacle);
        let direction = nearest_point - person.coordinates;
        let direction_length_sqr_in_meters = direction.length_sqr() * scene_scale;
        if direction_length_sqr_in_meters < DISTANCE_SQR_THRESHOLD && direction.length_sqr() != 0.0 {
            let angle = direction.y.atan2(direction.x);
            let ellipse_coeff = self.ellipse_sqr_radius_at_angle(REPULSION_ELLIPSE_R_X, REPULSION_ELLIPSE_R_Y, angle);
            - direction.normalized() / (direction_length_sqr_in_meters * 3_f64) * ellipse_coeff
        } else {
            Vector::zero()
        }
    }

    fn ellipse_sqr_radius_at_angle(&self, r_x: f64, r_y: f64, angle: f64) -> f64 {
        (r_x * r_y).powi(2) / (r_x.powi(2) * angle.sin().powi(2) + r_y.powi(2) * angle.cos().powi(2))
    }
}

impl Forceable for RepulsionForce {
    fn force_for_person(&self, person: &Person, scene: &Scene) -> Vector {
        let repulsion_coeff = person.forces_params.repulsion_coeff;
        let mut force = Vector::zero();
        for obstacle in scene.geometry.iter() {
            force = force + self.repulsion_from_obstacle(&person, obstacle, scene.scale);
        }
        for other_person in scene.people.iter() {
            force = force + self.repulsion_from_obstacle(&person, &other_person.coordinates, scene.scale);
        }
        force = force * repulsion_coeff;
        force
    }
}
