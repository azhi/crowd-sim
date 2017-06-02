extern crate anymap;

use self::anymap::AnyMap;

use std;
use std::io::prelude::*;

use ::simulation::Simulation;
use ::simulation::person::Person;

const CURRENT_TIME_TYPE: u8 = 0_u8;
const LOCATIONS_TYPE: u8 = 1_u8;
const DENSITY_MAP_TYPE: u8 = 2_u8;
const STATISTICS_TYPE: u8 = 3_u8;

pub struct Output {
    scene_file_name: String,
    scene_scale: f64,

    density_map_enabled: bool,
    density_map_min_threshold: f64,
    density_map_max_threshold: f64,
    ticks_without_density: u32,
}

impl Output {
    pub fn new(configuration: &AnyMap) -> Output {
        let scene_scale = config!(configuration, SceneScale);
        let scene_filename = config!(configuration, SceneFilename);

        let density_map_enabled = config!(configuration, DensityMapEnabled);
        let density_map_min_threshold = config!(configuration, DensityMapMinThreshold);
        let density_map_max_threshold = config!(configuration, DensityMapMaxThreshold);

        Output{ scene_file_name: scene_filename, scene_scale: scene_scale,
                density_map_enabled: density_map_enabled, density_map_min_threshold: density_map_min_threshold,
                density_map_max_threshold: density_map_max_threshold, ticks_without_density: 0 }
    }

    pub fn send_init(&self) {
        let mut out = ::std::io::stdout();
        self.write_string(&mut out, &self.scene_file_name);
        self.write_f64(&mut out, self.scene_scale);
        self.write_f64(&mut out, self.density_map_min_threshold);
        self.write_f64(&mut out, self.density_map_max_threshold);
    }

    pub fn dump_state(&mut self, simulation: &Simulation) {
        let mut out = ::std::io::stdout();

        self.dump_current_time(&mut out, simulation);

        if self.density_map_enabled {
            if self.ticks_without_density == 0 {
                self.dump_density_map(&mut out, &simulation.scene.get_density_map());
                self.ticks_without_density = (1_f64 / simulation.time.tick).ceil() as u32;
            } else {
                self.ticks_without_density -= 1;
            }
        }

        self.dump_people_location(&mut out, &simulation.scene.people);
    }

    pub fn dump_statistics(&mut self, simulation: &Simulation) {
        let mut out = ::std::io::stdout();
        let current_time = simulation.time.current_time;
        self.write_f64(&mut out, current_time);

        self.write_u8(&mut out, STATISTICS_TYPE);
        let ref statistic_item = simulation.statistics.travel_time;
        self.write_f64(&mut out, statistic_item.min);
        self.write_f64(&mut out, statistic_item.max);
        self.write_u32(&mut out, statistic_item.count);
        self.write_f64(&mut out, statistic_item.current_avg());
        self.write_f64(&mut out, statistic_item.current_variance());
        self.write_f64(&mut out, statistic_item.current_std_deviation());
    }

    fn dump_current_time(&mut self, mut out: &mut Write, simulation: &Simulation) {
        let current_time = simulation.time.current_time;
        self.write_u8(&mut out, CURRENT_TIME_TYPE);
        self.write_f64(&mut out, current_time);
    }

    fn dump_people_location(&self, mut out: &mut Write, people: &Vec<Person>) {
        self.write_u8(&mut out, LOCATIONS_TYPE);
        self.write_u32(out, people.len() as u32);
        for person in people.iter() {
            self.write_u16(out, person.coordinates.x.round() as u16);
            self.write_u16(out, person.coordinates.y.round() as u16);
            self.write_f64(out, person.heading);
            self.write_f64(out, person.panic_level);
        }
    }

    fn dump_density_map(&self, mut out: &mut Write, density_map: &Vec<Vec<f64>>) {
        self.write_u8(&mut out, DENSITY_MAP_TYPE);
        let mut values_to_write = Vec::new();
        for i in 0..density_map.len() {
            for j in 0..density_map[i].len() {
                let value = density_map[i][j];
                if value > self.density_map_min_threshold {
                    values_to_write.push((j, i, value));
                }
            }
        };

        self.write_u32(out, values_to_write.len() as u32);
        for &(x, y, value) in values_to_write.iter() {
            self.write_u16(out, x as u16);
            self.write_u16(out, y as u16);
            self.write_f64(out, value);
        }
    }

    fn write_string(&self, out: &mut Write, string: &String) {
        let string_length = string.len();
        self.write_u16(out, string_length as u16);
        out.write(string.as_bytes()).ok().expect("Can't write to file");
    }

    fn write_u8(&self, out: &mut Write, num: u8) {
        let buf = [num];
        out.write(&buf).ok().expect("Can't write to file");
    }

    fn write_u16(&self, out: &mut Write, num: u16) {
        let buf = [
            ((num >> 8) & 0xFF) as u8,
            (num & 0xFF) as u8,
        ];
        out.write(&buf).ok().expect("Can't write to file");
    }

    fn write_u32(&self, out: &mut Write, num: u32) {
        let buf = [
            ((num >> 24) & 0xFF) as u8,
            ((num >> 16) & 0xFF) as u8,
            ((num >> 8) & 0xFF) as u8,
            (num & 0xFF) as u8,
        ];
        out.write(&buf).ok().expect("Can't write to file");
    }

    fn write_f64(&self, out: &mut Write, num: f64) {
        let buf : [u8; 8] = unsafe { std::mem::transmute(num) };
        out.write(&buf).ok().expect("Can't write to file");
    }
}
