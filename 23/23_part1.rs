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
    elves.render_map();
    println!("Input read. Simulating elf moves...");

    for _ in 0..10 {
        elves.run_round();
        elves.render_map();
    }

} 
