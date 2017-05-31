extern crate anymap;

use self::anymap::AnyMap;

use ::simulation::person::Person;
use ::simulation::forces::Forces;

use ::utils::linelg::Line;
use ::utils::linelg::Point;
use ::utils::linelg::Rectangle;
use ::utils::linelg::distance::DistanceTo; 
pub const APPROX_PERSON_RADIUS: f64 = 0.3_f64;

pub struct Scene {
    pub people: Vec<Person>,
    pub last_person_id: u64,
    pub geometry: Vec<Line>,
    paths: Vec<Path>,
    pub scale: f64,
    pub width: u16,
    pub height: u16,
}

pub struct Path {
    pub id: u8,
    spawn_area: SpawnArea,
    pub target_areas: Vec<Area>,
}

struct SpawnArea {
    area: Area,
    rate: f64,
    ticks_to_next_spawn: u16,
}

#[derive(Debug,Clone)]
pub struct Area {
    pub p0: Point, pub p1: Point,
    pub sequence_no: u8,
    rectangle: Rectangle,
}

impl Area {
    fn new(p0: Point, p1: Point, sequence_no: u8) -> Area {
        let rectangle = Rectangle::new_from_raw(
            p0.x, p0.y, p1.x, p1.y,
        );
        Area{p0: p0, p1: p1, sequence_no: sequence_no, rectangle: rectangle}
    }

    pub fn nearest_point(&self, other: &Point) -> Point {
        other.nearest_point(&self.rectangle)
    }

    fn random_inside(&self) -> Point {
        Point::new(
            ::utils::distributions::generate_uniform(self.p0.x, self.p1.x),
            ::utils::distributions::generate_uniform(self.p0.y, self.p1.y)
        )
    }

}

impl Scene {
    pub fn new(configuration: &AnyMap) -> Scene {
        let scene_width = config!(configuration, SceneWidth);
        let scene_height = config!(configuration, SceneHeight);
        let scene_scale = config!(configuration, SceneScale);

        let scene_walls = config!(configuration, SceneWalls);
        let scene_spawn_areas = config!(configuration, SceneSpawnAreas);
        let scene_target_areas = config!(configuration, SceneTargetAreas);
        let spawn_rate = config!(configuration, SpawnRate);

        let parsed_geometry = Scene::parse_walls(scene_walls);
        let parsed_paths = Scene::parse_paths(scene_spawn_areas, scene_target_areas, spawn_rate);

        Scene{ people: Vec::new(), last_person_id: 0, geometry: parsed_geometry,
               paths: parsed_paths, scale: scene_scale, width: scene_width, height: scene_height }
    }

    fn parse_walls(walls: Vec<::configuration::SceneWall>) -> Vec<Line> {
        let mut geometry = Vec::new();
        for wall in walls.iter() {
            geometry.push(Line::new_from_raw(wall.x0 as f64, wall.y0 as f64, wall.x1 as f64, wall.y1 as f64))
        }
        geometry
    }

    fn parse_paths(spawn_areas: Vec<::configuration::SceneSpawnArea>, target_areas: Vec<::configuration::SceneTargetArea>, spawn_rate: f64) -> Vec<Path> {
        let mut paths = Vec::new();
        for scene_spawn_area in spawn_areas.iter() {
            let id = scene_spawn_area.id;
            let area = Area::new(
                Point::new(scene_spawn_area.x0 as f64, scene_spawn_area.y0 as f64),
                Point::new(scene_spawn_area.x1 as f64, scene_spawn_area.y1 as f64),
                0
            );
            let spawn_area = SpawnArea{ area: area,
                                        rate: spawn_rate,
                                        ticks_to_next_spawn: 1 };

            let mut parsed_target_areas : Vec<Area> = Vec::new();
            for scene_target_area in target_areas.iter() {
                if scene_target_area.id == scene_spawn_area.id {
                    let target_area = Area::new(
                        Point::new(scene_target_area.x0 as f64, scene_target_area.y0 as f64),
                        Point::new(scene_target_area.x1 as f64, scene_target_area.y1 as f64),
                        scene_target_area.sequence_no
                    );
                    parsed_target_areas.push(target_area)
                }
            }
            parsed_target_areas.sort_by(|a, b| a.sequence_no.cmp(&b.sequence_no));

            paths.push(Path{ id: id, spawn_area: spawn_area, target_areas: parsed_target_areas });
        }
        paths
    }

