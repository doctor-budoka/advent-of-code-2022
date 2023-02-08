use std::env;
use std::fs;

mod space;
use space::{Direction,Marker,Point,Rotation,StdInt};
mod face;
use face::Tile;
mod map;
use map::Map;
mod reading_input;
use reading_input::get_input_data;

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

