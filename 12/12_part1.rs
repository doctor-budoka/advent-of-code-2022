use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
}


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);

    let (mut map, start, end) = initialise_map(file_name);  
} 


fn initialise_map(file_name: &String) -> (Vec<Vec<Position>>, Coordinate, Coordinate) {
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
                        'E' => (height, end) = (26, Some(coord)),
                        other => height = other as u32 - 'a' as u32,                        
                    };
                    latitude.push(Position::new(height, row, col));
                    col += 1;
                }
                map.push(latitude);
                row += 1;
            }
        }
    }
    return (map, start.expect("This should be initialised"), end.expect("This should be initialised"));
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

