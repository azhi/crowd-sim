use ::simulation::forces::Forceable;

use ::simulation::person::Person;
use ::simulation::scene::Scene;
use ::simulation::scene::Area;

use ::utils::linelg::Vector;

#[derive(Debug)]
pub struct HerdingForce {
    pub path_change: Option<(u8, Area, u16)>,
}

impl HerdingForce {
    pub fn new() -> HerdingForce {
        HerdingForce{path_change: None}
    }
}

impl Forceable for HerdingForce {
    fn force_for_person(&mut self, person: &Person, scene: &Scene) -> Vector {
        const HERDING_FOV_DISTANCE: f64 = 3f64;

        self.path_change = None;
        let mut force = Vector::zero();
        let mut n = 0;
        let mut some_person: Option<Person> = None;

        if person.panic_level != 0_f64 {
            for other_person in scene.people.iter() {
                if (person.coordinates - other_person.coordinates).length() * scene.scale < HERDING_FOV_DISTANCE {
                    force = force + Vector::new(other_person.heading.cos(), other_person.heading.sin()) * other_person.forces_params.target_speed;

                    n += 1;
                    some_person = Some(other_person.clone())
                }
            }
        }

        if n != 0 {
            force = force / (n as f64);
        }

        force = force * person.panic_level * person.forces_params.herding;
        if force.length() > 0.7_f64 * person.forces_params.target_speed {
            match some_person {
                Some(other_person) => self.path_change = Some((other_person.path_id, other_person.current_target_area.clone(), other_person.current_target_index)),
                None => panic!("A large force out of nowhere?")
            }
        }
        force
    }
}
