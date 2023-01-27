use std::env;
use std::fs;
use std::collections::HashMap;

mod state_and_blueprints;
use state_and_blueprints::{Blueprint, ResourceType, ResourceTally};


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let blueprints = get_blueprints_from_input(input);
    println!("{:?}", blueprints);
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
