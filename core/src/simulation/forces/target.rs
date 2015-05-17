use ::simulation::forces::Force;
use ::simulation::forces::Forceable;

use ::simulation::person::Person;
use ::simulation::scene::Scene;

use ::configuration::DistributionValue;
use ::utils::linelg::Vector;

pub struct TargetForce;

impl Forceable for TargetForce {
    fn force_for_person(&self, person: &Person, scene: &Scene) -> Vector {
        let target_speed = person.forces_params.target_speed;
        let direction = person.current_target_point - person.coordinates;
        let force = direction.normalized() * target_speed;
        force
    }
}
