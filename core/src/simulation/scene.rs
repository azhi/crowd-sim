extern crate anymap;
extern crate linked_list;

use self::linked_list::LinkedList;
use self::anymap::AnyMap;

use ::simulation::person::Person;
use ::simulation::forces::Forces;

use ::utils::linelg::Line;
use ::utils::linelg::Point;
use ::utils::linelg::distance::DistanceTo;

pub const APPROX_PERSON_RADIUS: f64 = 0.5_f64;

pub struct Scene {
    pub people: LinkedList<Person>,
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

pub struct Area {
    pub p0: Point, pub p1: Point,
    pub sequence_no: u8,
}

impl Area {
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

        Scene{ people: LinkedList::new(), geometry: parsed_geometry, paths: parsed_paths,
               scale: scene_scale, width: scene_width, height: scene_height }
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
            let spawn_area = SpawnArea{ area: Area{p0: Point::new(scene_spawn_area.x0 as f64, scene_spawn_area.y0 as f64),
                                                   p1: Point::new(scene_spawn_area.x1 as f64, scene_spawn_area.y1 as f64),
                                                   sequence_no: 0 },
                                        rate: spawn_rate,
                                        ticks_to_next_spawn: 1 };

            let mut parsed_target_areas : Vec<Area> = Vec::new();
            for scene_target_area in target_areas.iter() {
                if scene_target_area.id == scene_spawn_area.id {
                    let target_area = Area{ p0: Point::new(scene_target_area.x0 as f64, scene_target_area.y0 as f64),
                                            p1: Point::new(scene_target_area.x1 as f64, scene_target_area.y1 as f64),
                                            sequence_no: scene_target_area.sequence_no };
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
        for i in (1 .. 10) {
            let try_point = Point::new(
                ::utils::distributions::generate_uniform(path.spawn_area.area.p0.x, path.spawn_area.area.p1.x),
                ::utils::distributions::generate_uniform(path.spawn_area.area.p0.y, path.spawn_area.area.p1.y)
            );
            if self.is_free(&try_point) {
                coordinates = Some(try_point);
                break;
            }
        }

        match coordinates {
            Some(point) => {
                let new_person = Person{
                    coordinates: point.clone(),
                    heading: 270_f64.to_radians(),
                    path_id: path.id,
                    current_target_index: 0,
                    current_target_point: path.target_areas[0].random_inside(),
                    forces_params: forces.generate_person_forces_param()
                };
                self.people.push_back(new_person);
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

    pub fn process_reached_destination_people(&mut self) {
        let mut cursor = self.people.cursor();
        loop {
            let next_or_remove = match cursor.peek_next() {
                Some(person) => {
                    if person.reached_destination(&self.paths[person.path_id as usize]) {
                        person.current_target_index += 1;
                        let ref path = self.paths[person.path_id as usize];
                        if (person.current_target_index as usize) < path.target_areas.len() {
                            person.current_target_point = path.target_areas[person.current_target_index as usize].random_inside();
                            // next
                            true
                        } else {
                            // remove
                            false
                        }
                    } else {
                        // next
                        true
                    }
                }
                None => break
            };
            if next_or_remove {
                cursor.next();
            } else {
                cursor.remove();
            }
        }
    }

    pub fn get_density_map(&self) -> Vec<Vec<f64>> {
        const KERNEL_C : f64 = 2_f64;

        let mut res = Vec::new();
        res.reserve(self.height as usize);
        for i in (0 .. self.height) {
            let mut inner_vec = Vec::new();
            inner_vec.reserve(self.width as usize);
            for j in (0 .. self.width) {
                inner_vec.push(0_f64);
            }
            res.push(inner_vec);
        }

        let effective_c = (KERNEL_C / self.scale).round() as i32;
        for person in &self.people {
            for i in (person.coordinates.y as i32 - 3 * effective_c .. person.coordinates.y as i32 + 3 * effective_c) {
                for j in (person.coordinates.x as i32 - 3 * effective_c .. person.coordinates.x as i32 + 3 * effective_c) {
                    if i > 0 && i < self.height as i32 &&
                       j > 0 && j < self.width  as i32 {
                        let density_point = Point::new(j as f64, i as f64);
                        let density_addition = (1_f64 - density_point.distance_sqr(&person.coordinates) / 9_f64 / (effective_c as f64).powi(2));
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
