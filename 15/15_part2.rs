use std::env;
use std::fs;
use std::collections::HashMap;
use std::ops::Add;


#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Sensor,
    Beacon,
    Empty,
    Unknown,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        return Point {x: x, y: y};
    }

    fn from_string(string: &String) -> Point {
        let points: Vec<&str> = string.trim().split(",").collect();
        let x = points[0].trim().replace("x=", "").parse::<i32>().unwrap();
        let y = points[1].trim().replace("y=", "").parse::<i32>().unwrap();
        return Point {x: x, y: y};
    }   

    fn scalar_mult(&self, scalar: i32) -> Point {
        return Self::new(scalar * self.x, scalar * self.y);
    }

    fn distance(&self, other: &Point) -> u32 {
        return ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32;
    }

    fn vector_prod(&self, other: &Point) -> Point {
        return Point::new(self.x * other.x, self.y * other.y);
    }
}

impl Add for Point { 
    type Output = Self;
    fn add(self, other: Self) -> Self {
        return Self::new(self.x + other.x, self.y + other.y)
    }
}


struct Map {
    points: HashMap<Point, Tile>,
    distances: HashMap<Point, u32>,
    max_x: Option<i32>,
    min_x: Option<i32>,
    max_y: Option<i32>,
    min_y: Option<i32>,
    max_distance: Option<u32>,
}

impl Map {
    fn new() -> Map {
        return Map {
            points: HashMap::new(), 
            distances: HashMap::new(),
            max_x: None,
            min_x: None,
            max_y: None,
            min_y: None,
            max_distance: None,
        };
    }

    fn fill_point(&mut self, point: Point, tile: Tile) {
        if (self.max_x == None) || (point.x > self.max_x.unwrap()) {
            self.max_x = Some(point.x);
        }
        if (self.min_x == None) || (point.x < self.min_x.unwrap()) {
            self.min_x = Some(point.x);
        }
        if (self.max_y == None) || (point.y > self.max_y.unwrap()) {
            self.max_y = Some(point.y);
        }
        if (self.min_y == None) || (point.y < self.min_y.unwrap()) {
            self.min_y = Some(point.y);
        }
        self.points.insert(point, tile);
    }

    fn add_distance(&mut self, point: Point, distance: u32) {
        if (self.max_distance == None) || (distance > self.max_distance.unwrap()) {
            self.max_distance = Some(distance);
        }
        self.distances.insert(point, distance);
    }

    fn add_sensor_info(&mut self, sensor: Point, beacon: Point) {
        self.fill_point(sensor, Tile::Sensor);
        self.fill_point(beacon, Tile::Beacon);
        self.add_distance(sensor, sensor.distance(&beacon));
    }

    fn tile_known(&self, point: Point) -> bool {
        if self.points.contains_key(&point) {return true;}
        for (sensor, distance) in &self.distances {
            if sensor.distance(&point) <= *distance {
                return true;
            }
        }
        return false;
    }

    fn tile_type(&self, point: Point) -> Tile {
        if self.points.contains_key(&point) {
            return *self.points.get(&point).unwrap();
        }
        else {
            return if self.tile_known(point) {Tile::Empty} else {Tile::Unknown};
        }
    }
}


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    let max_coord: i32 = env_args[2].parse().unwrap();
    println!("file name is '{}'", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");

    let mut map: Map = Map::new();
    for line in input.trim().lines().collect::<Vec<&str>>() {
        let vertex_descriptions: Vec<&str> = line.trim().split(": ").collect();
        let sensor_str_coords = vertex_descriptions[0].trim().replace("Sensor at ", "");
        let beacon_str_coords = vertex_descriptions[1].trim().replace("closest beacon is at ", "");
        let sensor_point = Point::from_string(&sensor_str_coords.to_string());
        let beacon_point = Point::from_string(&beacon_str_coords.to_string());
        // map.add_sensor_info(sensor_point, beacon_point);
        map.add_sensor_info(sensor_point, beacon_point);
    }
    println!("Initialised. Bounds: {:?} <= x <= {:?}, {:?} <= y <= {:?}. Max distance: {:?}", map.min_x, map.max_x, map.min_y, map.max_y, map.max_distance);

    let mut unknowns: Vec<Point> = Vec::new();
    let mut current_point = Point::new(0, 0);
    while (current_point.x) <= max_coord && (current_point.y <= max_coord) {
        let mut moved: bool = false;
        for (sensor, distance) in &map.distances {
            let current_distance = current_point.distance(sensor);
            if current_distance <= *distance {
                let dist_diff: i32 = (distance - current_distance) as i32;
                let to_go = if current_point.x < sensor.x {dist_diff + 2*(sensor.x - current_point.x) + 1} else {dist_diff + 1};
                current_point = move_point(current_point, to_go as u32, max_coord);
                moved = true;
                break;
            }
        }
        if !moved {
            unknowns.push(current_point);
            current_point = move_point(current_point, 1, max_coord);
        }
    }
    println!("{:?}", &unknowns);
    if unknowns.len() == 1 {
        let x = unknowns[0].x as i64;
        let y = unknowns[0].y as i64;
        println!("Tuning frequency for beacon: {}", 4000000*x + y);
    }
    else if unknowns.len() == 0 {
        println!("No potential positions for the beacon found");
    }
    else {
        println!("Too many possibilities for the beacon");
    }
} 

fn move_point(point: Point, distance: u32, max_coord: i32) -> Point {
    let attempted_x: i32 = point.x + (distance as i32);
    if attempted_x <= max_coord {
        return Point::new(attempted_x, point.y);
    }
    else {
        return Point::new(0, point.y + 1);
    }
}
