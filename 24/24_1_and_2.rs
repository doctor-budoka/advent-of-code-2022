use std::env;
use std::fs;
use std::cmp::Reverse;
use std::collections::{BinaryHeap,HashSet,HashMap};

mod valley;
use valley::{Tile,Valley};
mod space;
use space::{Direction,Point,StdInt};
mod path;
use path::PathPoint;

const DEBUG: bool = false;
const DISPLAY_PATH: bool = false;

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file"); 

    let mut valley = Valley::new();
    let mut start_option: Option<Point> = None;
    let mut end_option: Option<Point> = None;
    for (i, line) in input.lines().enumerate() {
        for (j, tile_char) in line.trim().chars().enumerate() {
            let this_point = Point::new(j as StdInt + 1, i as StdInt + 1);
            if (i == 0) & (tile_char == '.') {start_option = Some(this_point);}
            if (i > 0) & (tile_char == '.') {end_option = Some(this_point);}
            if tile_char == '.' {continue;}
            let this_point = Point::new(j as StdInt + 1, i as StdInt + 1);
            valley.add_tile(&this_point, &Tile::from_char(tile_char).unwrap());
        }
    }
    println!("Input read.");
    let start = start_option.expect("Starting point should have been found");
    let end = end_option.expect("Ending point should have been found");
    valley.render();
    println!("Start: {start}, End: {end}");

    let mut total_time: StdInt = 0;
    let shortest_1: PathPoint = find_shortest_path(start, end, valley.copy_valley(), 0);
    total_time += shortest_1.steps_from_start;
    println!("Shortest path avoiding blizzards (first part answer): {}", shortest_1.steps_from_start);
    let shortest_2: PathPoint = find_shortest_path(end, start, valley.copy_valley(), shortest_1.steps_from_start as usize);
    total_time += shortest_2.steps_from_start;
    println!("Path back to start to get snacks: {}", shortest_2.steps_from_start);
    let shortest_3: PathPoint = find_shortest_path(start, end, valley.copy_valley(), shortest_2.steps_from_start as usize);
    total_time += shortest_3.steps_from_start;
    println!("Shortest path avoiding blizzards with snacks: {}", shortest_1.steps_from_start);
    println!("Shortest total path (part 2 answer): {}", total_time);
}

fn find_shortest_path(start: Point, end: Point, valley: Valley, start_state: usize) -> PathPoint {
    let start_path = PathPoint::new_start(&start, start.distance(&end));
    let valley_states = get_all_valley_states(valley);
    let (previous_points, final_pathpoint) = search_for_shortest_path(&start_path, end, start_state, &valley_states);
    if DISPLAY_PATH {replay_path(&valley_states, previous_points, &final_pathpoint);}
    return final_pathpoint;
}

pub fn gcd(x: StdInt, y: StdInt) -> StdInt {
    let x_abs = x.abs();
    let y_abs = y.abs();
    if x == 0 {
        return y_abs;
    } 
    else if y == 0 {
        return x_abs;
    }
    if x == y {
        return x_abs;
    }
    let bigger: StdInt = if x_abs > y_abs {x_abs} else {y_abs};
    let smaller: StdInt = if x_abs > y_abs {y_abs} else {x_abs};

    let remainder: StdInt = bigger % smaller;
    return gcd(smaller, remainder);
}

fn get_all_valley_states(initial_valley: Valley) -> Vec<Valley> {
    let valley_width: StdInt = initial_valley.max_x.unwrap() - initial_valley.min_x.unwrap() - 1;
    let valley_height: StdInt = initial_valley.max_y.unwrap() - initial_valley.min_y.unwrap() - 1;
    let periodicity: StdInt = valley_height * valley_width / gcd(valley_height, valley_width);

    let mut states: Vec<Valley> = Vec::new();
    let mut current_state: Valley = initial_valley;
    for _ in 0..periodicity {
        states.push(current_state.copy_valley());
        current_state = current_state.move_blizzards();
    }
    return states;
}

