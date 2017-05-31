use ::std::fmt::Debug;

use ::simulation::forces::Forceable;

use ::simulation::person::Person;
use ::simulation::scene::Scene;

use ::utils::linelg::distance::DistanceTo;
use ::utils::linelg::Vector;
use ::utils::linelg::Point;
use ::utils::linelg::Line;

#[derive(Debug)]
pub struct RepulsionForce;

impl RepulsionForce {
    fn repulsion_from_obstacle<T: Debug>(&self, person: &Person, obstacle: &T, panic_coeff: f64, scene_scale: f64) -> Vector where Point: DistanceTo<T> {
        // some magic numbers
        const DISTANCE_THRESHOLD: f64 = 5_f64;

        const ANISOTROPIC_K: f64 = 0.2_f64;
        const REPULSION_DISTANCE: f64 = 0.2_f64;
        const REPULSION_AT_ZERO: f64 = 0.9_f64;

        let repulsion_point = person.coordinates.nearest_point(obstacle);
        let direction = repulsion_point - person.coordinates;
        let distance = direction.length() * scene_scale;
        if distance < DISTANCE_THRESHOLD && direction.length_sqr() != 0.0 {
            let distance_coeff = REPULSION_AT_ZERO * ((::simulation::scene::APPROX_PERSON_RADIUS - distance) / REPULSION_DISTANCE).exp();

            let heading_vector = Vector::new(person.heading.cos(), person.heading.sin());
            let cos_phi = direction.normalized().dot_product(&heading_vector);
            let anisotropic_coeff = ANISOTROPIC_K + (1_f64 - ANISOTROPIC_K) * (1_f64 + cos_phi) / 2_f64;

            let fov_coeff = person.fov_coeff(repulsion_point);
            - direction.normalized() * distance_coeff * anisotropic_coeff * panic_coeff
        } else {
            Vector::zero()
        }
    }

    fn physical_repulsion(&self, person: &Person, other_person: &Person, scene_scale: f64) -> Vector {
        Vector::zero()
    }
}

impl Forceable for RepulsionForce {
    fn force_for_person(&mut self, person: &Person, scene: &Scene) -> Vector {
        let repulsion_coeff = person.forces_params.repulsion_coeff;
        let mut force = Vector::zero();
        for obstacle in scene.geometry.iter() {
            force = force + self.repulsion_from_obstacle(&person, obstacle, 1.0_f64, scene.scale);
        }
        for other_person in scene.people.iter() {
            force = force + self.repulsion_from_obstacle(&person, &other_person.coordinates, 1.0_f64 - person.panic_level, scene.scale);
            force = force + self.physical_repulsion(&person, &other_person, scene.scale);
        }
        let force_power = force.length().min(person.forces_params.target_speed * 2_f64);
        if force_power != 0_f64 {
            force = force.normalized() * force_power;
        }
        force = force * repulsion_coeff;
        force
    }
}
