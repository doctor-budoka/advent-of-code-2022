use std::env;
use std::fs;
use std::collections::HashMap;

mod space;
use space::{Direction,Marker,Point,Rotation,StdInt};
mod face;
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
    let (unglued_map, instructions, mut current_marker): (Map, Vec<String>, Marker) = get_input_data(input, face_size);
    let map = glue_faces(&unglued_map);

    let mut marker_trail: HashMap<Point,Direction> = HashMap::new();
    println!("Initial state: {}, num instructions: {}", current_marker, &instructions.len());
    println!("Data loaded. Traversing map...");

    for instruction in instructions {
        match instruction.parse::<StdInt>() {
            Ok(distance) => {
                let (new_marker, new_trail) = map.get_new_position_with_trail(&current_marker, distance);
                current_marker = new_marker;
                marker_trail.extend(new_trail);
            },
            Err(_) => {
                current_marker = current_marker.get_rotated_marker(&Rotation::from_string(&instruction));
                marker_trail.insert(current_marker.get_position(), current_marker.get_direction());
            },
        };
        // println!("Instruction: {}, marker: {}", instruction, current_marker);
        // map.render_map_with_marker(&current_marker);
    }
    println!("Map traversed");
    println!("Final marker: {}", &current_marker);
    let current_point: Point = current_marker.get_position();
    let current_direction: Direction = current_marker.get_direction();
    let password: StdInt = (1000 * current_point.y) + (4 * current_point.x) + current_direction.as_int();
    println!("Password is {}", password);
    println!("Rendering map...");
    map.render_map_with_trail(marker_trail);
} 


fn glue_faces(old_map: &Map) -> Map {
    let mut map = old_map.create_copy();
    println!("Gluing faces...");
    let max_face = map.find_face(&Point::new(map.get_max_x().unwrap(), map.get_max_y().unwrap()));
    let max_x = max_face.x;
    let max_y = max_face.y;
    // println!("{}", max_face);
    for j in 1..=max_y {
        for i in 1..=max_x {
            let this_face: Point = Point::new(i, j);
            // println!("{}", this_face);
            if !map.has_face(&this_face) {continue;}
            let neighbours: HashMap<Direction,Point> = find_neighbours(&map, &this_face);
            // println!("{:?}", neighbours);
            for (direction, neighbour) in neighbours.iter() {
                map.bidirectional_glue_faces(&this_face, &neighbour, &direction, &Rotation::None);
            }
            // println!("face: {} is_glued: {}", this_face, map.is_face_fully_glued(&this_face));
        }
    }
    println!("Faces glued");
    return map;
}

fn find_neighbours(map: &Map, face: &Point) -> HashMap<Direction, Point> {
    let mut neighbours: HashMap<Direction,Point> = HashMap::new();
    for direction in Direction::get_directions() {
        let neighbour = find_neighbour_in_direction(map, face, &direction);
        neighbours.insert(direction, neighbour);
    }
    return neighbours;
}

fn find_neighbour_in_direction(map: &Map, face: &Point, direction: &Direction) -> Point {
    let direction_vector: Point = direction.as_vector();
    let first_attempt: Point = *face + direction_vector;
    if map.has_face(&first_attempt) {
        return first_attempt;
    }
    else {
        let reverse_direction: Point = direction.inverse().as_vector();
        let mut prev_face: Point = *face;
        let mut this_face: Point = *face + reverse_direction;
        while map.has_face(&this_face) {
            prev_face = this_face;
            this_face += reverse_direction;
        }
        return prev_face;
    }
}
