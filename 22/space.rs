use std::ops::{Add,Sub,Neg};
use std::fmt;

pub type StdInt = i64;

pub enum Rotation {
    Left,
    Right,
}

impl Rotation {
    pub fn from_string(string: &String) -> Self {
        return match string.chars().next().unwrap() {
            'R' => Self::Right,
            'L' => Self::Left,
            other => panic!("'{}' is not a valid character for rotation", other),
        };
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate(&self, rotation: Rotation) -> Direction {
        return match (self, rotation) {
            (Self::Up, Rotation::Left) => Self::Left,
            (Self::Left, Rotation::Left) => Self::Down,
            (Self::Down, Rotation::Left) => Self::Right,
            (Self::Right, Rotation::Left) => Self::Up,
            (Self::Up, Rotation::Right) => Self::Right,
            (Self::Left, Rotation::Right) => Self::Up,
            (Self::Down, Rotation::Right) => Self::Left,
            (Self::Right, Rotation::Right) => Self::Down,
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

    pub fn as_char(&self) -> char {
        return match self {
            Self::Right => '>',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Up => '^',
        };
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

    pub fn from_direction(direction: Direction) -> Self {
        return match direction {
            Direction::Up => Self::new(0, -1),
            Direction::Down => Self::new(0, 1),
            Direction::Left => Self::new(-1, 0),
            Direction::Right => Self::new(1, 0),
        }
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
