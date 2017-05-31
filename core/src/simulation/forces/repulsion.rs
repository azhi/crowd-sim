use ::std::fmt::Debug;

use ::simulation::forces::Forceable;

use ::simulation::person::Person;
use ::simulation::scene::Scene;

use ::utils::linelg::distance::DistanceTo;
use ::utils::linelg::Vector;
use ::utils::linelg::Point;
use ::utils::linelg::Line;

pub trait RepulsionPoint<T> {
    fn repulsion_point(&self, other: &T) -> Option<Point>;
}

impl RepulsionPoint<Line> for Person {
    fn repulsion_point(&self, line: &Line) -> Option<Point> {
        let nearest_point = self.coordinates.nearest_point(line);
        if nearest_point != line.from && nearest_point != line.to {
            Some(nearest_point)
        } else {
            Some(nearest_point)
            // let heading_vector = Vector::new(self.heading.cos(), self.heading.sin());
            // let heading_line = Line::new(self.coordinates, self.coordinates + heading_vector);
            // match ::utils::linelg::intersection::line_intersection(&heading_line, line) {
            //     Some(intersection) => {
            //         if intersection.distance(&line.from).min(intersection.distance(&line.to)) < 1_f64 &&
            //            intersection.distance(&self.coordinates) < 2_f64 {
            //             Some(nearest_point)
            //         } else {
            //             None
            //         }
            //     }
            //     None => None
            // }
        }
    }
}

impl RepulsionPoint<Point> for Person {
    fn repulsion_point(&self, other: &Point) -> Option<Point> {
        Some(self.coordinates.nearest_point(other))
    }
}

#[derive(Debug)]
pub struct RepulsionForce;

impl RepulsionForce {
    fn repulsion_from_obstacle<T: Debug>(&self, person: &Person, obstacle: &T, scene_scale: f64) -> Vector where Person: RepulsionPoint<T> {
        // some magic numbers
        const DISTANCE_SQR_THRESHOLD: f64 = 25_f64;

        const ANISOTROPIC_K: f64 = 0.2_f64;
        const REPULSION_DISTANCE: f64 = 0.7_f64;
        const REPULSION_AT_ZERO: f64 = 2_f64;

        match person.repulsion_point(obstacle) {
            Some(repulsion_point) => {
                let direction = repulsion_point - person.coordinates;
                let direction_length_sqr_in_meters = direction.length_sqr() * scene_scale;
                if direction_length_sqr_in_meters < DISTANCE_SQR_THRESHOLD && direction.length_sqr() != 0.0 {
                    let distance_coeff = REPULSION_AT_ZERO * ((::simulation::scene::APPROX_PERSON_RADIUS - direction_length_sqr_in_meters.sqrt()) / REPULSION_DISTANCE).exp();

                    let heading_vector = Vector::new(person.heading.cos(), person.heading.sin());
                    let cos_phi = direction.normalized().dot_product(&heading_vector);
                    let anisotropic_coeff = ANISOTROPIC_K + (1_f64 - ANISOTROPIC_K) * (1_f64 + cos_phi) / 2_f64;

                    let fov_coeff = person.fov_coeff(repulsion_point);
                    let ret = - direction.normalized() * distance_coeff * anisotropic_coeff;
                    ret
                } else {
                    Vector::zero()
                }
            }
            None => {
                Vector::zero()
            }
        }
    }

}

impl Forceable for RepulsionForce {
    fn force_for_person(&mut self, person: &Person, scene: &Scene) -> Vector {
        let repulsion_coeff = person.forces_params.repulsion_coeff;
        let mut force = Vector::zero();
        for obstacle in scene.geometry.iter() {
            force = force + self.repulsion_from_obstacle(&person, obstacle, scene.scale);
        }
        for other_person in scene.people.iter() {
            force = force + self.repulsion_from_obstacle(&person, &other_person.coordinates, scene.scale);
        }
        let force_power = force.length().min(person.forces_params.target_speed * 2_f64);
        if force_power != 0_f64 {
            force = force.normalized() * force_power;
        }
        force = force * repulsion_coeff;
        force
    }
}
