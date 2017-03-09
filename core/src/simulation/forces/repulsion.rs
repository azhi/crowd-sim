use ::std::fmt::Debug;

use ::simulation::forces::Forceable;

use ::simulation::person::Person;
use ::simulation::scene::Scene;

use ::utils::linelg::distance::DistanceTo;
use ::utils::linelg::Vector;
use ::utils::linelg::Point;

#[derive(Debug)]
pub struct RepulsionForce;

impl RepulsionForce {
    fn repulsion_from_obstacle<T: Debug>(&self, person: &Person, obstacle: &T, scene_scale: f64) -> Vector where Point: DistanceTo<T> {
        // some magic numbers
        const DISTANCE_SQR_THRESHOLD: f64 = 125_f64;
        const REPULSION_ELLIPSE_R_X: f64 = 1.0_f64;
        const REPULSION_ELLIPSE_R_Y: f64 = 2.0_f64;

        let nearest_point = person.coordinates.nearest_point(obstacle);
        let direction = nearest_point - person.coordinates;
        let direction_length_sqr_in_meters = direction.length_sqr() * scene_scale;
        if direction_length_sqr_in_meters < DISTANCE_SQR_THRESHOLD && direction.length_sqr() != 0.0 {
            let angle = direction.y.atan2(direction.x);
            let ellipse_coeff = ::utils::linelg::ellipse_sqr_radius_at_angle(REPULSION_ELLIPSE_R_X, REPULSION_ELLIPSE_R_Y, angle);
            // let distance_coeff = 1_f64 / ((direction_length_sqr_in_meters.sqrt() - ::simulation::scene::APPROX_PERSON_RADIUS) * 5_f64);
            let distance_coeff = (- 1_f64 / 2.5_f64 * (direction_length_sqr_in_meters.sqrt() - ::simulation::scene::APPROX_PERSON_RADIUS) + 3_f64).max(0_f64).min(3_f64);
            let fov_coeff = person.fov_coeff(nearest_point);
            - direction.normalized() * distance_coeff * fov_coeff * ellipse_coeff.sqrt()
        } else {
            Vector::zero()
        }
    }

}

impl Forceable for RepulsionForce {
    fn force_for_person(&self, person: &Person, scene: &Scene) -> Vector {
        let repulsion_coeff = person.forces_params.repulsion_coeff;
        let mut force = Vector::zero();
        for obstacle in scene.geometry.iter() {
            force = force + 2_f64 * self.repulsion_from_obstacle(&person, obstacle, scene.scale);
        }
        for other_person in scene.people.iter() {
            force = force + self.repulsion_from_obstacle(&person, &other_person.coordinates, scene.scale);
        }
        let force_power = force.length().min(4_f64);
        if force_power != 0_f64 {
            force = force.normalized() * force_power;
        }
        force = force * repulsion_coeff;
        force
    }
}
