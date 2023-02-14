use std::env;
use std::fs;
use std::collections::{HashSet,VecDeque};

mod space;
use space::{Point,Direction,StdInt};

#[derive(Debug)]
struct Elves {
    map: HashSet<Point>,
    check_order: VecDeque<Direction>,
}

impl Elves {
    pub fn new() -> Self {
        return Self{
            map: HashSet::new(), 
            check_order: VecDeque::from([Direction::North, Direction::South, Direction::West, Direction::East])
        }
    }

    pub fn add_elf(&mut self, elf: &Point) {
        self.map.insert(*elf);
    }
}


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file"); 

    let mut elves = Elves::new();
    for (i, line) in input.lines().enumerate() {
        for (j, tile_char) in line.trim().chars().enumerate() {
            if tile_char == '.' {continue;}
            let this_point = Point::new(j as StdInt + 1, i as StdInt + 1);
            elves.add_elf(&this_point);
        }
    }
    println!("Input read: {elves:?}");
} 
