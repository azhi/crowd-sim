#[macro_use] extern crate log;
extern crate env_logger;

#[macro_use] mod configuration;
mod utils;

fn main() {
    env_logger::init().unwrap();

    let filename = std::env::args().nth(1).expect("NO ARGV");
    let conf = configuration::new(&filename);
    config!(conf, scene_width, SceneWidth);
    println!("{:?}", scene_width);
}
