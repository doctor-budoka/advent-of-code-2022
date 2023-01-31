use std::env;
use std::fs;

mod structs_and_enums;
use structs_and_enums::Blueprint;
mod blueprints_from_input;
use blueprints_from_input::get_blueprints_from_input;
mod get_best_value_from_blueprint;
use get_best_value_from_blueprint::get_best_value_from_blueprint;


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    let time_left: u32 = env_args[2].parse().unwrap();
    println!("File name is '{}'. Reading input...", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let blueprints: Vec<Blueprint> = get_blueprints_from_input(input);

    let mut product: u32 = 1;
    for blueprint in blueprints {
        let bid: u32 = blueprint.get_index();
        println!("Looking at blueprint number {}", bid);
        let best_for_blueprint = get_best_value_from_blueprint(blueprint, time_left);
        println!("Best for blueprint {}: {}", bid, best_for_blueprint);
        product *= best_for_blueprint;
        if bid >= 3 {
            break;
        }
    }
    println!("Product of geode counts: {}", product);
} 
