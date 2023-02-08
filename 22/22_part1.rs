use std::env;
use std::fs;
use std::collections::HashSet;

mod space;
use space::{Direction,Point,Rotation,StdInt};
mod map;
use map::{Map,Tile};

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file"); 
    let (map, instructions, mut current_point, mut current_direction): (Map, Vec<String>, Point, Direction) = get_input_data(input);

    map.render_map();
    println!("Instructions: {:?}, Initial point: {}, Initial direction: '{:?}'", instructions, current_point, current_direction);
    println!("Data loaded. Traversing map...");

    for instruction in instructions {
        match instruction.parse::<StdInt>() {
            Ok(distance) => current_point = map.get_new_position(&current_point, &current_direction, distance),
            Err(_) => current_direction = current_direction.rotate(Rotation::from_string(&instruction)),
        }
        println!("Instruction: {}, new state: {}, {:?}", &instruction, &current_point, &current_direction);
        map.render_map_with_current_position(&current_point, &current_direction);
    }
    println!("Final position: {}, Final direction: {:?}", &current_point, &current_direction);
    let password: StdInt = (1000 * current_point.y) + (4 * current_point.x) + current_direction.as_int();
    println!("Password is {}", password);
} 


fn get_input_data(input: String) -> (Map, Vec<String>, Point, Direction) {
    let digits: HashSet<char> = HashSet::from(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    let mut map: Map = Map::new(); 
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
                let this_tile = Tile::from_char(tile_char);
                if this_tile != Tile::None {
                    let this_point = Point::new(j as StdInt + 1, i as StdInt + 1);
                    map.add_point(this_point, this_tile);
                }
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
    return (map, instructions, initial_point.unwrap(), Direction::Right);
}
