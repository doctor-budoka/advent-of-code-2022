use std::env;
use std::fs;
use std::collections::HashMap;
use std::ops::Add;


#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
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
    let row_to_check: i32 = env_args[2].parse().unwrap();
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

    let mut count: u32 = 0;
    let mut this_point = Point::new(map.min_x.unwrap() - (map.max_distance.unwrap() as i32), row_to_check);
    let end_point = Point::new(map.max_x.unwrap() + (map.max_distance.unwrap() as i32), row_to_check);
    let increment = Point::new(1, 0);

    while this_point.x <= end_point.x {
        let tile_type = map.tile_type(this_point);
        let increase_count = match tile_type {
            Tile::Empty | Tile::Sensor => 1,
            Tile::Beacon | Tile::Unknown => 0,
        };
        count += increase_count;
        this_point = this_point + increment;
    }
    println!("Known on line y={}: {}", row_to_check, count);
} 
