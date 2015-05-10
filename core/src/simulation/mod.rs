extern crate anymap;

mod forces;
mod scene;
mod time;

use self::anymap::AnyMap;

use forces::Force;
use scene::Scene;
use time::Time;

use ::output::Output;

struct Simulation {
    configuration: AnyMap,
    forces: Vec<Force>,
    scene: Scene,
    time: Time,
}

impl Simulation {
    fn new(configuration: AnyMap) {
        let scene = Scene::new(configuration);
        let time = Time::new(configuration);
        let forces = Force::used_forces(configuration);
        Simulation{ configuration: configuration, forces: forces, scene: scene, time: time }
    }

    fn main_loop(&self) {
        let output = Output::new(self.configuration);
        while !self.time.is_passed() {
            self.update_state();
            output.dump_state(self);
            self.time.next_tick();
        }
    }

    fn update_state(&self) {
        for person in self.scene.people.iter() {
        }
    }
}

