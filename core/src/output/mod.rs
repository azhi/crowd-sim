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
}

impl Output {
    pub fn new(configuration: &AnyMap) -> Output {
        let scene_scale = config!(configuration, SceneScale);
        let scene_filename = config!(configuration, SceneFilename);
        Output{ scene_file_name: scene_filename, scene_scale: scene_scale }
    }

    pub fn send_init(&self) {
        let mut out = ::std::io::stdout();
        self.write_string(&mut out, &self.scene_file_name);
        self.write_f64(&mut out, self.scene_scale);
    }

    pub fn dump_state(&self, simulation: &Simulation) {
        let mut out = ::std::io::stdout();
        let current_time = simulation.time.current_time;
        self.write_f64(&mut out, current_time);
        self.dump_people_location(&mut out, &simulation.scene.people);
    }

    fn dump_people_location(&self, out: &mut Write, people: &LinkedList<Person>) {
        self.write_u32(out, people.len() as u32);
        for person in people.iter() {
            self.write_u16(out, person.coordinates.x.round() as u16);
            self.write_u16(out, person.coordinates.y.round() as u16);
            self.write_f64(out, person.heading);
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
