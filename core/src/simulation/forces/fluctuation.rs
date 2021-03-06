use ::simulation::forces::Forceable;

use ::simulation::person::Person;
use ::simulation::scene::Scene;

use ::utils::linelg::Vector;

#[derive(Debug)]
pub struct FluctuationForce;

impl Forceable for FluctuationForce {
    fn force_for_person(&self, _person: &Person, _scene: &Scene) -> Vector {
        let direction = Vector::new(::utils::distributions::generate_uniform(0.0, 1.0),
                                    ::utils::distributions::generate_uniform(0.0, 1.0));
        let power = ::utils::distributions::generate_uniform(0.0, 0.1);
        let force = direction.normalized() * power;
        force
    }
}
