extern crate anymap;

mod repulsion;
mod target;

use std::ops::Deref;

macro_rules! trait_enum {
    (enum $name:ident : $_trait:ident { $($var:ident($ty:ty)),* }) => {
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
}

#[derive(Debug,Clone)]
pub struct PersonForcesParams {
    pub target_speed: f64,
    pub repulsion_coeff: f64,
}

trait_enum! {
    enum Force : Forceable {
        Target(TargetForce),
        Repulsion(RepulsionForce)
    }
}


impl Forces {
    pub fn new(configuration: &AnyMap) -> Forces {
        let target_speed = config!(configuration, ForcesTargetSpeed);
        let repulsion_coeff = config!(configuration, ForcesRepulsionCoeff);

        let used_forces = vec![
            Force::Target(TargetForce),
            Force::Repulsion(RepulsionForce)
        ];
        Forces{ used_forces: used_forces, target_speed: target_speed, repulsion_coeff: repulsion_coeff }
    }

    pub fn total_force_for_person(&self, person: &Person, scene: &Scene) -> Vector {
        let mut total_force = Vector::zero();
        for force in self.used_forces.iter() {
            total_force = total_force + force.deref().force_for_person(person, scene);
        }
        total_force
    }

    pub fn generate_person_forces_param(&self) -> PersonForcesParams {
        PersonForcesParams{
            target_speed: ::utils::distributions::generate(&self.target_speed),
            repulsion_coeff: ::utils::distributions::generate(&self.repulsion_coeff),
        }
    }
}
