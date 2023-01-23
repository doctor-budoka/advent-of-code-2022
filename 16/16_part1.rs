use std::env;
use std::fs;
use std::collections::{HashMap, VecDeque, HashSet};
use std::iter::FromIterator;

const MAX_TIME: u32 = 30;

#[derive(Debug)]
struct Path {
    current_node: String,
    time_elapsed: u32,
    current_score: u32,
    pressure: u32,
    path: Vec<String>,
}

impl Path {
    fn new(node: &String, time_elapsed: u32, current_score: u32, pressure: u32, path: &Vec<String>) -> Path {
        return Path {
            current_node: node.to_string(),
            time_elapsed: time_elapsed,
            current_score: current_score,
            pressure: pressure,
            path: path.iter().map(|x| x.to_string()).collect(),
        }
    }
    
    fn get_current_node(&self) -> String {
        return self.current_node.to_string();
    }

    fn get_copy(&self) -> Path {
        let path_copy: Vec<String> = self.path.iter().map(|x| x.to_string()).collect();
        return Path::new(&self.current_node, self.time_elapsed, self.current_score, self.pressure, &path_copy);
    }
    
    fn get_updated_path(&self, new_node: &String, valve_pressure: u32, distance: u32) -> Path {
        let mut new_path: Vec<String> =  self.path.iter().map(|x| x.to_string()).collect();
        new_path.push((&new_node).to_string());
        return Path::new(
            new_node,
            self.time_elapsed + distance,
            self.current_score + (distance * self.pressure), 
            self.pressure + valve_pressure,
            &new_path,
        )
    }

    fn nodes_visited(&self) -> HashSet<String> {
        return HashSet::from_iter(self.path.iter().map(|x| x.to_string()));
    }

    fn score_at_time(&self) -> u32 {
        let time_left: u32 = MAX_TIME - self.time_elapsed;
        return self.current_score + (time_left * self.pressure);
    }
}


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");

    let (vertices, valves) = read_input_to_hashmaps(input);
    println!("Input read in:");
    println!("Vertices: {:?}", vertices);
    println!("Valves: {:?}\n", valves);

    let best_path: Path = get_best_score(vertices, valves);
    println!("Winning path: {:?}", best_path);
    println!("Most pressure released: {}", best_path.score_at_time());
} 

fn read_input_to_hashmaps(input: String) -> (HashMap<String, Vec<String>>, HashMap<String, u32>) {
    let mut vertices: HashMap<String, Vec<String>> = HashMap::new();
    let mut valves: HashMap<String, u32> = HashMap::new();
    for line in input.trim().lines().collect::<Vec<&str>>() {
        let this_node: String = line.split(" ").nth(1).unwrap().trim().to_string();
        let flow_rate: u32 = {
            line.split(";").nth(0).unwrap()
            .split(" ").nth(4).unwrap().trim().to_string()
            .split("=").nth(1).unwrap().to_string()
            .parse::<u32>().unwrap()
        };
        let edges_part: String = line.split("; ").nth(1).unwrap().to_string();
        let edge_nodes: Vec<String>;
        if edges_part.contains(",") {
            edge_nodes = {
                edges_part.split(" valves ").nth(1).unwrap()
                .split(", ").map(|x| x.to_string()).collect()
            };
        }
        else {
            edge_nodes = {
                edges_part.split(" valve ").nth(1).unwrap()
                .split(", ").map(|x| x.to_string()).collect()
            };
        }

        vertices.insert(this_node.to_string(), edge_nodes);
        if flow_rate > 0 {
            valves.insert(this_node, flow_rate);
        }
    }
    return (vertices, valves);
}

fn get_best_score(vertices: HashMap<String, Vec<String>>, valves: HashMap<String, u32>) -> Path {
    let all_distances = get_distances_between_nodes(vertices);
    let new_distances = reduce_graph("AA".to_string(), all_distances, valves.keys().cloned().collect());
    
    return dfs_for_best(new_distances, valves);
}

fn get_distances_between_nodes(vertices: HashMap<String, Vec<String>>) -> HashMap<(String, String), u32> {
    let mut shortest_paths: HashMap<(String, String), u32> = HashMap::new();
    for source in vertices.keys() {
        let distances_from_node = dijkstra(source.to_string(), &vertices);
        for (target, distance) in distances_from_node {
            shortest_paths.insert((source.to_string(), target), distance);
        }
    }
    return shortest_paths;
}

fn dijkstra(source: String, vertices: &HashMap<String, Vec<String>>) -> HashMap<String, u32> {
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut queued: HashSet<String> = HashSet::new();
    let mut explored: HashSet<String> = HashSet::new();

    let mut previous_node: HashMap<String, Option<String>> = HashMap::new();
    let mut distances: HashMap<String, u32> = HashMap::new();
    let mut current_node = source;
    previous_node.insert((&current_node).to_string(), None);
    loop {
        let distance = match previous_node.get(&current_node).unwrap() {
            Some(prev_node) => distances.get(&prev_node.to_string()).unwrap() + 1,
            None => 0,
        };
        distances.insert((&current_node).to_string(), distance);

        let next_nodes: Vec<String> = vertices.get(&current_node).unwrap().to_vec();
        for node in next_nodes {
            if !queued.contains(&node) && !explored.contains(&node) {
                previous_node.insert((&node).to_string(), Some((&current_node).to_string()));
                queued.insert((&node).to_string());
                queue.push_back(node);
            }
        }
        explored.insert(current_node);

        if explored.len() == vertices.keys().len() {
            break;
        }
        current_node = queue.pop_front().unwrap();
    }
    return distances;
}

fn reduce_graph(source: String, distances: HashMap<(String, String), u32>, mut valves: Vec<String>) -> HashMap<(String, String), u32> {
    let mut edge_distances: HashMap<(String, String), u32> = HashMap::new();
    valves.push(source);
    for node1 in &valves {
        for node2 in &valves {
            if node1 == node2 {continue;}
            // Adding one because this is the "distance" to turn on a valve
            edge_distances.insert((node1.to_string(), node2.to_string()), distances.get(&(node1.to_string(), node2.to_string())).unwrap() + 1);
        }
    }
    return edge_distances;
}

fn dfs_for_best(edge_weights: HashMap<(String, String), u32>, valves: HashMap<String, u32>) -> Path {
    let start = "AA".to_string();
    let mut visited = HashSet::new();
    visited.insert((&start).to_string());
    let path = Path::new(&start, 0, 0, 0, &vec!["AA".to_string()]);
    return dfs(path, &edge_weights, &valves);
}

fn dfs(current_path: Path, edge_weights: &HashMap<(String, String), u32>, valves: &HashMap<String, u32>) -> Path {
    let current_node = current_path.get_current_node();
    let mut best_path: Path = current_path.get_copy();
    let time_left: u32 = MAX_TIME - best_path.time_elapsed;
    for node in valves.keys() {
        if current_path.nodes_visited().contains(node) {continue;}
        let distance = *edge_weights.get(&(current_node.to_string(), node.to_string())).unwrap();
        if distance > time_left {continue;}
        
        let extra_pressure = *valves.get(node).unwrap();
        let new_path_to_attempt = current_path.get_updated_path(&node, extra_pressure, distance);
        let new_path = dfs(new_path_to_attempt, edge_weights, valves);
        if new_path.score_at_time() > best_path.score_at_time() {best_path = new_path;}
    }
    return best_path;
}
