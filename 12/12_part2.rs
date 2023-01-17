use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{VecDeque, HashSet, HashMap};
use std::rc::Rc;
use std::cell::RefCell;


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


struct Position {
    height: u32,
}

impl Position {
    fn new(height: u32) -> Position {
        return Position {
            height: height,
        }
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

    fn get_height_for_coord(&mut self, coord: Coordinate) -> u32 {
        return self.get_node_from_coord(coord).height;
    }

    fn check_coord_available(&mut self, row: i32, column: i32) -> bool {
        let coord_exists = (row >= 0) & (column >= 0) & (row < (self.get_map_length() as i32)) & (column < (self.get_map_width() as i32));
        return coord_exists;
    }
}


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);

    let mut map = initialise_map(file_name);
    let low_coords = get_lowest_coords(&mut map);
    let mut distances: Vec<u32> = Vec::new();
    let rc_map = Rc::new(RefCell::new(map));
    for low_coord in low_coords {
        println!("Trying coord {:?}", low_coord);
        rc_map.as_ref().borrow_mut().start = low_coord;
        let distance_to_end: Option<u32> = get_distance_to_end(&mut rc_map.as_ref().borrow_mut());
        if let Some(num) = distance_to_end {
            distances.push(num);
            println!("Distance from coord {:?}: {}", low_coord, num);
        }
        else {
            println!("Can't reach the end from {:?}", low_coord)
        }
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
                    latitude.push(Position::new(height));
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

fn get_distance_to_end(map: &mut Map) -> Option<u32> {
    let mut distances: HashMap<Coordinate,u32> = HashMap::new();
    let mut prev_in_path: HashMap<Coordinate,Coordinate> = HashMap::new();

    let mut current_coord: Coordinate = map.start;
    let mut queue: VecDeque<Coordinate> = VecDeque::new();
    let mut queued: HashSet<Coordinate> = HashSet::new();
    let mut explored: HashSet<Coordinate> = HashSet::new();
    
    loop {
        let distance = match prev_in_path.get(&current_coord) {
            Some(coord) => distances.get(&coord).unwrap() + 1,
            None => 0,
        };
        distances.insert(current_coord, distance);

        let next_coords: Vec<Coordinate> = get_next_nodes(map, &current_coord);
        for coord in next_coords {
            if !queued.contains(&coord) & !explored.contains(&coord) {
                prev_in_path.insert(coord, current_coord);
                queued.insert(coord);
                queue.push_back(coord);
            }
        }
        explored.insert(current_coord);

        if current_coord == map.end {
            break;
        }
        match queue.pop_front() {
            Some(coord) => current_coord = coord,
            None => break,
        };
    }
    return match distances.get(&map.end) {None=>None, Some(ref_num) => Some(*ref_num)};
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
