use std::env;
use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::io::stdout;
use std::io::Write;
use std::ops::Add;


#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
enum Tile {
    Air,
    Stone,
    Sand,
    Source,
}

#[derive(PartialEq, Eq, Debug)]
enum DropStatus {
    Success,
    SourceBlocked,
    Void,
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
        return Point {x: points[0].parse::<i32>().unwrap(), y: points[1].parse::<i32>().unwrap()};
    }   

    fn scalar_mult(&self, scalar: i32) -> Point {
        return Self::new(scalar * self.x, scalar * self.y);
    }
}

impl Add for Point { 
    type Output = Self;
    fn add(self, other: Self) -> Self {
        return Self::new(self.x + other.x, self.y + other.y)
    }
}


struct Tiles {
    tiles: HashMap<Point, Tile>,
    source: Point,
    max_x: i32,
    min_x: i32,
    max_y: i32,
    min_y: i32,
}

impl Tiles {
    fn new(source: Point) -> Tiles {
        let mut hashmap = HashMap::new();
        hashmap.insert(source, Tile::Source);
        return Tiles {
            tiles: hashmap, 
            source: source, 
            max_x: source.x,
            min_x: source.x,
            max_y: source.y,
            min_y: source.y,
        };
    }

    fn add_point(&mut self, point: Point, tile: Tile) {
        if point.x > self.max_x {self.max_x = point.x;}
        if point.x < self.min_x {self.min_x = point.x;}
        if point.y > self.max_y {self.max_y = point.y;}
        if point.y > self.max_y {self.min_y = point.y;}
        self.tiles.insert(point, tile);
    }

    fn draw_line(&mut self, start: Point, end: Point, tile: Tile) {
        let points = Self::get_points_for_line(start, end);
        for point in points.iter() {
            self.add_point(*point, tile);
        }
    }

    fn get_points_for_line(start: Point, end: Point) -> Vec<Point> {
        let increment: Point;
        match (start.x.cmp(&end.x), start.y.cmp(&end.y)) {
            (Ordering::Equal, Ordering::Equal) => return vec![Point::new(start.x, end.y)],
            (Ordering::Equal, Ordering::Less) => increment = Point::new(0, 1),
            (Ordering::Less, Ordering::Equal) => increment = Point::new(1, 0),
            (Ordering::Equal, Ordering::Greater) => increment = Point::new(0, -1),
            (Ordering::Greater, Ordering::Equal) => increment = Point::new(-1, 0),
            (_, _) => panic!("Lines should be horizontal or vertical!"),
        }

        let mut i = 0;
        let mut line = Vec::new();
        loop {
            let new_point = start + increment.scalar_mult(i);
            line.push(new_point);
            if new_point == end {
                break;
            }
            i += 1;
        }
        return line;
    }


    #[allow(dead_code)]
    fn tile_at_point(&self, point: Point) -> Tile {
        return match self.tiles.get(&point) {
            Some(tile) => *tile,
            None => Tile::Air,
        };
    }

    fn drop(&mut self) -> DropStatus {
        let mut sand = self.source;
        loop {
            let potential_new_sand = self.gravity(sand);
            let moving = potential_new_sand != sand;
            let to_source = potential_new_sand == self.source;
            let stayed_in_bounds = self.within_bounds(potential_new_sand);
            match (moving, to_source, stayed_in_bounds) {
                (true, false, true) => sand = potential_new_sand,
                (true, false, false) => return DropStatus::Void,
                (true, true, _) => panic!("Sand should move away from source!"),
                (false, true, true) => {
                    self.add_point(sand, Tile::Sand);
                    return DropStatus::SourceBlocked;
                },
                (false, false, true) => {
                    self.add_point(sand, Tile::Sand);
                    return DropStatus::Success;
                }
                (false, _, false) => panic!("Sand shouldn't be able to stay in place and be out of bounds!"),
            };
        }
    }

    fn gravity(&self, sand: Point) -> Point {
        let first_direction = Point::new(0, 1);
        let second_direction = Point::new(-1, 1);
        let third_direction = Point::new(1, 1);
        let gravity_vec = vec![first_direction, second_direction, third_direction];

        for this_vec in gravity_vec {
            if !self.tile_occupied(sand + this_vec) {
                return sand + this_vec;
            }
        }
        return sand;
    }

    fn within_bounds(&self, point: Point) -> bool {
        return (point.x <= self.max_x) && (point.x >= self.min_x) && (point.y <= self.max_y) && (point.y >= self.min_y);
    }

    fn tile_occupied(&self, point: Point) -> bool {
        return self.tiles.contains_key(&point);
    }

    #[allow(dead_code)]
    fn render_to_stdout(&self) {
        let width: i32 = self.max_x - self.min_x + 1;
        let height: i32 = self.max_y - self.min_y + 1;
        for i in 0..height {
            for j in 0..width {
                let tile_char = match self.tile_at_point(Point::new(self.min_x + j, self.min_y + i)) {
                    Tile::Air => '.',
                    Tile::Stone => '#',
                    Tile::Sand => 'o',
                    Tile::Source => '+',
                };
                print!("{}", tile_char);
            }
            print!("\n");
            stdout().flush().expect("This should print to screen");
        }
    }
}


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");

    let mut tiles: Tiles = Tiles::new(Point::new(500, 0));
    for wall in input.trim().lines().collect::<Vec<&str>>() {
        let vertices: Vec<&str> = wall.trim().split(" -> ").collect();
        let mut current: Option<Point> = None;
        let mut prev: Option<Point>;
        for point in vertices {
            prev = current;
            let this_point = Point::from_string(&point.to_string());
            current = Some(this_point);
            match (prev, current) {
                (Some(point1), Some(point2)) => tiles.draw_line(point1, point2, Tile::Stone),
                (_, _) => {},
            };
        }
    }

    let floor_height = tiles.max_y + 2;
    let max_left = tiles.source.x - floor_height;
    let max_right = tiles.source.x + floor_height;
    tiles.draw_line(Point::new(max_left, floor_height), Point::new(max_right, floor_height), Tile::Stone);

    let mut sand_count: u32 = 0;
    let mut flow: bool = true;
    while flow {
        let status = tiles.drop();
        flow = status == DropStatus::Success;
        sand_count += (status != DropStatus::Void) as u32;
    }
    println!("Sand dropped before stopping: {}", sand_count);
} 
