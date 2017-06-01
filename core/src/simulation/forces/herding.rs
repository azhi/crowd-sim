use ::simulation::forces::Forceable;

use ::simulation::person::Person;
use ::simulation::scene::Scene;

use ::utils::linelg::Vector;

#[derive(Debug)]
pub struct HerdingForce;

impl Forceable for HerdingForce {
    fn force_for_person(&mut self, person: &Person, scene: &Scene) -> Vector {
        const HERDING_FOV_DISTANCE: f64 = 3f64;

        let mut force = Vector::zero();
        let mut n = 0;

        if person.panic_level != 0_f64 {
            for other_person in scene.people.iter() {
                if (person.coordinates - other_person.coordinates).length() * scene.scale < HERDING_FOV_DISTANCE {
                    force = force + Vector::new(other_person.heading.cos(), other_person.heading.sin()) * other_person.forces_params.target_speed;

                    n += 1;
                }
            }
        }

        if n != 0 {
            force = force / (n as f64);
        }

        force = force * person.panic_level * person.forces_params.herding;
        force
    }
}
