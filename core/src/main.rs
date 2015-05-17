#![feature(convert)]

#[macro_use] extern crate log;
extern crate env_logger;

#[macro_use] mod configuration;
mod simulation;
mod output;
mod utils;

use std::io;

fn main() {
    env_logger::init().unwrap();

    // let mut file = File::open(config_filename).ok().expect("Can't open provided config file!");
    let configuration = configuration::new(&mut std::io::stdin());
    let mut simulation = simulation::Simulation::new(configuration);
    simulation.main_loop();
}
