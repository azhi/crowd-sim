extern crate anymap;
extern crate time as system_time;

pub mod person;
pub mod scene;
mod forces;
mod time;
mod statistics;

use self::anymap::AnyMap;

use self::forces::Forces;
use self::scene::Scene;
use self::time::Time;
use self::statistics::Statistics;

use ::output::Output;
use ::configuration::SimType;

pub struct Simulation {
    configuration: AnyMap,
    sim_type: SimType,
    forces: Forces,
    pub statistics: Statistics,
    pub scene: Scene,
    pub time: Time,
}

impl Simulation {
    pub fn new(configuration: AnyMap) -> Simulation {
        let sim_type = config!(configuration, SimTypeCfgWrap);
        let time = Time::new(&configuration);
        let forces = Forces::new(&configuration);
        let scene = Scene::new(&configuration);
        let statistics = Statistics::new(&configuration);
        Simulation{ sim_type: sim_type, configuration: configuration, statistics: statistics, forces: forces, scene: scene, time: time }
    }

    pub fn main_loop(&mut self) {
        info!("Starting main simulation loop");
        let mut output = Output::new(&self.configuration);
        debug!("Sending init message to output");
        output.send_init();

        match self.sim_type {
            SimType::Escape => {
                info!("Simulation is in Escape mode, doing initial spawn ...");
                self.scene.spawn_people(&self.forces, self.time.tick);
                debug!("Spawned {} people", self.scene.people.len());
            },
            _ => ()
        }

        let mut sum_running_time : f64 = 0.0_f64;

        while !self.is_simulation_finished() {
            let t1 = system_time::precise_time_ns();
            self.update_state();
            output.dump_state(self);
            self.time.next_tick();
            let t2 = system_time::precise_time_ns();
            sum_running_time += (t2 - t1) as f64;
        }

        output.dump_statistics(self);
        let avg_tick = sum_running_time / self.time.current_time * self.time.tick;
        info!("Avg tick took {} ns", avg_tick.round());
        info!("Simulation done.");
    }

    fn is_simulation_finished(&self) -> bool {
        return match self.sim_type {
            SimType::Flow => self.time.is_passed(),
            SimType::Escape => self.scene.people.len() == 0
        }
    }



    fn update_state(&mut self) {
        let mut total_forces_for_person = Vec::new();
        total_forces_for_person.reserve(self.scene.people.len());
        for person in self.scene.people.iter() {
            let mut total_force = self.forces.total_force_for_person(person, &self.scene);
            total_force = total_force / self.scene.scale;
            total_forces_for_person.push(total_force);
        }
        for (person, total_force) in self.scene.people.iter_mut().zip(total_forces_for_person.iter()) {
            if total_force.length() < 0.01_f64 {
                warn!("Small total force: {}", total_force.length());
            }
            person.move_by(*total_force, self.time.tick);
        }

        match self.sim_type {
            SimType::Escape => (),
            _ => self.scene.spawn_people(&self.forces, self.time.tick)
        }
        let reached_destination_people = self.scene.process_reached_destination_people();
        self.statistics.update_from_reached_destination_people(reached_destination_people, self.time.current_time);
    }
}

