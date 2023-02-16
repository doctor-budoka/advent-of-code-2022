use std::env;
use std::fs;

mod space;
use space::{Point,StdInt};
mod elves;
use elves::Elves;


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file"); 

    let mut elves = Elves::new();
    for (i, line) in input.lines().enumerate() {
        for (j, tile_char) in line.trim().chars().enumerate() {
            if tile_char == '.' {continue;}
            let this_point = Point::new(j as StdInt + 1, i as StdInt + 1);
            elves.add_elf(&this_point);
        }
    }
    println!("Input read. Simulating elf moves...");

    let mut at_stationary_state = false;
    let mut num_rounds = 0;
    while !at_stationary_state  {
        at_stationary_state = elves.run_round();
        num_rounds += 1;
    }
    println!("Number of rounds to get to stationary state: {}", num_rounds);
} 
