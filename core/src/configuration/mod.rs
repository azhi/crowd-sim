extern crate anymap;

use std;
use std::str;

use std::io::prelude::*;
use std::fs::File;

use self::anymap::AnyMap;

macro_rules! config {
    ($config:ident, $config_type:ident) => {
        {
            let &::configuration::$config_type(ref config_tmp) =
                $config.get::<::configuration::$config_type>()
                  .expect(format!("Trying to access missing config. Called from {:?}:{:?}", file!(), line!()).as_ref());
            config_tmp.clone()
        }
    };
}

#[derive(Debug,Clone)]
pub enum DistributionValue {
    UniformDistributionValue{ from: f64, to: f64 },
    NormalDistributionValue{ mean: f64, std_deviation: f64 },
    TimeInfiniteDistributionValue{ avg_rate: f64, rate_deviation: f64 },
}

#[derive(Debug,Clone)]
pub struct SceneWidth(pub u16);
#[derive(Debug,Clone)]
pub struct SceneHeight(pub u16);
#[derive(Debug,Clone)]
pub struct SceneScale(pub f64);
#[derive(Debug,Clone)]
pub struct SceneWall {
    pub x0 : u16, pub y0 : u16, pub x1 : u16, pub y1: u16,
}
#[derive(Debug,Clone)]
pub struct SceneWalls(pub Vec<SceneWall>);
#[derive(Debug,Clone)]
pub struct SceneSpawnArea {
    pub x0 : u16, pub y0 : u16, pub x1 : u16, pub y1: u16,
    pub id: u8
}
#[derive(Debug,Clone)]
pub struct SceneSpawnAreas(pub Vec<SceneSpawnArea>);
#[derive(Debug,Clone)]
pub struct SceneTargetArea {
    pub x0 : u16, pub y0 : u16, pub x1 : u16, pub y1: u16,
    pub sequence_no: u8,
    pub last: bool
}
#[derive(Debug,Clone)]
pub struct SceneTargetAreas(pub Vec<SceneTargetArea>);
#[derive(Debug,Clone)]
pub struct SceneFilename(pub String);

#[derive(Debug,Clone)]
pub struct TimeEndTime(pub u32);
#[derive(Debug,Clone)]
pub struct TimeTick(pub f64);

#[derive(Debug,Clone)]
pub struct SpawnTime(pub DistributionValue);
#[derive(Debug,Clone)]
pub struct SpawnRate(pub f64);

#[derive(Debug,Clone)]
pub struct ForcesTargetSpeed(pub DistributionValue);
#[derive(Debug,Clone)]
pub struct ForcesRepulsionCoeff(pub DistributionValue);

pub fn new(file: &mut Read) -> AnyMap {
    let mut config = AnyMap::new();
    parse_config_file(&mut config, file);
    config
}

fn parse_config_file(config: &mut AnyMap, file: &mut Read) {
    info!("Starting to parse config file");

    let mut buf = [0u8; 1024];
    let mut done = false;
    while !done {
        done = !parse_single_item(config, file, &mut buf);
    }
    info!("Config readed.");
}

fn parse_single_item(config: &mut AnyMap, file: &mut Read, buf : &mut [u8]) -> bool {
    let read = file.read(&mut buf[0 .. 1]).ok().expect("Can't read from file");
    if read != 0 {
        let section = buf[0];
        match section {
            0x01 => parse_scene_item(config, file, buf),
            0x02 => parse_time_item(config, file, buf),
            0x03 => parse_spawn_item(config, file, buf),
            0x04 => parse_forces_item(config, file, buf),
            _ => panic!("Unknown section in config: {}", section)
        }
        // let str_value = str::from_utf8(&[116, 116, 101, 115, 116]).unwrap().to_string().clone();
        // config.insert(SceneFile(str_value));
        true
    } else {
        false
    }
}

