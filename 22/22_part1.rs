use std::env;
use std::fs;

mod space;
use space::{Point,StdInt};
mod map;
use map::{Map,Tile};

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file"); 

    let mut map: Map = Map::new(); 
    let mut initial_point: Option<Point> = None;
    let mut read_map_state = true;
    for (i, line) in input.lines().enumerate() {
        if line.trim() == "" {
            read_map_state = false;
            continue;
        }
        else if read_map_state {
            for (j, tile_char) in line.chars().enumerate() {
                let this_tile = Tile::from_char(tile_char);
                if this_tile != Tile::None {
                    let this_point = Point::new(i as StdInt + 1, j as StdInt + 1);
                    map.add_point(this_point, this_tile);
                }
                if (this_tile == Tile::Clear) && (initial_point == None) {
                    initial_point = Some(Point::new(i as StdInt + 1, j as StdInt + 1));
                }
            }
        }
        else {
            continue;
        }
    }
    map.render_map();
    println!("Data loaded. Traversing map...");
} 