    pub fn spawn_people(&mut self, forces: &Forces, tick: f64) {
        let mut paths_needed_spawn = Vec::new();
        let paths_count = self.paths.len();

        for (path, index) in self.paths.iter_mut().zip(0 .. paths_count) {
            let ref mut spawn_area = path.spawn_area;
            spawn_area.ticks_to_next_spawn -= 1;
            if spawn_area.ticks_to_next_spawn == 0 {
                paths_needed_spawn.push(index);
                spawn_area.ticks_to_next_spawn = (1_f64 / spawn_area.rate / tick).ceil() as u16;
            }
        }

        for path_index in paths_needed_spawn.iter() {
            self.spawn_in_path(forces, *path_index);
        }
    }

    fn spawn_in_path(&mut self, forces: &Forces, path_index: usize) {
        let path = &self.paths[path_index];

        let mut coordinates: Option<Point> = None;
        for _i in 1..10 {
            let try_point = path.spawn_area.area.random_inside();
            if self.is_free(&try_point) {
                coordinates = Some(try_point);
                break;
            }
        }

        match coordinates {
            Some(point) => {
                let current_target_area = path.target_areas[0].clone();
                let target = current_target_area.nearest_point(&point);
                let direction = target - point;
                let new_person = Person{
                    id: self.last_person_id,
                    coordinates: point.clone(),
                    heading: ::utils::headings::vector_heading(direction),
                    path_id: path.id,
                    current_target_index: 0,
                    current_target_area: current_target_area,
                    panic_level: 0.5_f64,
                    forces_params: forces.generate_person_forces_param()
                };
                self.last_person_id += 1;
                self.people.push(new_person);
            },
            None => warn!("Couldn't find a place for a new person in 10 attempts, skipping ...")
        }
    }

    fn is_free(&self, p: &Point) -> bool {
        let mut free = true;
        for person in self.people.iter() {
            if person.coordinates.distance_sqr(p) < (APPROX_PERSON_RADIUS / self.scale).powi(2) {
                free = false;
                break;
            }
        }
        free
    }

    pub fn process_reached_destination_people(&mut self) -> Vec<Person> {
        let cloned_people = self.people.clone();
        let mut reached_destination_people = Vec::new();
        self.people = cloned_people.into_iter().filter_map(|mut person|
            if person.reached_destination(&self.paths[person.path_id as usize]) {
                person.current_target_index += 1;
                let ref path = self.paths[person.path_id as usize];
                if (person.current_target_index as usize) < path.target_areas.len() {
                    person.current_target_area = path.target_areas[person.current_target_index as usize].clone();
                    // person has next target, do not filter him
                    Some(person)
                } else {
                    // person reached his final target, save him for returning & filter out from people
                    reached_destination_people.push(person);
                    None
                }
            } else {
                // person in on the way to his next target, do not filter him
                Some(person)
            }
        ).collect();

        return reached_destination_people;
    }

    pub fn get_density_map(&self) -> Vec<Vec<f64>> {
        const KERNEL_C : f64 = 2_f64;

        let mut res = Vec::new();
        res.reserve(self.height as usize);
        for _i in 0..self.height {
            let mut inner_vec = Vec::new();
            inner_vec.reserve(self.width as usize);
            for _j in 0..self.width {
                inner_vec.push(0_f64);
            }
            res.push(inner_vec);
        }

        let effective_c = (KERNEL_C / self.scale).round() as i32;
        for person in &self.people {
            for i in person.coordinates.y as i32 - 3 * effective_c .. person.coordinates.y as i32 + 3 * effective_c {
                for j in person.coordinates.x as i32 - 3 * effective_c .. person.coordinates.x as i32 + 3 * effective_c {
                    if i > 0 && i < self.height as i32 &&
                       j > 0 && j < self.width  as i32 {
                        let density_point = Point::new(j as f64, i as f64);
                        let density_addition = 1_f64 - density_point.distance_sqr(&person.coordinates) / 9_f64 / (effective_c as f64).powi(2);
                        if density_addition > 0_f64 {
                            res[i as usize][j as usize] += density_addition.powi(2);
                        }
                    }
                }
            }
        }
        res
    }
}
