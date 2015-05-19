extern crate anymap;
extern crate linked_list;

use self::linked_list::LinkedList;
use self::anymap::AnyMap;

use std;
use std::io::prelude::*;

use ::simulation::Simulation;
use ::simulation::person::Person;

pub struct Output {
    scene_file_name: String,
    scene_scale: f64,
    ticks_without_density: u32,
}

impl Output {
    pub fn new(configuration: &AnyMap) -> Output {
        let scene_scale = config!(configuration, SceneScale);
        let scene_filename = config!(configuration, SceneFilename);
        Output{ scene_file_name: scene_filename, scene_scale: scene_scale, ticks_without_density: 0 }
    }

    pub fn send_init(&self) {
        let mut out = ::std::io::stdout();
        self.write_string(&mut out, &self.scene_file_name);
        self.write_f64(&mut out, self.scene_scale);
    }

    pub fn dump_state(&mut self, simulation: &Simulation) {
        let mut out = ::std::io::stdout();
        let current_time = simulation.time.current_time;
        self.write_f64(&mut out, current_time);
        if self.ticks_without_density == 0 {
            self.write_u8(&mut out, 1_u8);
            self.dump_density_map(&mut out, &simulation.scene.get_density_map());
            self.ticks_without_density = (1_f64 / simulation.time.tick).ceil() as u32;
        } else {
            self.write_u8(&mut out, 0_u8);
            self.ticks_without_density -= 1;
        }
        self.dump_people_location(&mut out, &simulation.scene.people);
    }

    fn dump_people_location(&self, out: &mut Write, people: &LinkedList<Person>) {
        // debug!("People {}", people.len());
        self.write_u32(out, people.len() as u32);
        for person in people.iter() {
            self.write_u16(out, person.coordinates.x.round() as u16);
            self.write_u16(out, person.coordinates.y.round() as u16);
            self.write_f64(out, person.heading);
        }
    }

    fn dump_density_map(&self, out: &mut Write, density_map: &Vec<Vec<f64>>) {
        const MIN_DENSITY_TRESHOLD : f64 = 5.0;
        let mut min_non_zero = ::std::f64::MAX;
        let mut max = ::std::f64::MIN;
        let mut values_to_write = Vec::new();
        for i in (0 .. density_map.len()) {
            for j in (0 .. density_map[i].len()) {
                let value = density_map[i][j];
                if value > MIN_DENSITY_TRESHOLD {
                    values_to_write.push((j, i, value));
                }

                if value > max {
                    max = value;
                };
                if value != 0_f64 && value < min_non_zero {
                    min_non_zero = value;
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
