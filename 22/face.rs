use std::collections::HashMap;

use space::{Direction,Point,Rotation,StdInt};

#[derive(PartialEq,Copy,Clone)]
pub enum Tile {
    Stone,
    Clear,
}

impl Tile {
    pub fn from_char(character: char) -> Self {
        return match character {
            '#' => Self::Stone,
            '.' => Self::Clear,
            other => panic!("'{}' is not a valid tile character", other),
        }
    }

    pub fn to_char(&self) -> char {
        return match self {
            Self::Stone => '#',
            Self::Clear => '.',
        }
    }
}

pub type EdgeGlue = (Point, Rotation);

pub struct Face {
    size: StdInt,
    places: HashMap<Point,Tile>,
    left: Option<EdgeGlue>,
    right: Option<EdgeGlue>,
    top: Option<EdgeGlue>,
    bottom: Option<EdgeGlue>,
}

impl Face {
    pub fn new(size: StdInt) -> Self {
        return Self {
            places: HashMap::new(), 
            size: size, 
            left: None, 
            right:None, 
            top: None, 
            bottom: None
        };
    }

    pub fn add_point(&mut self, point: &Point, tile: &Tile) {
        self.places.insert(*point, *tile);
    }

    pub fn add_left_edge(&mut self, face: &Point, rotation: &Rotation) {
        self.left = Some((*face, *rotation));
    }

    pub fn get_left_glue(&self) -> Option<EdgeGlue> {
        return self.left;
    }

    pub fn add_right_edge(&mut self, face: &Point, rotation: &Rotation) {
        self.right = Some((*face, *rotation));
    }

    pub fn get_right_glue(&self) -> Option<EdgeGlue> {
        return self.right;
    }

    pub fn add_top_edge(&mut self, face: &Point, rotation: &Rotation) {
        self.top = Some((*face, *rotation));
    }

    pub fn get_top_glue(&self) -> Option<EdgeGlue> {
        return self.top;
    }

    pub fn add_bottom_edge(&mut self, face: &Point, rotation: &Rotation) {
        self.bottom = Some((*face, *rotation));
    }

    pub fn get_bottom_glue(&self) -> Option<EdgeGlue> {
        return self.bottom;
    }

    pub fn get_tile(&self, point: &Point) -> Option<Tile> {
        return self.places.get(&point).copied();
    }

    pub fn get_glue_from_direction(&self, direction: &Direction) -> Option<EdgeGlue> {
        return match direction {
            Direction::Up => self.get_top_glue(),
            Direction::Down => self.get_bottom_glue(),
            Direction::Left => self.get_left_glue(),
            Direction::Right => self.get_right_glue(),
        };
    }
}
