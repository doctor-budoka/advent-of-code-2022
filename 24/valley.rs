use std::io::stdout;
use std::collections::HashMap;
use std::io::Write;
use std::char;
use std::rc::Rc;
use std::cell::RefCell;
use space::{Point,Direction,StdInt};

#[derive(Debug,Copy,Clone)]
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