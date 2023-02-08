use std::collections::HashSet;

use space::{Direction,Marker,Point,StdInt};
use face::Tile;
use map::Map;

pub fn get_input_data(input: String, size: StdInt) -> (Map, Vec<String>, Marker) {
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
