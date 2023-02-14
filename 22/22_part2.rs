use std::env;
use std::fs;
use std::collections::{HashMap,HashSet,VecDeque};

mod space;
use space::{Direction,Marker,Point,Rotation,StdInt};
mod space3d;
use space3d::{Direction3D,Orientation};
mod face;
use face::EdgeGlue;
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

    let total_instructions = &instructions.len();
    println!("Initial state: {}, num instructions: {}", current_marker, &total_instructions);
    println!("Data loaded. Traversing map...");

    for instruction in instructions.iter() {
        current_marker = match instruction.parse::<StdInt>() {
            Ok(distance) => map.get_new_position(&current_marker, distance),
            Err(_) => current_marker.get_rotated_marker(&Rotation::from_string(&instruction)),
        };
    }
    println!("Map traversed");
    println!("Final marker: {}", &current_marker);
    let current_point: Point = current_marker.get_position();
    let current_direction: Direction = current_marker.get_direction();
    let password: StdInt = (1000 * current_point.y) + (4 * current_point.x) + current_direction.as_int();
    println!("Password is {}", password);
} 


fn glue_faces(old_map: &Map) -> Map {
    let mut map = old_map.create_copy();
    println!("Glueing faces...");
    let max_face = map.find_face(&Point::new(map.get_max_x().unwrap(), map.get_max_y().unwrap()));
    let max_x = max_face.x;
    let max_y = max_face.y;
    for j in 1..=max_y {
        for i in 1..=max_x {
            let this_face: Point = Point::new(i, j);
            if !map.has_face(&this_face) || map.is_face_fully_glued(&this_face) {continue;}
            let neighbours: HashMap<Direction,EdgeGlue> = find_neighbours(&map, &this_face);
            for (direction, (neighbour, rotation)) in neighbours.iter() {
                map.bidirectional_glue_faces(&this_face, &neighbour, &direction, &rotation);
            }
        }
    }
    return map;
}


fn find_neighbours(map: &Map, start: &Point) -> HashMap<Direction, EdgeGlue> {
    let (previous_in_path, d3d_to_face) = dfs_for_rotations(map, start);

    let mut edge_glues: HashMap<Direction, EdgeGlue> = HashMap::new();
    for direction in map.get_faces_unglued_directions(start) {
        let direction3d: Direction3D = Direction3D::from2d_as_face_direction_rel_z(&direction);
        let face: Point = *d3d_to_face.get(&direction3d).unwrap();

        let new_tangent = get_new_tangent(start, &face, &direction3d, &previous_in_path);
        let current_tangent: Direction3D = Direction3D::from2d_as_face_direction_rel_z(&direction);
        let face_rotation: Rotation = Direction3D::get_2d_rotation_from_tangent_change_on_z_face(&current_tangent, &new_tangent);
        edge_glues.insert(direction, (face, face_rotation));
    }
    return edge_glues;
}

fn dfs_for_rotations(map: &Map, start: &Point) 
    -> (HashMap<Point,(Point,Direction3D)>, HashMap<Direction3D,Point>) {    
    let mut from_previous: HashMap<Point,(Point,Direction3D)> = HashMap::new();
    let mut orientation_from_face: HashMap<Point,Orientation> = HashMap::new();
    let mut d3d_to_face: HashMap<Direction3D,Point> = HashMap::from([(Direction3D::Z, *start)]);

    let mut queue: VecDeque<Point> = VecDeque::new();
    let mut queued: HashSet<Point> = HashSet::new();
    let mut explored: HashSet<Point> = HashSet::new();
    let mut current_face: Point = *start;
    let mut current_orientation: Orientation = Orientation::new();

    loop {
        let next_faces: Vec<(Point,Direction)> = get_flat_neighbour_faces(map, &current_face);
        for (face, direction) in next_faces {
            if !queued.contains(&face) && !explored.contains(&face) {
                let new_rotation = current_orientation.get_rotation_axis_for_direction(&direction);
                let new_orientation = current_orientation.rotate(&new_rotation);
                orientation_from_face.insert(face, new_orientation);
                d3d_to_face.insert(new_orientation.get_face_direction(), face);
                from_previous.insert(face, (current_face, new_rotation));
                queued.insert(face);
                queue.push_back(face);
            }
        }
        explored.insert(current_face);

        if (explored.len() == 6) || (queue.len() == 0) {
            break;
        }
        current_face = queue.pop_front().unwrap();
        current_orientation = *orientation_from_face.get(&current_face).unwrap();
    }
    return (from_previous, d3d_to_face);
}

fn get_flat_neighbour_faces(map: &Map, current_face: &Point) -> Vec<(Point, Direction)> {
    let mut neighbours: Vec<(Point, Direction)> = Vec::new();
    for direction in Direction::get_directions() {
        let new_flat_position = *current_face + direction.as_vector();
        if map.has_face(&new_flat_position) {
            neighbours.push((new_flat_position, direction));
        }
    }
    return neighbours;
}

fn get_new_tangent(start: &Point, target: &Point, target_as_3d: &Direction3D, previous_steps: &HashMap<Point,(Point,Direction3D)>) -> Direction3D {
    let mut rotations: Vec<Direction3D> = Vec::new();
    let mut current_face = target;
    while current_face != start {
        let (prev_face, rotation) = previous_steps.get(&current_face).unwrap();
        rotations.push(*rotation);
        current_face = prev_face;
    }
    let mut new_tangent: Direction3D = *target_as_3d;
    for rotation in rotations.iter().rev() {
        new_tangent = new_tangent.rotate_around(&rotation);
    }
    return new_tangent;
}