fn parse_scene_item(config: &mut AnyMap, file: &mut Read, buf : &mut [u8]) {
    let element = parse_u16(file, buf);
    match element {
        0x01 => {
            let (x0, y0, x1, y1) = parse_coordinates(file, buf);

            let mut walls_vec = match config.remove::<SceneWalls>() {
                Some(scene_walls) => {
                    let SceneWalls(vec) = scene_walls;
                    vec
                },
                None => Vec::new()
            };

            walls_vec.push(SceneWall{ x0: x0, y0: y0, x1: x1, y1: y1});
            config.insert(SceneWalls(walls_vec));
            debug!("Parsed SceneWall: {} {} {} {}", x0, y0, x1, y1);
        },
        0x02 => {
            let (x0, y0, x1, y1) = parse_coordinates(file, buf);
            let id = parse_u8(file, buf);

            let mut spawn_areas_vec = match config.remove::<SceneSpawnAreas>() {
                Some(scene_spawn_areas) => {
                    let SceneSpawnAreas(vec) = scene_spawn_areas;
                    vec
                },
                None => Vec::new()
            };

            spawn_areas_vec.push(SceneSpawnArea{ x0: x0, y0: y0, x1: x1, y1: y1, id: id});
            config.insert(SceneSpawnAreas(spawn_areas_vec));
            debug!("Parsed SceneSpawnArea: {} {} {} {} {}", x0, y0, x1, y1, id);
        },
        0x03 => {
            let (x0, y0, x1, y1) = parse_coordinates(file, buf);
            let seq_no_and_last = parse_u8(file, buf);
            let last = seq_no_and_last & 0x01 == 0x01;
            let seq_no = (seq_no_and_last & 0xFE) >> 1;

            let mut target_areas_vec = match config.remove::<SceneTargetAreas>() {
                Some(scene_target_areas) => {
                    let SceneTargetAreas(vec) = scene_target_areas;
                    vec
                },
                None => Vec::new()
            };

            target_areas_vec.push(SceneTargetArea{ x0: x0, y0: y0, x1: x1, y1: y1, sequence_no: seq_no, last: last});
            config.insert(SceneTargetAreas(target_areas_vec));
            debug!("Parsed SceneTargetArea: {} {} {} {} {} {}", x0, y0, x1, y1, seq_no, last);
        },
        0x11 => {
            let scene_width = parse_u16(file, buf);
            config.insert(SceneWidth(scene_width));
            debug!("Parsed SceneWidth: {}", scene_width);
        },
        0x12 => {
            let scene_height = parse_u16(file, buf);
            config.insert(SceneHeight(scene_height));
            debug!("Parsed SceneHeight: {}", scene_height);
        },
        0x13 => {
            let scene_scale = parse_f64(file, buf);
            config.insert(SceneScale(scene_scale));
            debug!("Parsed SceneScale: {}", scene_scale);
        },
        0xFF => {
            let scene_filename = parse_string(file, buf);
            debug!("Parsed SceneFilename: {}", scene_filename);
            config.insert(SceneFilename(scene_filename));
        }
        _ => panic!("Unknown element in scene config: {}", element)
    };
}

fn parse_time_item(config: &mut AnyMap, file: &mut Read, buf : &mut [u8]) {
    let element = parse_u16(file, buf);
    match element {
        0x01 => {
            let end_time = parse_u32(file, buf);
            config.insert(TimeEndTime(end_time));
            debug!("Parsed TimeEndTime: {}", end_time);
        },
        0x02 => {
            let tick = parse_f64(file, buf);
            config.insert(TimeTick(tick));
            debug!("Parsed TimeTick: {}", tick);
        },
        _ => panic!("Unknown element in time config: {}", element)
    };
}

fn parse_spawn_item(config: &mut AnyMap, file: &mut Read, buf : &mut [u8]) {
    let element = parse_u16(file, buf);
    match element {
        0x01 => {
            let rate = parse_f64(file, buf);
            config.insert(SpawnRate(rate));
            debug!("Parsed SpawnRate: {}", rate);
        },
        0x02 => {
            let distribution = parse_distribution(file, buf);
            debug!("Parsed SpawnTime: {:?}", distribution);
            config.insert(SpawnTime(distribution));
        },
        _ => panic!("Unknow element in spawn config: {}", element)
    }
}

fn parse_forces_item(config: &mut AnyMap, file: &mut Read, buf : &mut [u8]) {
    let sub_section = parse_u8(file, buf);
    match sub_section {
        0x01 => parse_repulsion_force_item(config, file, buf),
        0x02 => parse_target_force_item(config, file, buf),
        _ => panic!("Unknown force: {}", sub_section)
    }
}

