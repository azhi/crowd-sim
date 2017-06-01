extern crate anymap;

mod repulsion;
mod target;
mod fluctuation;
mod herding;

use self::anymap::AnyMap;

use self::repulsion::RepulsionForce;
use self::target::TargetForce;
use self::fluctuation::FluctuationForce;
use self::herding::HerdingForce;

use ::simulation::person::Person;
use ::simulation::scene::Scene;
use ::simulation::scene::Area;

use ::configuration::DistributionValue;
use ::utils::linelg::Vector;

pub trait Forceable {
    fn force_for_person(&mut self, person: &Person, scene: &Scene) -> Vector;
}

pub struct Forces {
    used_forces: Vec<Force>,
    target_speed: DistributionValue,
    repulsion_coeff: DistributionValue,
    forward_fov: DistributionValue,
    backward_fov: DistributionValue,
}

#[derive(Debug,Clone)]
pub struct PersonForcesParams {
    pub target_speed: f64,
    pub repulsion_coeff: f64,
    pub forward_fov: f64,
    pub backward_fov: f64,
    pub herding: f64,
}

#[derive(Debug)]
pub enum Force {
    Target(TargetForce),
    Repulsion(RepulsionForce),
    Fluctuation(FluctuationForce),
    Herding(HerdingForce),
}

impl Forceable for Force {
    fn force_for_person(&mut self, person: &Person, scene: &Scene) -> Vector {
        match self {
            &mut Force::Target(ref mut force) => force.force_for_person(person, scene),
            &mut Force::Repulsion(ref mut force) => force.force_for_person(person, scene),
            &mut Force::Fluctuation(ref mut force) => force.force_for_person(person, scene),
            &mut Force::Herding(ref mut force) => force.force_for_person(person, scene),
        }
    }
}


impl Forces {
    pub fn new(configuration: &AnyMap) -> Forces {
        let target_speed = config!(configuration, ForcesTargetSpeed);
        let repulsion_coeff = config!(configuration, ForcesRepulsionCoeff);
        let forward_fov = config!(configuration, FovForward);
        let backward_fov = config!(configuration, FovBackward);

        let used_forces = vec![
            Force::Target(TargetForce),
            Force::Repulsion(RepulsionForce),
            Force::Fluctuation(FluctuationForce::new()),
            Force::Herding(HerdingForce::new()),
        ];
        Forces{ used_forces: used_forces, target_speed: target_speed, repulsion_coeff: repulsion_coeff,
                forward_fov: forward_fov, backward_fov: backward_fov }
    }

    pub fn total_force_for_person(&mut self, person: &Person, scene: &Scene) -> (Vector, Option<(u8, Area, u16)>) {
        let mut total_force = Vector::zero();

        let mut path_change_option: Option<(u8, Area, u16)> = None;

        for force in self.used_forces.iter_mut() {
            total_force = total_force + force.force_for_person(person, scene);
            if let &mut Force::Herding(ref herding_force) = force {
                path_change_option = herding_force.path_change.clone();
            };
        }

        let force_power = total_force.length().min(person.forces_params.target_speed * 1.2);
        if force_power != 0_f64 {
            total_force = total_force.normalized() * force_power;
        }

        (total_force, path_change_option)
    }

    pub fn generate_person_forces_param(&self) -> PersonForcesParams {
        let mut res = PersonForcesParams{
            target_speed: ::utils::distributions::generate(&self.target_speed),
            repulsion_coeff: ::utils::distributions::generate(&self.repulsion_coeff),
            forward_fov: ::utils::distributions::generate(&self.forward_fov),
            backward_fov: ::utils::distributions::generate(&self.backward_fov),
            herding: ::utils::distributions::generate_normal(0.1, 0.1),
        };
        res.target_speed = res.target_speed.max(0.1);
        res.repulsion_coeff = res.repulsion_coeff.max(0.01);
        res.forward_fov = res.forward_fov.max(0.01);
        res.backward_fov = res.backward_fov.max(0.01);
        res.herding = res.herding.max(0.05);
        res
    }
}
