use std::collections::HashMap;
use std::io::stdout;
use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;

use space::{Marker,Point,StdInt};
use face::{Face,Tile};


pub struct Map {
    faces: HashMap<Point, Rc<RefCell<Face>>>,
    face_size: StdInt,
    max_x: Option<StdInt>,
    max_y: Option<StdInt>,
}

impl Map {
    pub fn new(size: StdInt) -> Self {
        return Map {faces: HashMap::new(), face_size: size, max_x: None, max_y: None};
    }

    pub fn find_face(&self, point: &Point) -> Point {
        return Point::new(((point.x - 1) / self.face_size) + 1, ((point.y - 1) / self.face_size) + 1);
    }

    pub fn find_point_on_face(&self, point: &Point) -> Point {
        let new_x = ((point.x - 1) % self.face_size) + 1;
        let new_y = ((point.y - 1) % self.face_size) + 1;
        if new_x < 0 {panic!("x value negative!");}
        if new_y < 0 {panic!("y value negative!");}
        return Point::new(new_x, new_y);
    }

    pub fn flatten_point(&self, face: &Point, point: &Point) -> Point {
        return Point::new(((face.x - 1) * self.face_size) + point.x, ((face.y - 1) * self.face_size) + point.y)
    }

    pub fn add_point(&mut self, point: &Point, tile: &Tile) {
        let face: Point = self.find_face(point);
        self.add_face(&face);

        let point_on_face: Point = self.find_point_on_face(point);
        self.add_point_to_face(&face, &point_on_face, tile);

        if (self.max_x == None) || (point.x > self.max_x.unwrap()) {
            self.max_x = Some(point.x);
        }
        if (self.max_y == None) || (point.y > self.max_y.unwrap()) {
            self.max_y = Some(point.y);
        }
    }

    pub fn add_face(&mut self, face: &Point) {
        self.faces.entry(*face).or_insert(Rc::new(RefCell::new(Face::new(self.face_size))));
    }

    pub fn add_point_to_face(&mut self, face: &Point, point_on_face: &Point, tile: &Tile) {
        let mut boxed_face =  self.faces.get(&face).expect("Face should already exist!").as_ref().borrow_mut();
        boxed_face.add_point(point_on_face, tile);
    }

    pub fn get_tile(&self, point: &Point) -> Option<Tile> {
        let face = self.find_face(point);
        let point_on_face = self.find_point_on_face(point);
        if let Some(face) = self.faces.get(&face) {
            return face.as_ref().borrow().get_tile(&point_on_face);
        }
        else {
            return None;
        }
    }

    pub fn get_new_position(&self, marker: &Marker, distance: StdInt) -> Marker {
        let mut current_marker: Marker = *marker;
        for _ in 0..distance {
            current_marker = self.attempt_move(&current_marker);
        }
        return current_marker;
    }

    fn attempt_move(&self, marker: &Marker) -> Marker {
        let attempt_marker: Marker = marker.next();

        return match self.get_tile(&attempt_marker.get_position()) {
            Some(Tile::Clear) => attempt_marker,
            Some(Tile::Stone) => *marker,
            None => self.wrap_around_if_possible(&marker),
        }
    }

    fn wrap_around_if_possible(&self, marker: &Marker) -> Marker {
        // let current_face = self.find_face(position);
        // if let Some((new_face, rotation)) = current_face.get_glue_from_direction(direction);
        // let mut last_point: Point = *position;
        // let mut current_point: Point = *position + opposite_direction;
        // while let Some(_) = self.places.get(&current_point) {
        //         last_point += opposite_direction;
        //         current_point += opposite_direction;
        // }
        // return if *self.places.get(&last_point).unwrap() == Tile::Clear {last_point} else {*position};
        return *marker;
    }

    #[allow(dead_code)]
    pub fn render_map(&self) {
        for j in 1..=self.max_y.unwrap() {
            for i in 1..=self.max_x.unwrap() {
                let this_point: Point = Point::new(i, j);
                match self.get_tile(&this_point) {
                    Some(tile) => print!("{}", tile.to_char()),
                    None => print!(" "),
                };
            }
            print!("\n");
            stdout().flush().expect("This should print to screen");
        }
        println!("");
    }

    #[allow(dead_code)]
    pub fn render_map_with_marker(&self, marker: &Marker) {
        let marker_pos = marker.get_position();
        for j in 1..=self.max_y.unwrap() {
            for i in 1..=self.max_x.unwrap() {
                let this_point: Point = Point::new(i, j);
                if this_point == *marker_pos {
                    print!("{}", marker.get_direction().as_char());
                }
                else {
                    match self.get_tile(&this_point) {
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