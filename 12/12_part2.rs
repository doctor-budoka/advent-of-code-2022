use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{VecDeque, HashSet};
use std::rc::Rc;
use std::borrow::BorrowMut;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Coordinate {
    row: usize,
    column: usize,
}

impl Coordinate {
    fn new(row: usize, column: usize) -> Coordinate {
        return Coordinate {row: row, column: column};
    }
}

#[derive(Debug)]
struct Position {
    height: u32,
    coordinate: Coordinate,
    distance: Option<u32>,
    prev_in_path: Option<Coordinate>,
    explored: bool,
}

impl Position {
    fn new(height: u32, row: usize, column: usize) -> Position {
        return Position {
            height: height,
            coordinate: Coordinate::new(row, column),
            distance: None,
            prev_in_path: None,
            explored: false,
        }
    }

    fn update_prev_in_path(&mut self, node_coord: Coordinate) {
        self.prev_in_path = Some(node_coord);
    }

    fn update_distance(&mut self, distance: u32) {
        self.distance = Some(distance);
    }

    fn mark_explored(&mut self) {
        self.explored = true;
    }
}

struct Map {
    positions: Vec<Vec<Position>>,
    start: Coordinate,
    end: Coordinate,
}

impl Map {
    fn get_map_length(&self) -> usize {
        return self.positions.len();
    }

    fn get_map_width(&self) -> usize {
        return self.positions[0].len();
    }

    fn get_node_from_coord(&mut self, coord: Coordinate) -> &mut Position {
        return &mut self.positions[coord.row][coord.column];
    }

    fn update_prev_in_path_for_coord(&mut self, coord: Coordinate, prev_node_coord: Coordinate) {
        self.get_node_from_coord(coord).update_prev_in_path(prev_node_coord);
    }

    fn update_distance_for_coord(&mut self, coord: Coordinate, distance: u32) {
        self.get_node_from_coord(coord).update_distance(distance);
    }

    fn mark_explored_for_coord(&mut self, coord: Coordinate) {
        self.get_node_from_coord(coord).mark_explored();
    }

    fn get_height_for_coord(&mut self, coord: Coordinate) -> u32 {
        return self.get_node_from_coord(coord).height;
    }  

    fn get_distance_for_coord(&mut self, coord: Coordinate) -> Option<u32> {
        return self.get_node_from_coord(coord).distance;
    }  

    fn get_prev_in_path_for_coord(&mut self, coord: Coordinate) -> Option<Coordinate> {
        return self.get_node_from_coord(coord).prev_in_path;
    }  

    fn get_explored_for_coord(&mut self, coord: Coordinate) -> bool {
        return self.get_node_from_coord(coord).explored;
    }  

    fn check_coord_available(&mut self, row: i32, column: i32) -> bool {
        let coord_exists = (row >= 0) & (column >= 0) & (row < (self.get_map_length() as i32)) & (column < (self.get_map_width() as i32));
        if coord_exists {
            let coord = Coordinate::new(row as usize, column as usize);
            return !self.get_explored_for_coord(coord);
        }
        return false;
    }
}


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);

    let mut map = initialise_map(file_name);
    let low_coords = get_lowest_coords(&mut map);
    let mut distances: Vec<u32> = Vec::new();
    let mut rc_map = Rc::new(map);
    for low_coord in low_coords {
        let distance_to_end: u32 = get_distance_to_end(rc_map.borrow_mut(), low_coord);
        distances.push(distance_to_end);
        println!("Distance from coord {:?}: {}", low_coord, distance_to_end);
    }
    println!("Shortest distance: {}", distances.iter().min().unwrap());
} 


fn initialise_map(file_name: &String) -> Map {
    let mut map = Vec::new();
    let mut start: Option<Coordinate> = None;
    let mut end: Option<Coordinate> = None;
    let mut row: usize = 0;
    let mut col: usize = 0;
    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(val) = line {
                let mut latitude = Vec::new();
                for height_char in val.chars() {
                    let height: u32;
                    let coord = Coordinate::new(row, col);
                    match height_char {
                        'S' => (height, start) = (0, Some(coord)),
                        'E' => (height, end) = (25, Some(coord)),
                        other => height = other as u32 - 'a' as u32,                        
                    };
                    latitude.push(Position::new(height, row, col));
                    col += 1;
                }
                map.push(latitude);
                row += 1;
                col = 0;
            }
        }
    }
    return Map {
        positions: map,
        start: start.expect("This should be initialised"), 
        end: end.expect("This should be initialised"),
    };
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_distance_to_end(mut map: &mut Rc<Map>, start: Coordinate) -> u32 {
    let mut current_coord: Coordinate = start;
    let mut queue: VecDeque<Coordinate> = VecDeque::new();
    let mut queued: HashSet<Coordinate> = HashSet::new();

    loop {
        let distance = match map.get_prev_in_path_for_coord(current_coord) {
            Some(coord) => map.get_distance_for_coord(coord).unwrap() + 1,
            None => 0,
        };
        map.update_distance_for_coord(current_coord, distance);

        let next_coords: Vec<Coordinate> = get_next_nodes(&mut map, &current_coord);
        for coord in next_coords {
            if !queued.contains(&coord) {
                map.update_prev_in_path_for_coord(coord, current_coord);
                queued.insert(coord);
                queue.push_back(coord);
            }
        }
        map.mark_explored_for_coord(current_coord);

        if (current_coord.row == map.end.row) & (current_coord.column == map.end.column) {
            break;
        }
        current_coord = queue.pop_front().unwrap();
    }
    return map.get_distance_for_coord(current_coord).unwrap();
}

fn get_lowest_coords(map: &mut Map) -> Vec<Coordinate> {
    let mut coords: Vec<Coordinate> = Vec::new();
    for i in 0..map.get_map_length() {
        for j in 0..map.get_map_width() {
            let this_coord = Coordinate::new(i, j);
            if map.get_height_for_coord(this_coord) == 0 {
                coords.push(this_coord);
            }
        }
    }
    return coords;
}

fn get_next_nodes(map: &mut Map, current_coord: &Coordinate) -> Vec<Coordinate> {
    let mut next_positions: Vec<Coordinate> = Vec::new();
    let max_height = map.get_height_for_coord(*current_coord) + 1;
    let curr_row = current_coord.row;
    let curr_col = current_coord.column;

    for direction in vec![(0, -1), (0, 1), (-1, 0), (1, 0)] {
        let attempt_row = curr_row as i32 + direction.0;
        let attempt_col = curr_col as i32 + direction.1;
        if map.check_coord_available(attempt_row, attempt_col) {
            let this_coord = Coordinate::new(attempt_row as usize, attempt_col as usize);
            if map.get_height_for_coord(this_coord) <= max_height {
                next_positions.push(this_coord);
            }
        }
    }
    return next_positions;
}
