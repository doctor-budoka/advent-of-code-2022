use std::io::stdout;
use std::collections::HashMap;
use std::io::Write;
use std::char;
use std::rc::Rc;
use std::cell::RefCell;
use space::{Point,Direction,StdInt};

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Tile {
    Wall,
    Blizzard(Direction),
}

impl Tile {
    pub fn from_char(tile_char: char) -> Option<Self> {
        return match tile_char {
            '#' => Some(Self::Wall),
            '.' => None,
            '>' | '<' | '^' | 'v' => Some(Self::Blizzard(Direction::from_char(tile_char).unwrap())),
            other => panic!("'{}' is not an admissable tile char", other),
        };
    }

    pub fn to_char(&self) -> char {
        return match self {
            Self::Wall => '#',
            Self::Blizzard(direction) => direction.to_char(),
        };
    }
}

#[derive(Debug)]
pub struct Valley {
    map: HashMap<Point, Rc<RefCell<Vec<Tile>>>>,
    min_x: Option<StdInt>,
    max_x: Option<StdInt>,
    min_y: Option<StdInt>,
    max_y: Option<StdInt>,
}

impl Valley {
    pub fn new() -> Self {
        return Self{
            map: HashMap::new(), 
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
        }
    }

    pub fn add_tile(&mut self, point: &Point, tile: &Tile) {
        if !self.map.contains_key(point) {
            self.map.insert(*point, Rc::new(RefCell::new(Vec::new())));
        }
        self.update_bounds(point);
        let boxed_contents = self.map.get(point).unwrap();
        let mut contents = boxed_contents.as_ref().borrow_mut();
        contents.push(*tile);
    }

    pub fn update_bounds(&mut self, new_point: &Point) {
        if (self.min_x == None) || (self.min_x.unwrap() > new_point.x) {
            self.min_x = Some(new_point.x);
        } 
        if (self.max_x == None) || (self.max_x.unwrap() < new_point.x) {
            self.max_x = Some(new_point.x);
        } 
        if (self.min_y == None) || (self.min_y.unwrap() > new_point.y) {
            self.min_y = Some(new_point.y);
        } 
        if (self.max_y == None) || (self.max_y.unwrap() < new_point.y) {
            self.max_y = Some(new_point.y);
        } 
    }

    pub fn move_blizzards(&mut self) -> Self {
        let mut new_map: HashMap<Point, Rc<RefCell<Vec<Tile>>>> = HashMap::new();
        for (point, boxed_blizzards) in &self.map {
            // let boxed_contents = self.map.get(&point).unwrap();
            let contents = boxed_blizzards.as_ref().borrow();
            for tile in contents.iter() {
                if *tile == Tile::Wall {
                    new_map.insert(*point, Rc::new(RefCell::new(vec![*tile])));
                    continue;
                }
                let new_position = self.find_new_blizzard_pos(&point, tile);

                if !new_map.contains_key(&new_position) {
                    new_map.insert(new_position, Rc::new(RefCell::new(Vec::new())));
                }
                let boxed_contents = new_map.get(&new_position).unwrap();
                let mut contents = boxed_contents.as_ref().borrow_mut();
                contents.push(*tile);
            }
        }
        return Self {map: new_map, min_x: self.min_x, max_x: self.max_x, min_y: self.min_y, max_y: self.max_y} 
    }

    pub fn find_new_blizzard_pos(&self, point: &Point, blizzard: &Tile) -> Point {
        if let Tile::Blizzard(direction) = blizzard {
            let attempted_position: Point = *point + Point::from_direction(&direction);
            let returned_position: Point;
            if (self.min_x == None) | (self.max_x == None) | (self.min_y == None) | (self.max_y == None) {
                unreachable!();
            }
            else if attempted_position.x <= self.min_x.unwrap() {
                returned_position = Point::new(self.max_x.unwrap() - 1, attempted_position.y);
            }
            else if attempted_position.x >= self.max_x.unwrap() {
                returned_position = Point::new(self.min_x.unwrap() + 1, attempted_position.y);
            }
            else if attempted_position.y <= self.min_y.unwrap() {
                returned_position = Point::new(attempted_position.x, self.max_y.unwrap() - 1);
            }
            else if attempted_position.y >= self.max_y.unwrap() {
                returned_position = Point::new(attempted_position.x, self.min_y.unwrap() + 1);
            }
            else {
                returned_position =  attempted_position;
            }
            if self.check_point_in_bounds(&returned_position) {
                return returned_position;
            }
            else {
                panic!("The new position should be in bounds: {}", returned_position);
            }
        }
        else {panic!("find_blizzard_pos should only have blizzard inputs");}
    }

    pub fn check_point_in_bounds(&self, point: &Point) -> bool {
        return match (self.max_x, self.min_x, self.max_y, self.min_y) {
            (Some(x_max), Some(x_min), Some(y_max), Some(y_min)) => (point.x < x_max) & (point.x > x_min) & (point.y < y_max) & (point.y > y_min),
            (None, None, None, None) => false,
            _ => unreachable!(),
        };
    }

    pub fn render(&self) {
        let y_start = self.min_y.unwrap();
        let y_end = self.max_y.unwrap();
        let x_start = self.min_x.unwrap();
        let x_end = self.max_x.unwrap();

        for j in y_start..=y_end {
            for i in x_start..=x_end {
                let this_point: Point = Point::new(i, j);
                let position = self.map.get(&this_point);
                let char_to_print: char;
                if let Some(boxed_contents) = position {
                    let contents = boxed_contents.as_ref().borrow();
                    let num_blizzards = contents.len();
                    char_to_print = if num_blizzards > 1 {char::from_digit(num_blizzards as u32, 10).unwrap()} else {contents[0].to_char()};
                }
                else {
                    char_to_print = '.';
                }
                print!("{}", char_to_print);
            }
            print!("\n");
            stdout().flush().expect("This should print to screen");
        }
        println!("");
    }
}