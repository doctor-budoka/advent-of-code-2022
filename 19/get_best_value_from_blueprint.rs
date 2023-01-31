use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use structs_and_enums::{Blueprint, ResourceType, ResourceTally, State};


pub fn get_best_value_from_blueprint(blueprint: Blueprint, time_left: u32) -> u32 {
    let this_state: State = State::new(time_left);
    let mut previous_states: HashMap<State, Option<State>> = HashMap::from([(this_state, None)]);
    let shareable_blueprint: Rc<RefCell<Blueprint>> = Rc::new(RefCell::new(blueprint));
    dfs(this_state, &mut shareable_blueprint.as_ref().borrow_mut(), &mut previous_states);

    return shareable_blueprint.as_ref().borrow().get_best_value_from_blueprint();
}

fn dfs(state: State, blueprint: &mut Blueprint, previous_states: &mut HashMap<State, Option<State>>) {
    let new_states: Vec<State> = get_potential_states(&state, &blueprint);
    let shareable_blueprint: Rc<RefCell<&mut Blueprint>> = Rc::new(RefCell::new(blueprint));
    let current_best_geodes: u32 = shareable_blueprint.as_ref().borrow().get_best_value_from_blueprint();

    for next_state in &new_states {
        let potentially_more_geodes: bool = get_max_potential_geodes(next_state) >= current_best_geodes;
        if !previous_states.contains_key(next_state) && potentially_more_geodes {
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

fn get_max_potential_geodes(state: &State) -> u32 {
    let time_left: u32 = state.get_time_left();
    let current_geodes: u32 = state.get_resource_amount(&ResourceType::Geode);
    let current_geode_bots: u32 = state.get_num_robots(&ResourceType::Geode);
    let geodes_at_current_rate: u32 = current_geodes + (current_geode_bots * time_left);

    if time_left == 0 {
        return geodes_at_current_rate;
    }
    else {
        let max_potential_extra_geodes: u32 = time_left * (time_left - 1) / 2;
        return geodes_at_current_rate + max_potential_extra_geodes;
    }
}