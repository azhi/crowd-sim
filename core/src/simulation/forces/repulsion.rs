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
    fn repulsion_from_obstacle<T: Debug>(&self, person: &Person, obstacle: &T, panic_coeff: f64, scene_scale: f64) -> Vector where Point: DistanceTo<T> {
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

    fn geom_physical_repulsion<T: Debug>(&self, person: &Person, obstacle: &T, scene_scale: f64) -> Vector where Point: DistanceTo<T> {
        const K_PHYS: f64 = 5_f64;

        let repulsion_point = person.coordinates.nearest_point(obstacle);
        let direction = repulsion_point - person.coordinates;
        let distance = direction.length() * scene_scale;
        if distance < ::simulation::scene::APPROX_PERSON_RADIUS {
            let distance_coeff = ::simulation::scene::APPROX_PERSON_RADIUS - distance;

            let phys_repulsion = - direction.normalized() * K_PHYS * distance_coeff;

            // let v = Vector::new(person.heading.cos(), person.heading.sin()) * person.forces_params.target_speed;
            // let normal = - direction.normalized();
            // let tangent_direction = Vector::new(-normal.y, normal.x);
            // let tangent_coeff = v.dot_product(&tangent_direction);
            // let tangent_repulsion = tangent_direction * K_TANGENT * distance_coeff * tangent_coeff;

            let force = phys_repulsion;
            // debug!("Physical repulsion for person #{} {:?} (from person #{} {:?}): {:?}", person.id, person.coordinates, other_person.id, other_person.coordinates, force);
            // ; Vector::zero()
            force
        } else {
            Vector::zero()
        }
    }

    fn person_physical_repulsion(&self, person: &Person, other_person: &Person, scene_scale: f64) -> Vector {
        const K_PHYS: f64 = 5_f64;
        const K_TANGENT: f64 = 5_f64;

        let repulsion_point = person.coordinates.nearest_point(&other_person.coordinates);
        let direction = repulsion_point - person.coordinates;
        let distance = direction.length() * scene_scale;
        if distance < 2_f64 * ::simulation::scene::APPROX_PERSON_RADIUS {
            let distance_coeff = 2_f64 * ::simulation::scene::APPROX_PERSON_RADIUS - distance;

            let phys_repulsion = - direction.normalized() * K_PHYS * distance_coeff;

            let v = Vector::new(person.heading.cos(), person.heading.sin()) * person.forces_params.target_speed;
            let other_v = Vector::new(other_person.heading.cos(), other_person.heading.sin()) * other_person.forces_params.target_speed;
            let normal = - direction.normalized();
            let tangent_direction = Vector::new(-normal.y, normal.x);
            let tangent_coeff = (other_v - v).dot_product(&tangent_direction);
            let tangent_repulsion = tangent_direction * K_TANGENT * distance_coeff * tangent_coeff;

            let force = tangent_repulsion + phys_repulsion;
            // debug!("Physical repulsion for person #{} {:?} (from person #{} {:?}): {:?}", person.id, person.coordinates, other_person.id, other_person.coordinates, force);
            // ; Vector::zero()
            force
        } else {
            Vector::zero()
        }
    }

}

impl Forceable for RepulsionForce {
    fn force_for_person(&mut self, person: &Person, scene: &Scene) -> Vector {
        let repulsion_coeff = person.forces_params.repulsion_coeff;
        let mut force = Vector::zero();
        for obstacle in scene.geometry.iter() {
            force = force + self.repulsion_from_obstacle(&person, obstacle, 1.0_f64, scene.scale);
            force = force + self.geom_physical_repulsion(&person, obstacle, scene.scale);
        }
        for other_person in scene.people.iter() {
            if other_person.id != person.id {
                force = force + self.repulsion_from_obstacle(&person, &other_person.coordinates, 1.0_f64 - person.panic_level, scene.scale);
                force = force + self.person_physical_repulsion(&person, &other_person, scene.scale);
            }
        }
        let force_power = force.length().min(person.forces_params.target_speed * 2_f64);
        if force_power != 0_f64 {
            force = force.normalized() * force_power;
        }
        force = force * repulsion_coeff;
        force
    }
}
