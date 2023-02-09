use std::ops::{Add,Sub,Neg,AddAssign};
use std::fmt;

pub type StdInt = i64;

#[derive(Debug,Copy,Clone,Hash)]
pub enum Rotation {
    Left,
    Right,
    Half,
    None,
}

impl Rotation {
    pub fn from_string(string: &String) -> Self {
        return match string.chars().next().unwrap() {
            'R' => Self::Right,
            'L' => Self::Left,
            'H' => Self::Half,
            'N' => Self::None,
            other => panic!("'{}' is not a valid character for rotation", other),
        };
    }

    pub fn inverse(&self) -> Self {
        return match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Half => Self::Half,
            Self::None => Self::None,
        };
    }
}

#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate(&self, rotation: &Rotation) -> Direction {
        return match (self, *rotation) {
            (Self::Up, Rotation::Left) => Self::Left,
            (Self::Left, Rotation::Left) => Self::Down,
            (Self::Down, Rotation::Left) => Self::Right,
            (Self::Right, Rotation::Left) => Self::Up,
            (Self::Up, Rotation::Right) => Self::Right,
            (Self::Left, Rotation::Right) => Self::Up,
            (Self::Down, Rotation::Right) => Self::Left,
            (Self::Right, Rotation::Right) => Self::Down,
            (Self::Up, Rotation::Half) => Self::Down,
            (Self::Left, Rotation::Half) => Self::Right,
            (Self::Down, Rotation::Half) => Self::Up,
            (Self::Right, Rotation::Half) => Self::Left,
            (Self::Up, Rotation::None) => Self::Up,
            (Self::Left, Rotation::None) => Self::Left,
            (Self::Down, Rotation::None) => Self::Down,
            (Self::Right, Rotation::None) => Self::Right,
        }
    }

    pub fn as_vector(&self) -> Point {
        return match self {
            Self::Up => Point::new(0, -1),
            Self::Down => Point::new(0, 1),
            Self::Left => Point::new(-1, 0),
            Self::Right => Point::new(1, 0),
        }
    }

    pub fn as_int(&self) -> StdInt {
        return match self {
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3,
        };
    }

    #[allow(dead_code)]
    pub fn as_char(&self) -> char {
        return match self {
            Self::Right => '>',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Up => '^',
        };
    }

    pub fn inverse(&self) -> Self {
        return match self {
            Self::Right => Self::Left,
            Self::Left => Self::Right,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        };
    }

    pub fn get_directions() -> Vec<Self> {
        return vec![Self::Up, Self::Left, Self::Down, Self::Right];
    }
}


#[derive(Debug,Copy,Clone,Hash)]
pub struct Point {
    pub x: StdInt,
    pub y: StdInt,
}

impl Point {
    pub fn new(x: StdInt, y: StdInt) -> Self {
        return Point {x: x, y: y}
    }

    #[allow(dead_code)]
    pub fn from_direction(direction: &Direction) -> Self {
        return match *direction {
            Direction::Up => Self::new(0, -1),
            Direction::Down => Self::new(0, 1),
            Direction::Left => Self::new(-1, 0),
            Direction::Right => Self::new(1, 0),
        }
    }

    pub fn length(&self) -> StdInt {
        return self.x.abs() + self.y.abs();
    }

    pub fn scalar_multiplication(&self, scalar: StdInt) -> Self {
        return Self::new(self.x * scalar, self.y * scalar);
    }

    pub fn scalar_division(&self, scalar: StdInt) -> Self {
        return Self::new(self.x / scalar, self.y / scalar);
    }
}

impl Add for Point { 
    type Output = Self;
    fn add(self, other: Self) -> Self {
        return Self::new(self.x + other.x, self.y + other.y);
    }
}

impl Sub for Point { 
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        return Self::new(self.x - other.x, self.y - other.y);
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        return Self::new(-self.x, -self.y);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return (self.x == other.x) && (self.y == other.y);
    }
}

impl Eq for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}

#[derive(Debug,Copy,Clone,Hash)]
pub struct Marker {
    point: Point,
    direction: Direction,
}

impl Marker {
    pub fn new(point: Point, direction: Direction) -> Self {
        return Self{point: point, direction: direction};
    }

    pub fn get_position(&self) -> Point {
        return self.point;
    }

    pub fn get_direction(&self) -> Direction {
        return self.direction;
    }

    pub fn get_rotated_marker(&self, rotation: &Rotation) -> Self {
        return Self::new(self.get_position(), self.direction.rotate(rotation));
    }

    pub fn next(&self) -> Marker {
        let movement_vector: Point = self.get_direction().as_vector();
        let new_position: Point = self.get_position() + movement_vector;
        return Self::new(new_position, self.direction);
    }
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}{}", self.point, self.direction.as_char());
    }
}
