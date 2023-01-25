use std::env;
use std::fs;
use std::collections::HashSet;
use std::ops::Add;

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
    for line in input.trim().lines().collect::<Vec<&str>>() {
        let coords: Vec<i32> = line.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        let this_cube: Point = Point::new(coords[0], coords[1], coords[2]);

        for direction in &directions {
            let potential_cube = this_cube + *direction;
            overlapping_surfaces += 2 * (cubes.contains(&potential_cube) as i32);
        }
        cubes.insert(this_cube);
        num_cubes += 1;
    }
    println!("Surface area: {}", (6 * num_cubes) - overlapping_surfaces);
} 
