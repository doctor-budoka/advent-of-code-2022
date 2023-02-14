use std::ops::{Add,Sub,Neg,AddAssign};
use std::fmt;

pub type StdInt = i64;

#[derive(Debug)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
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
            Direction::North => Self::new(0, -1),
            Direction::South => Self::new(0, 1),
            Direction::West => Self::new(-1, 0),
            Direction::East => Self::new(1, 0),
            Direction::NorthWest => Self::new(-1, -1),
            Direction::NorthEast => Self::new(1, -1),
            Direction::SouthWest => Self::new(-1, 1),
            Direction::SouthEast => Self::new(1, 1),
        }
    }

    #[allow(dead_code)]
    pub fn length(&self) -> StdInt {
        return self.x.abs() + self.y.abs();
    }

    #[allow(dead_code)]
    pub fn scalar_multiplication(&self, scalar: StdInt) -> Self {
        return Self::new(self.x * scalar, self.y * scalar);
    }

    #[allow(dead_code)]
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