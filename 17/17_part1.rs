use std::env;
use std::fs;
use std::collections::HashSet;
use std::ops::Add;
use std::io::stdout;
use std::io::Write;

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        return Point {x: x, y: y};
    }

    #[allow(dead_code)]
    fn scalar_mult(&self, scalar: i32) -> Point {
        return Self::new(scalar * self.x, scalar * self.y);
    }

    #[allow(dead_code)]
    fn distance(&self, other: &Point) -> u32 {
        return ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32;
    }

    #[allow(dead_code)]
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


struct ChamberRock {
    resting_rocks: HashSet<Point>,
    highest_rock: i32,
    left_wall: i32,
    right_wall: i32,
    floor: i32,
    jets: Vec<Point>,
    current_jet: usize,
}

impl ChamberRock {
    fn new(jets: Vec<Point>) -> ChamberRock {
        return ChamberRock {
            resting_rocks: HashSet::new(),
            highest_rock: 0,
            left_wall: 0,
            right_wall: 8,
            floor: 0,
            jets: jets,
            current_jet: 0,
        }
    }

    fn rock_fall(&mut self, rock_shape: &HashSet<Point>) {
        let mut current_pos = Point::new(self.left_wall + 3, self.highest_rock + 4);
        loop {
            current_pos = self.apply_jet_to_rock(&current_pos, rock_shape);
            let attempt_fall = self.apply_gravity(&current_pos, rock_shape);
            if attempt_fall == current_pos {
                self.set_rock(&current_pos, rock_shape);
                break;
            }
            else {
                current_pos = attempt_fall;
            }
        }
    }

    fn apply_jet_to_rock(&mut self, point: &Point, rock_shape: &HashSet<Point>) -> Point {
        let jet_direction: Point = self.get_current_jet_direction();
        let attempt_new_position: Point = *point + jet_direction;
        self.update_jet();
        
        if self.check_collision(&attempt_new_position, rock_shape) {
            return Point::new(point.x, point.y);
        }
        else {
            return attempt_new_position;
        }
    }

    fn get_current_jet_direction(&self) -> Point {
        return self.jets[self.current_jet];
    }

    fn update_jet(&mut self) {
        self.current_jet = (self.current_jet + 1) % self.jets.len();
    }

    fn apply_gravity(&self, point: &Point, rock_shape: &HashSet<Point>) -> Point {
        let attempt_new_position: Point = *point + Point::new(0, -1);

        if self.check_collision(&attempt_new_position, rock_shape) {
            return Point::new(point.x, point.y);
        }
        else {
            return attempt_new_position;
        }
    }

    fn check_collision(&self, point: &Point, rock_shape: &HashSet<Point>) -> bool {
        return self.check_collision_with_chamber(point, rock_shape) | self.check_collision_with_rocks(point, rock_shape);
    }

    fn check_collision_with_rocks(&self, point: &Point, rock_shape: &HashSet<Point>) -> bool {
        let current_rock: HashSet<Point> = self.get_current_rock(point, rock_shape);
        return current_rock.intersection(&self.resting_rocks).count() > 0;
    }

    fn check_collision_with_chamber(&self, point: &Point, rock_shape: &HashSet<Point>) -> bool {
        let current_rock: HashSet<Point> = self.get_current_rock(point, rock_shape);
        for point in current_rock {
            if (point.x <= self.left_wall) | (point.x >= self.right_wall) | (point.y <= self.floor) {return true;}
        }
        return false;
    }

    fn set_rock(&mut self, point: &Point, rock_shape: &HashSet<Point>) {
        let current_rock: HashSet<Point> = self.get_current_rock(point, rock_shape);
        for rock in current_rock {
            self.resting_rocks.insert(rock);
        }
        self.highest_rock = self.resting_rocks.iter().map(|rock| rock.y).max().unwrap();
    }

    fn get_current_rock(&self, point: &Point, rock_shape: &HashSet<Point>) -> HashSet<Point> {
        let mut current_rock: HashSet<Point> = HashSet::new();
        for rock_point in rock_shape {
            current_rock.insert(*rock_point + *point);
        }
        return current_rock;
    }

    #[allow(dead_code)]
    fn render_chamber(&self) {
        let render_height = self.highest_rock + 5;
        for i in 0..=render_height {
            for j in self.left_wall..=self.right_wall {
                if (i == render_height) & ((j == self.left_wall) | (j == self.right_wall)) {print!("{}", '+');}
                else if i == render_height {print!("{}", '-');}
                else if (j == self.left_wall) | (j == self.right_wall) {print!("{}", '|');}
                else if self.resting_rocks.contains(&Point::new(j, render_height - i)) {print!("{}", '#');}
                else {print!("{}", '.');}
            }
            print!("\n");
            stdout().flush().expect("This should print to screen");
        }
        println!("");
    }
}


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let jets: Vec<Point> = input.chars().map(|x| char_to_direction(x)).collect();

    let rocks: Vec<HashSet<Point>> = vec![
        HashSet::from([Point::new(0, 0), Point::new(1, 0), Point::new(2, 0), Point::new(3, 0)]),
        HashSet::from([Point::new(0, 1), Point::new(1, 1), Point::new(2, 1), Point::new(1, 0), Point::new(1, 2)]),
        HashSet::from([Point::new(0, 0), Point::new(1, 0), Point::new(2, 0), Point::new(2, 1), Point::new(2, 2)]),
        HashSet::from([Point::new(0, 0), Point::new(0, 1), Point::new(0, 2), Point::new(0, 3)]),
        HashSet::from([Point::new(0, 0), Point::new(0, 1), Point::new(1, 0), Point::new(1, 1)]),
    ];

    let mut chamber: ChamberRock = ChamberRock::new(jets);
    for i in 0..2022 {
        let this_rock_index = (i % 5) as usize;
        let this_rock = &rocks[this_rock_index];
        chamber.rock_fall(&this_rock);
    }

    println!("Highest rock: {}", chamber.highest_rock);
} 

fn char_to_direction(direction_char: char) -> Point {
    return match direction_char {
        '<' => Point::new(-1, 0),
        '>' => Point::new(1, 0),
        c => panic!("Invalid character found for direction: '{}'", c),
    };
}
