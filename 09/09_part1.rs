use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

type Position = (i32, i32);

struct Rope {
    head: Position,
    tail: Position,
    history: HashSet<Position>,
}

impl Rope {
    fn new() -> Rope {
        return Rope {head: (0, 0), tail: (0, 0), history: HashSet::from([(0, 0)])};
    }

    fn move_head_some_distance(&mut self, direction: char, distance: i32) {
        for _ in 0..distance {
            self.move_head(direction);
        }
    }

    fn move_head(&mut self, direction: char) {
        let displacement: (i32, i32) = match direction {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            other => panic!("Unexpected char for direction: {}", other),
        };
        self.head.0 += displacement.0;
        self.head.1 += displacement.1;
        self.update_tail();
    }

    fn update_tail(&mut self) {
        let diff = (self.head.0 - self.tail.0, self.head.1 - self.tail.1);
        let displacement = match (diff.0.abs(), diff.1.abs()) {
            (2, 1) | (1, 2) => (diff.0/diff.0.abs(), diff.1/diff.1.abs()),
            (2, 0) => (diff.0/diff.0.abs(), 0),
            (0, 2) => (0, diff.1/diff.1.abs()),
            (1, 1) | (0, 1) | (1, 0) | (0, 0) => (0, 0),
            _ => panic!("Head and tail are further apart than they should be. Head: ({}, {}), Tail: ({}, {})", self.head.0, self.head.1, self.tail.0, self.tail.1),
        };
        self.tail.0 += displacement.0;
        self.tail.1 += displacement.1;
        self.history.insert(self.tail);
    }
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    
    let mut rope =  Rope::new();
    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(val) = line {
                let instr_vec: Vec<&str> = val.split(" ").collect();
                let direction: char = instr_vec[0].chars().nth(0).unwrap();
                let distance: i32 = instr_vec[1].parse().unwrap();
                rope.move_head_some_distance(direction, distance);    
            }
        }
    }
    println!("Rope has been in {} positions.", rope.history.len());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

