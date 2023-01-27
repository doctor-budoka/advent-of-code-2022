use std::env;
use std::fs;
use std::collections::{HashMap, VecDeque, HashSet};

mod state_and_blueprints;
use state_and_blueprints::{Blueprint, ResourceType, ResourceTally};


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let blueprints = get_blueprints_from_input(input);
    println!("{:?}", blueprints);

    let mut total_quality: u32 = 0;
    for blueprint in blueprints {
        let bid: u32 = blueprint.index;
        let best_for_blueprint = get_best_value_from_blueprint(blueprint);
        total_quality += bid * best_for_blueprint;
    }
    println!("Total quality: {}", total_quality);
} 

fn get_blueprints_from_input(input: String) -> Vec<Blueprint> {
    let mut blueprints: Vec<Blueprint> = Vec::new();
    for line in input.trim().lines().collect::<Vec<&str>>() {
        let parts: Vec<String> = line.trim().split(": ").map(|x| x.to_string()).collect();
        let blueprint_index: u32 = parts[0].replace("Blueprint ", "").parse().unwrap();

        let robot_recipes_in_words: Vec<String> = parts[1].split(". ").map(|x| x.replace(".", "").to_string()).collect();
        let mut blueprint_hashmap: HashMap<ResourceType, ResourceTally> = HashMap::new();
        for robot_recipe in &robot_recipes_in_words {
            let mut resource_tally: ResourceTally = ResourceTally::new();
            let words: Vec<String> = robot_recipe.split_whitespace().map(|x| x.to_string()).collect();
            let robot_type: ResourceType = ResourceType::from_string(&words[1]).unwrap();
            let resource_amount: u32 = words[4].parse().unwrap();
            let resource_type: ResourceType = ResourceType::from_string(&words[5]).unwrap();
            
            resource_tally.update_resource(resource_type, resource_amount);
            if words.len() > 6 {
                let resource_amount: u32 = words[7].parse().unwrap();
                let resource_type: ResourceType = ResourceType::from_string(&words[8]).unwrap();
                
                resource_tally.update_resource(resource_type, resource_amount);
            }
            blueprint_hashmap.insert(robot_type, resource_tally);
        }
        let blueprint = Blueprint::new(blueprint_index, blueprint_hashmap);
        blueprints.push(blueprint);
    }
    return blueprints;
}


fn get_best_value_from_blueprint(blueprint: Blueprint) {
    let mut this_state: State = State::new();
    let mut states_done: HashSet<State> = HashSet::new();
    let mut queued: HashSet<State> = HashSet::new();
    let mut queue: VecDeque<State> = Vec::new();
    let mut max_geode:u32 = 0;

    loop {
        let new_states: Vec<State> = get_potential_states(this_state, blueprint);
        for next_state in &new_states {
            let already_explored: bool = states_done.contains(&next_state);
            let already_queued: bool = queued.contains(&next_state);
            if !already_explored && !already_queued {
                queued.insert(next_state);
                queue.push_back(next_state);
            }
        }
        max_geode = max(max_geode, this_state.resources.geode);
        states_done.insert(this_state);

        match queue.pop_front() {
            Some(new_state) => this_state = new_state,
            None => break,
        };
    }
    return max_geode;
}

fn potential_states(current_state: &State, blueprint: &Blueprint) -> Vec<State> {
    let mut robots_to_produce = Vec<ResourceTally>: Vec::new();

    let mut explored: HashSet<ResourceTally> = HashSet::new();
    let mut queued: HashSet<ResourceTally> = HashSet::new();
    let mut queue: VecDeque<ResourceTally> = Vec::new();
    let mut resources_lookup: HashMap<ResourceTally, ResourceTally> = HashMap::new();

    let current_resources = current_state.resources.copy_tally();
    let mut this_robot_tally: ResourceTally = ResourceTally::new();
    resources_lookup.insert(this_tally, current_resources);
    loop {
        let this_resources = resources_lookup.get(&this_tally);
        let choices = potential_new_robots(this_resources, blueprint);
        for choice in choices {
            let resources_left = this_resources - blueprint.get(choice).unwrap();
            let new_tally = this_robot_tally.new_tally_with_added_resource(choice, 1);
            if !queued.contains(new_tally) and !explored.contains(new_tally) {
                resources_lookup.insert(new_tally, resources_left);
                queued.insert(new_tally);
                queue.push_back(new_tally);
            }
        }
        explored.insert(this_robot_tally)

        match queue.pop_front() {
            Some(new_robot_tally) => this_robot_tally = new_robot_tally,
            None => break,
        }
    }
}

fn potential_new_robots(resources: &ResourceTally, blueprint: &Blueprint) -> Vec<ResourceType> {
    let mut output = Vec::new();
    for resource_type in ResourceType::iter() {
        let cost = Blueprint.get(&resource_type).unwrap();
        if cost < current_resources {
            output.push(resource_type);
        }
    }
    return output;
}