fn search_for_shortest_path(start: &PathPoint, end: Point, initial_valley_state: usize, valley_states: &Vec<Valley>) -> (HashMap<(usize, Point), PathPoint>, PathPoint)  {
    let mut queue: BinaryHeap<Reverse<PathPoint>> = BinaryHeap::new();
    let mut queued: HashSet<(usize, Point)> = HashSet::new();
    let mut explored: HashSet<(usize, Point)> = HashSet::new();
    let mut previous: HashMap<(usize, Point), PathPoint> = HashMap::new();

    let mut current_best: Option<StdInt> = None;
    let mut current_best_path: Option<PathPoint> = None;

    let num_states: usize = valley_states.len();
    let mut current_node: PathPoint = PathPoint::new(&start.point, initial_valley_state as StdInt, start.point.distance(&end));
    loop {
        // Update the best path if necessary
        if current_node.point == end {
            match current_best {
                None => {
                    current_best = Some(current_node.steps_from_start);
                    current_best_path = Some(current_node);
                },
                Some(best) => {
                    if current_node.steps_from_start < best {
                        current_best = Some(current_node.steps_from_start);
                        current_best_path = Some(current_node);
                    }
                },
            }
        }
        else {
            let next_valley_state_ind: usize = (current_node.steps_from_start as usize + 1) % num_states;
            let next_valley_state: Valley = valley_states[next_valley_state_ind].copy_valley();
            let this_state: (usize, Point) = (current_node.steps_from_start as usize % num_states , current_node.point);
            let point_choices: Vec<Point> = get_point_choices(&current_node.point, &next_valley_state);
            for point in point_choices {
                let state: (usize, Point) = (next_valley_state_ind, point);
                if !queued.contains(&state) && !explored.contains(&state) {
                    let new_path_point = PathPoint::new(&point, &current_node.steps_from_start + 1, point.distance(&end));
                    previous.insert(state, current_node);
                    queued.insert(state);
                    queue.push(Reverse(new_path_point));
                }
                if DEBUG {
                    println!("Current step: {}", current_node.steps_from_start + 1);
                    next_valley_state.render_with_party_position(Some(state.1));
                    println!("{:?}, {:?}", this_state, state);
                    println!("{:?}", queue);
                }
            }
            explored.insert(this_state);
        }

        if queue.len() == 0 {
            break;
        }
        current_node = queue.pop().unwrap().0;
        if let Some(best) = current_best {
            if best <= current_node.estimated_path_length() {break;}
        }
    }
    return (previous, current_best_path.expect("We should have found at least one path to the end!"));
}

fn get_point_choices(current_point: &Point, next_state: &Valley) -> Vec<Point> {
    let mut choices: Vec<Point> = Vec::new();
    for direction in Direction::get_directions() {
        let vector_dir: Point = Point::from_direction(&direction);
        let new_point: Point = *current_point + vector_dir;

        match (next_state.map.get(&new_point), next_state.check_point_in_bounds(&new_point)) {
            (None, true) => choices.push(new_point),
            (_, _) => (),
        };
    }
    return choices;
}


fn replay_path(valley_states: &Vec<Valley>, previous_points: HashMap<(usize, Point), PathPoint>, final_pathpoint: &PathPoint) {
    let num_states: usize = valley_states.len();
    let path_points: Vec<Point> = get_path_as_points(previous_points, final_pathpoint, num_states as usize);
    println!("{:?}", &path_points);
    println!("Total length of path: {}", &path_points.len());
    for (ind, point) in path_points.iter().enumerate() {
        let valley_state_ind: usize = ind % num_states;
        let valley_state: Valley = valley_states[valley_state_ind].copy_valley();
        valley_state.render_with_party_position(Some(*point));
    }    
}

fn get_path_as_points(previous_points: HashMap<(usize, Point), PathPoint>, final_pathpoint: &PathPoint, num_valley_states: usize) -> Vec<Point> {
    let mut current_pt: PathPoint = *final_pathpoint;
    let mut output: Vec<Point> = vec![current_pt.point];

    loop {
        let current_ind: usize = (current_pt.steps_from_start as usize) % num_valley_states;
        if !previous_points.contains_key(&(current_ind, current_pt.point)) {
            break;
        }
        current_pt = previous_points[&(current_ind, current_pt.point)];
        output.push(current_pt.point);
    }
    output.reverse();
    return output;
}


