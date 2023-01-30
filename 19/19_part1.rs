use std::env;
use std::fs;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

mod state_and_blueprints;
use state_and_blueprints::{Blueprint, ResourceType, ResourceTally, State};


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let blueprints = get_blueprints_from_input(input);

    let mut total_quality: u32 = 0;
    for blueprint in blueprints {
        let bid: u32 = blueprint.get_index();
        let best_for_blueprint = get_best_value_from_blueprint(blueprint);
        println!("{}: {}", bid, best_for_blueprint);
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

fn get_best_value_from_blueprint(blueprint: Blueprint) -> u32 {
    let this_state: State = State::new();
    let mut previous_states: HashMap<State, Option<State>> = HashMap::from([(this_state, None)]);
    let shareable_blueprint: Rc<RefCell<Blueprint>> = Rc::new(RefCell::new(blueprint));
    dfs(this_state, &mut shareable_blueprint.as_ref().borrow_mut(), &mut previous_states);

    return shareable_blueprint.as_ref().borrow().get_best_value_from_blueprint();
}

fn dfs(state: State, blueprint: &mut Blueprint, previous_states: &mut HashMap<State, Option<State>>) {
    let new_states: Vec<State> = get_potential_states(&state, &blueprint);
    let shareable_blueprint: Rc<RefCell<&mut Blueprint>> = Rc::new(RefCell::new(blueprint));
    println!("{:?}", &state);
    for next_state in &new_states {
        if !previous_states.contains_key(next_state) {
            dfs(*next_state, &mut shareable_blueprint.as_ref().borrow_mut(), previous_states);
            previous_states.insert(*next_state, Some(state));
        }
    }
    blueprint.update_max_geode_count_for_blueprint(state.get_resource_amount(&ResourceType::Geode));
}

fn get_potential_states(current_state: &State, blueprint: &Blueprint) -> Vec<State> {
    let mut potential_states: Vec<State> = Vec::new();
    if current_state.get_time_left() == 0 {
        return potential_states;
    }
    let potential_robots_to_build = get_potential_new_robots(&current_state.get_current_resources(), blueprint);
    let unchanged_state = current_state.create_updated_state(ResourceTally::new(), ResourceTally::new());

    potential_states.push(unchanged_state);
    for new_robot_type in potential_robots_to_build {
        let new_tally = ResourceTally::new().new_tally_with_added_resource(&new_robot_type, 1);
        let new_state = current_state.create_updated_state(new_tally, blueprint.get_total_cost(&new_tally));
        if (new_robot_type == ResourceType::Geode) || (new_state.get_num_robots(&new_robot_type) <= blueprint.get_max_resource_cost(&new_robot_type)) {
            potential_states.push(new_state);
        }
    }
    return potential_states;
}

fn get_potential_new_robots(resources: &ResourceTally, blueprint: &Blueprint) -> Vec<ResourceType> {
    let mut output = Vec::new();
    for resource_type in ResourceType::resource_types() {
        let cost = blueprint.get_costs(&resource_type);
        if cost <= *resources {
            output.push(resource_type);
        }
    }
    return output;
}

fn max_potential_geodes(state: &State) -> u32 {
    let time_left: u32 = state.get_time_left();
    let current_geodes: u32 = state.get_resource_amount(&ResourceType::Geode);
    let current_geode_bots: u32 = state.get_num_robots(&ResourceType::Geode);
    let geodes_at_current_rate: u32 = current_geodes + (current_geode_bots * time_left);

    let max_potential_extra_geodes: u32 = time_left * (time_left - 1) / 2;
    return geodes_at_current_rate + max_potential_extra_geodes;
}
