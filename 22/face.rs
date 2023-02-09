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

    pub fn get_left_glue(&self) -> Option<EdgeGlue> {
        return self.left;
    }

    pub fn get_right_glue(&self) -> Option<EdgeGlue> {
        return self.right;
    }

    pub fn get_top_glue(&self) -> Option<EdgeGlue> {
        return self.top;
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

    pub fn add_glue(&mut self, other_face: &Point, direction: &Direction, rotation: &Rotation) {
        match direction {
            Direction::Up => self.add_top_glue(other_face, rotation),
            Direction::Down => self.add_bottom_glue(other_face, rotation),
            Direction::Left => self.add_left_glue(other_face, rotation),
            Direction::Right => self.add_right_glue(other_face, rotation),
        }
    }

    pub fn add_top_glue(&mut self, face: &Point, rotation: &Rotation) {
        self.top = Some((*face, *rotation));
    }

    pub fn add_bottom_glue(&mut self, face: &Point, rotation: &Rotation) {
        self.bottom = Some((*face, *rotation));
    }

    pub fn add_left_glue(&mut self, face: &Point, rotation: &Rotation) {
        self.left = Some((*face, *rotation));
    }

    pub fn add_right_glue(&mut self, face: &Point, rotation: &Rotation) {
        self.right = Some((*face, *rotation));
    }

    #[allow(dead_code)]
    pub fn is_fully_glued(&self) -> bool {
        for direction in Direction::get_directions() {
            if !self.is_glued_in_direction(&direction) {
                return false;
            }
        }
        return true;
    }

    #[allow(dead_code)]
    pub fn is_glued_in_direction(&self, direction: &Direction) -> bool {
        return match direction {
            Direction::Up => self.is_top_glued(),
            Direction::Down => self.is_bottom_glued(),
            Direction::Left => self.is_left_glued(),
            Direction::Right => self.is_right_glued(),
        }
    }

    #[allow(dead_code)]
    pub fn is_top_glued(&self) -> bool {
        return match self.top {
            Some(_) => true,
            None => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_bottom_glued(&self) -> bool {
        return match self.bottom {
            Some(_) => true,
            None => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_left_glued(&self) -> bool {
        return match self.left {
            Some(_) => true,
            None => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_right_glued(&self) -> bool {
        return match self.right {
            Some(_) => true,
            None => false,
        }
    }

    pub fn create_copy(&self) -> Self {
        let mut hashmap_copy: HashMap<Point,Tile> = HashMap::new();
        for (key, value) in self.places.iter() {
            hashmap_copy.insert(*key, *value);
        }
        return Self {
            places: hashmap_copy, 
            size: self.size, 
            left: self.left, 
            right: self.right, 
            top: self.top, 
            bottom: self.bottom
        };
    }
}