fn parse_repulsion_force_item(config: &mut AnyMap, file: &mut Read, buf : &mut [u8]) {
    let element = parse_u8(file, buf);
    match element {
        0x01 => {
            let distribution = parse_distribution(file, buf);
            debug!("Parsed ForcesRepulsionCoeff: {:?}", distribution);
            config.insert(ForcesRepulsionCoeff(distribution));
        }
        _ => panic!("Unknown element in repulsion force: {}", element)
    }
}

fn parse_target_force_item(config: &mut AnyMap, file: &mut Read, buf : &mut [u8]) {
    let element = parse_u8(file, buf);
    match element {
        0x01 => {
            let distribution = parse_distribution(file, buf);
            debug!("Parsed ForcesTargetSpeed: {:?}", distribution);
            config.insert(ForcesTargetSpeed(distribution));
        }
        _ => panic!("Unknown element in target force: {}", element)
    }
}

fn parse_coordinates(file: &mut Read, buf : &mut [u8]) -> (u16, u16, u16, u16) {
    let mut coordinates = [0u16; 4];
    file.read(&mut buf[0 .. 8]).ok().expect("Can't read from file");
    for i in (0..4) {
        coordinates[i] = two_u8le_to_u16(buf[2 * i], buf[2 * i + 1]);
    }
    (coordinates[0], coordinates[1], coordinates[2], coordinates[3])
}

fn parse_distribution(file: &mut Read, buf : &mut [u8]) -> DistributionValue {
    let distribution_type = parse_u8(file, buf);
    match distribution_type {
        0x01 => {
            let (from, to) = (parse_f64(file, buf), parse_f64(file, buf));
            DistributionValue::UniformDistributionValue{ from: from, to: to }
        },
        0x02 => {
            let (mean, std_deviation) = (parse_f64(file, buf), parse_f64(file, buf));
            DistributionValue::NormalDistributionValue{ mean: mean, std_deviation: std_deviation }
        },
        0x03 => {
            let (avg_rate, rate_deviation) = (parse_f64(file, buf), parse_f64(file, buf));
            DistributionValue::TimeInfiniteDistributionValue{ avg_rate: avg_rate, rate_deviation: rate_deviation }
        },
        _ => panic!("Unknown distribution type: {}", distribution_type)
    }
}

fn parse_string(file: &mut Read, buf : &mut [u8]) -> String {
    let string_length = parse_u16(file, buf);
    file.read(&mut buf[0 .. string_length as usize]).ok().expect("Can't read from file");
    let mut string_bin = Vec::new();
    for &x in (&buf[0 .. string_length as usize]).iter() {
        string_bin.push(x);
    }
    let string = str::from_utf8(string_bin.as_slice()).ok().expect("Invalid UTF-8 sequence in string");
    string.to_string()
}

fn parse_u8(file: &mut Read, buf : &mut [u8]) -> u8 {
    file.read(&mut buf[0 .. 1]).ok().expect("Can't read from file");
    buf[0]
}

fn parse_u16(file: &mut Read, buf : &mut [u8]) -> u16 {
    file.read(&mut buf[0 .. 2]).ok().expect("Can't read from file");
    two_u8le_to_u16(buf[0], buf[1])
}

fn parse_u32(file: &mut Read, buf : &mut [u8]) -> u32 {
    file.read(&mut buf[0 .. 4]).ok().expect("Can't read from file");
    four_u8le_to_u32(buf[0], buf[1], buf[2], buf[3])
}

fn parse_f64(file: &mut Read, buf : &mut [u8]) -> f64 {
    file.read(&mut buf[0 .. 8]).ok().expect("Can't read from file");
    let mut value_bin = [0u8; 8];
    for (&x, p) in (&buf[0 .. 8]).iter().zip(value_bin.iter_mut()) {
        *p = x;
    }
    let value : f64 = unsafe { std::mem::transmute(value_bin) };
    value
}

fn two_u8le_to_u16(x1: u8, x2: u8) -> u16 {
    let result = ((x1 as u16) << 8) & 0xFF00 | (x2 as u16);
    result
}

fn four_u8le_to_u32(x1: u8, x2: u8, x3: u8, x4: u8) -> u32 {
    let result =
        ((x1 as u32) << 24) & 0xFF000000 |
        ((x2 as u32) << 16) & 0x00FF0000 |
        ((x3 as u32) << 8)  & 0x0000FF00 |
        (x4 as u32);
    result
}
