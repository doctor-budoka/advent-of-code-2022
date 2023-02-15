use std::env;
use std::fs;
use std::collections::{HashMap,HashSet,VecDeque};

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

    pub fn run_round(&mut self) {
        let accepted_proposals = self.get_accepted_proposals();
        for (elf, move_to) in accepted_proposals {
            self.move_elf(&elf, &move_to);
        }
        let new_last = self.check_order.pop_front().unwrap();
        self.check_order.push_back(new_last);
    }

    pub fn get_accepted_proposals(&self) -> HashMap<Point,Point> {
        let mut proposals: HashMap<Point,Point> = HashMap::new();
        let mut counts: HashMap<Point,u32> = HashMap::new();
        for elf in &self.map {
            let this_proposal = self.get_proposal_for_elf(&elf);
            if let Some(proposed_point) = this_proposal {
                proposals.insert(*elf, proposed_point);
                if !counts.contains_key(&proposed_point) {
                    counts.insert(proposed_point, 0);
                }
                let this_count: u32 = *counts.get(&proposed_point).unwrap();
                counts.insert(proposed_point, this_count + 1);
            }
        }
        let mut allowed_proposals: HashMap<Point,Point> = HashMap::new();
        for (elf, move_to) in proposals {
            let count = *counts.get(&move_to).unwrap();
            if count == 1 {
                allowed_proposals.insert(elf, move_to);
            }
        }
        return allowed_proposals
    }

    pub fn get_proposal_for_elf(&self, elf: &Point) -> Option<Point> {
        let neighbours: HashSet<Point> = self.get_elf_neighbours(elf);
        if neighbours.len() == 0 {return None;}

        for direction in &self.check_order {
            let all_directions = direction.get_directionlies();
            let directions_as_vec: Vec<Point> = all_directions.iter().map(|x| Point::from_direction(&x)).collect();
            let direction_empty = directions_as_vec.iter().map(|x| !neighbours.contains(&x)).reduce(|x, y| x & y).unwrap();
            if direction_empty {return Some(*elf + Point::from_direction(&direction));}
        }
        return None;
    }

    pub fn get_elf_neighbours(&self, elf: &Point) -> HashSet<Point> {
        let mut elf_directions = HashSet::new();
        for vec_direction in Direction::get_directions().iter().map(|x| Point::from_direction(x)) {
            let potential_elf: Point = *elf + vec_direction;
            if self.map.contains(&potential_elf) {
                elf_directions.insert(vec_direction);
            }
        }
        return elf_directions;
    }

    pub fn move_elf(&mut self, elf: &Point, to: &Point) {
        self.map.remove(elf);
        self.map.insert(*to);
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

    elves.run_round();
    println!("New state: {elves:?}");
} 
