use std::env;
use std::fs;
use std::collections::HashSet;

mod space;
use space::{Direction,Marker,Point,Rotation,StdInt};
mod face;
use face::Tile;
mod map;
use map::Map;

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    let face_size: StdInt = env_args[2].parse().unwrap();
    println!("File name is '{}'. Reading input...", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file"); 
    let (map, instructions, mut current_marker): (Map, Vec<String>, Marker) = get_input_data(input, face_size);

    println!("Initial state: {}", current_marker);
    println!("Data loaded. Traversing map...");

    for instruction in instructions {
        current_marker = match instruction.parse::<StdInt>() {
            Ok(distance) => map.get_new_position(&current_marker, distance),
            Err(_) => current_marker.get_rotated_marker(&Rotation::from_string(&instruction)),
        }
    }
    println!("Final marker: {}", &current_marker);
    let current_point: Point = current_marker.get_position();
    let current_direction: Direction = current_marker.get_direction();
    let password: StdInt = (1000 * current_point.y) + (4 * current_point.x) + current_direction.as_int();
    println!("Password is {}", password);
} 


fn get_input_data(input: String, size: StdInt) -> (Map, Vec<String>, Marker) {
    let digits: HashSet<char> = HashSet::from(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    let mut map: Map = Map::new(size); 
    let mut initial_point: Option<Point> = None;
    let mut read_map_state = true;
    let mut instructions: Vec<String> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if line.trim() == "" {
            read_map_state = false;
            continue;
        }
        else if read_map_state {
            for (j, tile_char) in line.chars().enumerate() {
                if tile_char == ' ' {
                    continue;
                }
                let this_tile = Tile::from_char(tile_char);
                let this_point = Point::new(j as StdInt + 1, i as StdInt + 1);
                map.add_point(&this_point, &this_tile);

                if (this_tile == Tile::Clear) && (initial_point == None) {
                    initial_point = Some(Point::new(j as StdInt + 1, i as StdInt + 1));
                }
            }
        }
        else {
            let mut curr_string: String = "".to_string();
            for instruction_char in line.chars() {
                if digits.contains(&instruction_char) {
                    curr_string.push(instruction_char);
                }
                else {
                    instructions.push(curr_string);
                    instructions.push(instruction_char.to_string());
                    curr_string = "".to_string();
                }
            }
            if curr_string.len() > 0 {
                instructions.push(curr_string);
            }
        }
    }
    return (map, instructions, Marker::new(initial_point.unwrap(), Direction::Right));
}
