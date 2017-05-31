use std::collections::HashMap;

use ::simulation::forces::Forceable;

use ::simulation::person::Person;
use ::simulation::scene::Scene;

use ::utils::linelg::Vector;

#[derive(Debug)]
struct Fluctuation {
    pub force: Vector,
    pub ticks_left: u32,
}

impl Fluctuation {
    pub fn new(person: &Person) -> Fluctuation {
        const MAX_POWER: f64 = 1_f64;
        const MAX_DURATION: u32 = 10_u32;

        let direction = Vector::new(::utils::distributions::generate_uniform(0.0, 1.0),
                                    ::utils::distributions::generate_uniform(0.0, 1.0));
        let power = ::utils::distributions::generate_uniform(0.0, MAX_POWER) * person.panic_level;
        let force = direction.normalized() * power;

        let mut duration = (::utils::distributions::generate_uniform(0.0, MAX_DURATION as f64) * person.panic_level * 10.0).round() as u32;
        duration = std::cmp::max(duration, 1);

        Fluctuation{
            force: force,
            ticks_left: duration,
        }
    }
}

#[derive(Debug)]
pub struct FluctuationForce {
    active_fluctuations: HashMap<u64, Fluctuation>
}

impl FluctuationForce {
    pub fn new() -> FluctuationForce {
        FluctuationForce{active_fluctuations: HashMap::new()}
    }
}

impl Forceable for FluctuationForce {
    fn force_for_person(&mut self, person: &Person, _scene: &Scene) -> Vector {
        const MAX_EMERGE_PROBABILITY: f64 = 0.001_f64;

        let (mut force, create_new_fluctuation, remove_fluctuation) = match self.active_fluctuations.get_mut(&person.id) {
            Some(fluctuation) => {
                fluctuation.ticks_left -= 1;
                (fluctuation.force.clone(), false, fluctuation.ticks_left == 0)
            }
            None => {
                if ::utils::distributions::generate_uniform(0.0, 1.0) < person.panic_level * MAX_EMERGE_PROBABILITY {
                    (Vector::zero(), true, false)
                } else {
                    (Vector::zero(), false, false)
                }
            }
        };

        if create_new_fluctuation {
            let fluctuation = Fluctuation::new(person);
            force = fluctuation.force.clone();
            self.active_fluctuations.insert(person.id, fluctuation);
        };

        if remove_fluctuation {
            match self.active_fluctuations.remove(&person.id) {
                Some(_fluctuation) => {}
                None => panic!("Trying to remove fluctuation, but it's not there!")
            }
        };


        force
    }
}
