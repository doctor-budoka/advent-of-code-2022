use std::io::stdout;
use std::collections::{HashMap,HashSet,VecDeque};
use std::io::Write;
use space::{Point,Direction,StdInt};

#[derive(Debug)]
pub struct Elves {
    map: HashSet<Point>,
    check_order: VecDeque<Direction>,
    min_x: Option<StdInt>,
    max_x: Option<StdInt>,
    min_y: Option<StdInt>,
    max_y: Option<StdInt>,
}

impl Elves {
    pub fn new() -> Self {
        return Self{
            map: HashSet::new(), 
            check_order: VecDeque::from([Direction::North, Direction::South, Direction::West, Direction::East]),
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
        }
    }

    pub fn add_elf(&mut self, elf: &Point) {
        self.map.insert(*elf);
        self.update_bounds(elf);
    }

    pub fn update_bounds(&mut self, new_point: &Point) {
        if (self.min_x == None) || (self.min_x.unwrap() > new_point.x) {
            self.min_x = Some(new_point.x);
        } 
        if (self.max_x == None) || (self.max_x.unwrap() < new_point.x) {
            self.max_x = Some(new_point.x);
        } 
        if (self.min_y == None) || (self.min_y.unwrap() > new_point.y) {
            self.min_y = Some(new_point.y);
        } 
        if (self.max_y == None) || (self.max_y.unwrap() < new_point.y) {
            self.max_y = Some(new_point.y);
        } 
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
        self.update_bounds(to);
    }

    pub fn render_map(&self) {
        let y_start = self.min_y.unwrap() - 1;
        let y_end = self.max_y.unwrap() + 1;
        let x_start = self.min_x.unwrap() - 1;
        let x_end = self.max_x.unwrap() + 1;

        for j in y_start..=y_end {
            for i in x_start..=x_end {
                let this_point: Point = Point::new(i, j);
                let char_to_print = if self.map.contains(&this_point) {'#'} else {'.'};
                print!("{}", char_to_print);
            }
            print!("\n");
            stdout().flush().expect("This should print to screen");
        }
        println!("");
    }

    pub fn count_empty_tiles(&self) -> StdInt {
        let num_elves = self.map.len() as StdInt;
        let num_tiles = (self.max_x.unwrap() - self.min_x.unwrap() + 1) * (self.max_y.unwrap() - self.min_y.unwrap() + 1);
        return num_tiles - num_elves;
    }
}