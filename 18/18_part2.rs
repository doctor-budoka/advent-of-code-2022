use std::env;
use std::fs;
use std::collections::{VecDeque, HashSet};
use std::ops::Add;
use std::cmp::{min,max};

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        return Point {x: x, y: y, z:z};
    }

    #[allow(dead_code)]
    fn scalar_mult(&self, scalar: i32) -> Point {
        return Self::new(scalar * self.x, scalar * self.y, scalar * self.z);
    }

    #[allow(dead_code)]
    fn distance(&self, other: &Point) -> u32 {
        return ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as u32;
    }

    #[allow(dead_code)]
    fn vector_prod(&self, other: &Point) -> Point {
        return Point::new(self.x * other.x, self.y * other.y, self.z * other.z);
    }
}

impl Add for Point { 
    type Output = Self;
    fn add(self, other: Self) -> Self {
        return Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

#[derive(Debug)]
struct BoundingBox {
    max_x: Option<i32>,
    min_x: Option<i32>,
    max_y: Option<i32>,
    min_y: Option<i32>,
    max_z: Option<i32>,
    min_z: Option<i32>,

}

impl BoundingBox {
    fn new() -> BoundingBox {
        return BoundingBox {
            max_x: None,
            min_x: None,
            max_y: None,
            min_y: None,
            max_z: None,
            min_z: None,
        }
    }

    fn update_bounds(&mut self, point: &Point) {
        self.update_x_bounds(point);
        self.update_y_bounds(point);
        self.update_z_bounds(point);
    }

    fn update_x_bounds(&mut self, point: &Point) {
        let new_x = point.x;
        match self.max_x {
            Some(bound) => self.max_x = Some(max(bound, new_x)),
            None => self.max_x = Some(new_x),
        }
        match self.min_x {
            Some(bound) => self.min_x = Some(min(bound, new_x)),
            None => self.min_x = Some(new_x),
        }
    }

    fn update_y_bounds(&mut self, point: &Point) {
        let new_y = point.y;
        match self.max_y {
            Some(bound) => self.max_y = Some(max(bound, new_y)),
            None => self.max_y = Some(new_y),
        }
        match self.min_y {
            Some(bound) => self.min_y = Some(min(bound, new_y)),
            None => self.min_y = Some(new_y),
        }
    }

    fn update_z_bounds(&mut self, point: &Point) {
        let new_z = point.z;
        match self.max_z {
            Some(bound) => self.max_z = Some(max(bound, new_z)),
            None => self.max_z = Some(new_z),
        }
        match self.min_z {
            Some(bound) => self.min_z = Some(min(bound, new_z)),
            None => self.min_z = Some(new_z),
        }
    }

    fn exterior_surface(&self) -> i32 {
        return match (self.min_x, self.max_x, self.min_y, self.max_y, self.min_z, self.max_z) {
            (Some(_), Some(_), Some(_), Some(_), Some(_), Some(_)) => (2 * self.z_face_area()) + (2 * self.y_face_area())+ (2 * self.x_face_area()),
            _ => 0,
        };
    }

    fn z_face_area(&self) -> i32 {
        return self.length_x() * self.length_y();
    }

    fn y_face_area(&self) -> i32 {
        return self.length_x() * self.length_z();
    }

    fn x_face_area(&self) -> i32 {
        return self.length_y() * self.length_z();
    }

    fn length_x(&self) -> i32 {
        return match (self.min_x, self.max_x) {
            (Some(xl), Some(xu)) => xu - xl + 1,
            _ => 0,
        };
    }

    fn length_y(&self) -> i32 {
        return match (self.min_y, self.max_y) {
            (Some(yl), Some(yu)) => yu - yl + 1,
            _ => 0,
        };
    }

    fn length_z(&self) -> i32 {
        return match (self.min_z, self.max_z) {
            (Some(zl), Some(zu)) => zu - zl + 1,
            _ => 0,
        };
    }

    fn expand(&mut self) {
        let new_min = Point::new(self.min_x.unwrap() - 1, self.min_y.unwrap() - 1, self.min_z.unwrap() - 1);
        self.update_bounds(&new_min);
        let new_max = Point::new(self.max_x.unwrap() + 1, self.max_y.unwrap() + 1, self.max_z.unwrap() + 1);
        self.update_bounds(&new_max);
    }

    fn in_bounds(&self, point: &Point) -> bool {
        return match (self.min_x, self.max_x, self.min_y, self.max_y, self.min_z, self.max_z) {
            (Some(_), Some(_), Some(_), Some(_), Some(_), Some(_)) => self.check_existing_bounds(point),
            _ => false,
        };
    }

    fn check_existing_bounds(&self, point: &Point) -> bool {
        return self.check_existing_x_bounds(point) & self.check_existing_y_bounds(point) & self.check_existing_z_bounds(point);
    }

    fn check_existing_x_bounds(&self, point: &Point) -> bool {
        return (self.min_x.unwrap() <= point.x) & (self.max_x.unwrap() >= point.x)
    }

    fn check_existing_y_bounds(&self, point: &Point) -> bool {
        return (self.min_y.unwrap() <= point.y) & (self.max_y.unwrap() >= point.y)
    }

    fn check_existing_z_bounds(&self, point: &Point) -> bool {
        return (self.min_z.unwrap() <= point.z) & (self.max_z.unwrap() >= point.z)
    }
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");

    let directions: Vec<Point> = vec![
        Point::new(0, 0, 1),
        Point::new(0, 0, -1),
        Point::new(0, 1, 0),
        Point::new(0, -1, 0),
        Point::new(1, 0, 0),
        Point::new(-1, 0, 0),
    ];

    let mut cubes: HashSet<Point> = HashSet::new();
    let mut num_cubes: i32 = 0;
    let mut overlapping_surfaces: i32 = 0;
    let mut bounding_box: BoundingBox = BoundingBox::new();
    for line in input.trim().lines().collect::<Vec<&str>>() {
        let coords: Vec<i32> = line.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        let this_cube: Point = Point::new(coords[0], coords[1], coords[2]);

        for direction in &directions {
            let potential_cube = this_cube + *direction;
            overlapping_surfaces += 2 * (cubes.contains(&potential_cube) as i32);
        }
        cubes.insert(this_cube);
        num_cubes += 1;
        bounding_box.update_bounds(&this_cube);
    }
    println!("Full lava surface area: {}", (6 * num_cubes) - overlapping_surfaces);

    let mut air: HashSet<Point> = HashSet::new();
    bounding_box.expand();
    let mut this_air: Point = Point::new(
        bounding_box.min_x.unwrap(), 
        bounding_box.min_y.unwrap(), 
        bounding_box.min_z.unwrap()
    );
    let mut air_surface: i32 = 0;

    let mut queue: VecDeque<Point> = VecDeque::new();
    let mut queued: HashSet<Point> = HashSet::new();

    loop {
        let mut neighbouring_air: i32 = 0;
        for direction in &directions {
            let potential_air: Point = this_air + *direction;
            if air.contains(&potential_air) {
                neighbouring_air += 1;
            }
            else if !queued.contains(&potential_air) && bounding_box.in_bounds(&potential_air) && !cubes.contains(&potential_air) {
                queued.insert(potential_air);
                queue.push_back(potential_air);
            }
        }
        air.insert(this_air);
        air_surface += 6 - (2 * neighbouring_air);

        match queue.pop_front() {
            Some(neighbour) => this_air = neighbour,
            None => break,
        };
    }
    println!("Full air surface area: {}", air_surface);
    println!("Exterior air surface area: {}", bounding_box.exterior_surface());
    println!("Interior air/exterior lava surface area: {}", air_surface - bounding_box.exterior_surface());
} 
