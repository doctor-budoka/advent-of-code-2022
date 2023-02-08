use std::collections::HashMap;
use std::io::stdout;
use std::io::Write;

use space::{Point,StdInt};

#[derive(PartialEq,Copy,Clone)]
pub enum Tile {
    Stone,
    Clear,
    None,
}

impl Tile {
    pub fn from_char(character: char) -> Self {
        return match character {
            '#' => Self::Stone,
            '.' => Self::Clear,
            ' ' => Self::None,
            other => panic!("'{}' is not a valid tile character", other),
        }
    }

    pub fn to_char(&self) -> char {
        return match self {
            Self::Stone => '#',
            Self::Clear => '.',
            Self::None => ' ',
        }
    }
}

pub struct Map {
    places: HashMap<Point, Tile>,
    max_x: Option<StdInt>,
    max_y: Option<StdInt>,
}

impl Map {
    pub fn new() -> Self {
        return Map {places: HashMap::new(), max_x: None, max_y: None};
    }

    pub fn add_point(&mut self, point: Point, tile: Tile) {
        self.places.insert(point, tile);
        if (self.max_x == None) || (point.x > self.max_x.unwrap()) {
            self.max_x = Some(point.x);
        }
        if (self.max_y == None) || (point.y > self.max_y.unwrap()) {
            self.max_y = Some(point.y);
        }
    }

    pub fn render_map(&self) {
        for i in 1..=self.max_x.unwrap() {
            for j in 1..=self.max_y.unwrap() {
                let this_point: Point = Point::new(i, j);
                match self.places.get(&this_point) {
                    Some(tile) => print!("{}", tile.to_char()),
                    None => print!(" "),
                };
            }
            print!("\n");
            stdout().flush().expect("This should print to screen");
        }
        println!("");
    }
}