use std::env;
use std::fs;
use std::collections::{HashMap, VecDeque, HashSet};


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");

    let (vertices, valves) = read_input_to_hashmaps(input);

    println!("Vertices: {:?}", vertices);
    println!("Valves: {:?}", valves);

    let best_score: u32 = get_best_score(vertices, valves);
    println!("Most pressure released: {}", best_score);
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

fn get_best_score(vertices: HashMap<String, Vec<String>>, valves: HashMap<String, u32>) -> u32 {
    let all_distances = get_distances_between_nodes(vertices);
    let (new_nodes, new_distances) = reduce_graph("AA".to_string(), all_distances, valves.keys().cloned().collect());
    println!("New nodes: {:?}", new_nodes);
    println!("Distances: {:?}", new_distances);
    return 5;
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

fn reduce_graph(source: String, distances: HashMap<(String, String), u32>, mut valves: Vec<String>) -> (Vec<String>, HashMap<(String, String), u32>) {
    let mut edge_distances: HashMap<(String, String), u32> = HashMap::new();
    valves.push(source);
    for node1 in &valves {
        for node2 in &valves {
            if node1 == node2 {continue;}
            // Adding one because this is the "distance" to turn on a valve
            edge_distances.insert((node1.to_string(), node2.to_string()), distances.get(&(node1.to_string(), node2.to_string())).unwrap() + 1);
        }
    }
    return (valves, edge_distances);
}
