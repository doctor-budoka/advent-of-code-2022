use std::collections::HashMap;
use std::io::stdout;
use std::io::Write;

use space::{Direction,Point,StdInt};

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

    pub fn get_new_position(&self, position: &Point, direction: &Direction, distance: StdInt) -> Point {
        let mut current_point = *position;
        for i in 0..distance {
            current_point = self.attempt_move(&current_point, direction);
        }
        return current_point;
    }

    fn attempt_move(&self, position: &Point, direction: &Direction) -> Point {
        let movement_vector: Point = direction.as_vector();
        let attempt_move: Point = *position + movement_vector;

        return match self.places.get(&attempt_move) {
            Some(Tile::Clear) => attempt_move,
            Some(Tile::Stone) => *position,
            Some(Tile::None) => panic!("This shouldn't be possible"),
            None => self.walk_back_if_possible(&position, &direction),
        }
    }

    fn walk_back_if_possible(&self, position: &Point, direction: &Direction) -> Point {
        let opposite_direction: Point = -direction.as_vector();
        let mut last_point: Point = *position;
        let mut current_point: Point = *position + opposite_direction;
        while let Some(_) = self.places.get(&current_point) {
                last_point += opposite_direction;
                current_point += opposite_direction;
        }
        return if *self.places.get(&last_point).unwrap() == Tile::Clear {last_point} else {*position};
    }

    pub fn render_map(&self) {
        for j in 1..=self.max_y.unwrap() {
            for i in 1..=self.max_x.unwrap() {
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

    pub fn render_map_with_current_position(&self, position: &Point, direction: &Direction) {
        for j in 1..=self.max_y.unwrap() {
            for i in 1..=self.max_x.unwrap() {
                let this_point: Point = Point::new(i, j);
                if this_point == *position {
                    print!("{}", direction.as_char());
                }
                else {
                    match self.places.get(&this_point) {
                        Some(tile) => print!("{}", tile.to_char()),
                        None => print!(" "),
                    };
                }
            }
            print!("\n");
            stdout().flush().expect("This should print to screen");
        }
        println!("");
    }
}