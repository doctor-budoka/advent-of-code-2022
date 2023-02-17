use std::env;
use std::fs;

mod valley;
use valley::{Tile,Valley};
mod space;
use space::{Point,StdInt};

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file"); 

    let mut valley = Valley::new();
    for (i, line) in input.lines().enumerate() {
        for (j, tile_char) in line.trim().chars().enumerate() {
            if tile_char == '.' {continue;}
            let this_point = Point::new(j as StdInt + 1, i as StdInt + 1);
            valley.add_tile(&this_point, &Tile::from_char(tile_char).unwrap());
        }
    }
    println!("Input read.");
    valley.render();
} 
