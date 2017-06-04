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
        self.calculate_forces_and_move();
        self.calculate_panic_level();

        match self.sim_type {
            SimType::Escape => (),
            _ => self.scene.spawn_people(&self.forces, self.time.tick)
        }
        let reached_destination_people = self.scene.process_reached_destination_people();
        self.statistics.update_from_reached_destination_people(reached_destination_people, self.time.current_time);
    }

    fn calculate_forces_and_move(&mut self) {
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
    }

    fn calculate_panic_level(&mut self) {
        const K_INITIAL_PANIC: f64 = 1.0_f64;
        const K_SPREAD_PANIC: f64 = 1.0_f64;
        const K_DECAY_PANIC: f64 = 0.01_f64;
        const PANIC_FOV_DISTANCE: f64 = 4f64;

        let mut panic_levels = Vec::new();
        panic_levels.reserve(self.scene.people.len());
        for person in self.scene.people.iter() {
            let mut panic_from_sources = 0_f64;
            let mut n = 0;
            for panic_source in self.scene.panic_sources.iter() {
                if (panic_source.coordinates - person.coordinates).length() < panic_source.radius {
                    panic_from_sources += panic_source.power;
                    n += 1;
                }
            }
            if n != 0 {
                panic_from_sources = K_INITIAL_PANIC * panic_from_sources / (n as f64);
            }

            let mut panic_from_herding = 0_f64;
            n = 0;
            for other_person in self.scene.people.iter() {
                if (person.coordinates - other_person.coordinates).length() * self.scene.scale < PANIC_FOV_DISTANCE {
                    panic_from_herding += other_person.panic_level;
                    n += 1;
                }
            }
            if n != 0 {
                panic_from_herding = K_SPREAD_PANIC * panic_from_herding / (n as f64);
            }

            let desired_panic_level = (panic_from_sources + panic_from_herding).max(0_f64).min(1_f64);

            let new_panic_level = if desired_panic_level >= person.panic_level {
                desired_panic_level
            } else {
                person.panic_level - K_DECAY_PANIC * (person.panic_level - desired_panic_level)
            };
            panic_levels.push(new_panic_level);
        };

        for (person, panic_level) in self.scene.people.iter_mut().zip(panic_levels.iter()) {
            person.panic_level = *panic_level;
        }
    }
}

