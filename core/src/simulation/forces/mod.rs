extern crate anymap;

mod repulsion;
mod target;
mod fluctuation;

use std::ops::Deref;

macro_rules! trait_enum {
    (enum $name:ident : $_trait:ident { $($var:ident($ty:ty)),* }) => {
        #[derive(Debug)]
        pub enum $name {
            $(
                $var($ty),
            )*
        }

        impl<'a> Deref for $name {
            type Target = ($_trait + 'a);
            fn deref<'b>(&'b self) -> &'b $_trait {
                match self {
                    $(& $name::$var(ref x) => x,)*
                }
            }
        }
    }
}

use self::anymap::AnyMap;

use self::repulsion::RepulsionForce;
use self::target::TargetForce;
use self::fluctuation::FluctuationForce;

use ::simulation::person::Person;
use ::simulation::scene::Scene;

use ::configuration::DistributionValue;
use ::utils::linelg::Vector;

pub trait Forceable {
    fn force_for_person(&self, person: &Person, scene: &Scene) -> Vector;
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
}

trait_enum! {
    enum Force : Forceable {
        Target(TargetForce),
        Repulsion(RepulsionForce),
        Fluctuation(FluctuationForce)
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
            Force::Fluctuation(FluctuationForce),
        ];
        Forces{ used_forces: used_forces, target_speed: target_speed, repulsion_coeff: repulsion_coeff,
                forward_fov: forward_fov, backward_fov: backward_fov }
    }

    pub fn total_force_for_person(&self, person: &Person, scene: &Scene) -> Vector {
        let mut total_force = Vector::zero();
        for force in self.used_forces.iter() {
            total_force = total_force + force.deref().force_for_person(person, scene);
        }
        total_force
    }

    pub fn generate_person_forces_param(&self) -> PersonForcesParams {
        let mut res = PersonForcesParams{
            target_speed: ::utils::distributions::generate(&self.target_speed),
            repulsion_coeff: ::utils::distributions::generate(&self.repulsion_coeff),
            forward_fov: ::utils::distributions::generate(&self.forward_fov),
            backward_fov: ::utils::distributions::generate(&self.backward_fov),
        };
        res.target_speed = res.target_speed.max(0.1);
        res.repulsion_coeff = res.repulsion_coeff.max(0.01);
        res.forward_fov = res.forward_fov.max(0.01);
        res.backward_fov = res.backward_fov.max(0.01);
        res
    }
}
