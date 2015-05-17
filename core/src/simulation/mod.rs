extern crate anymap;
extern crate time as system_time;

pub mod person;
pub mod scene;
mod forces;
mod time;

use self::anymap::AnyMap;

use self::forces::Forces;
use self::scene::Scene;
use self::time::Time;

use ::output::Output;

pub struct Simulation {
    configuration: AnyMap,
    forces: Forces,
    pub scene: Scene,
    pub time: Time,
}

impl Simulation {
    pub fn new(configuration: AnyMap) -> Simulation {
        let time = Time::new(&configuration);
        let forces = Forces::new(&configuration);
        let scene = Scene::new(&configuration);
        Simulation{ configuration: configuration, forces: forces, scene: scene, time: time }
    }

    pub fn main_loop(&mut self) {
        info!("Starting main simulation loop");
        let output = Output::new(&self.configuration);
        debug!("Sending init message to output");
        output.send_init();
        let mut avg_tick : f64 = 0.0_f64;
        while !self.time.is_passed() {
            let t1 = system_time::precise_time_ns();
            self.update_state();
            output.dump_state(self);
            self.time.next_tick();
            let t2 = system_time::precise_time_ns();
            avg_tick += (t2 - t1) as f64;
        }
        avg_tick = avg_tick / self.time.end_time * self.time.tick;
        info!("Avg tick took {} ns", avg_tick.round());
        info!("Simulation done.");
    }

    fn update_state(&mut self) {
        let mut total_forces_for_person = Vec::new();
        total_forces_for_person.reserve(self.scene.people.len());
        for person in self.scene.people.iter() {
            let total_force = self.forces.total_force_for_person(person, &self.scene);
            total_forces_for_person.push(total_force);
        }
        for (person, total_force) in self.scene.people.iter_mut().zip(total_forces_for_person.iter()) {
            if total_force.length() < 0.01_f64 {
                warn!("Small total force: {}", total_force.length());
            }
            person.move_by(*total_force, self.time.tick);
        }

        self.scene.spawn_people(&self.forces, self.time.tick);
        self.scene.process_reached_destination_people();
    }
}

