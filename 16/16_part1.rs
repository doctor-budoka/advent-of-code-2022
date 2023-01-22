use std::env;
use std::fs;
use std::collections::HashMap;


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");

    let (vertices, valves) = read_input_to_hashmaps(input);

    println!("Vertices: {:?}", vertices);
    println!("Valves: {:?}", valves);

    let best_score: u32 = get_best_path_score(vertices, valves);
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

fn get_best_path_score(vertices: HashMap<String, Vec<String>>, valves: HashMap<String, u32>) -> u32 {
    return 5;
}
