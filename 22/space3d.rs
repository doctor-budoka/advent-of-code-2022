use std::ops::{Mul,Neg};
use space::{Direction,Rotation};

#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq)]
pub enum Direction3D {
    X,
    NegX,
    Y,
    NegY,
    Z,
    NegZ,
    Zero,
}

impl Direction3D {
    pub fn rotate_around(&self, axis: &Self) -> Self {
        if (axis == self) || (*axis == self.neg()) {
            return *axis;
        }
        return *axis * *self;
    }

    pub fn from2d_as_rotation_axis_rel_z(direction: &Direction) -> Self {
        return match direction {
            Direction::Up => Self::NegX,
            Direction::Down => Self::X,
            Direction::Right => Self::Y,
            Direction::Left => Self::NegY,
        };

    }

    pub fn from2d_as_face_direction_rel_z(direction: &Direction) -> Self {
        return match direction {
            Direction::Up => Self::Y,
            Direction::Down => Self::NegY,
            Direction::Right => Self::X,
            Direction::Left => Self::NegX,
        };
    }

    pub fn get_2d_rotation_from_tangent_change_on_z_face(start: &Direction3D, end: &Direction3D) -> Rotation {
        let rotation = match (start, end) {
            (_, Self::NegZ) => Rotation::None,
            (_, Self::Z) => Rotation::Half,
            (Self::Y, Self::NegX) | (Self::NegX, Self::NegY) | (Self::NegY, Self::X) | (Self::X,Self::Y) => Rotation::Left,
            (Self::NegY, Self::NegX) | (Self::X, Self::NegY) | (Self::Y, Self::X) | (Self::NegX,Self::Y) => Rotation::Right,
            (Self::Y,Self::Y) | (Self::X, Self::X) | (Self::NegY, Self::NegY) | (Self::NegX, Self::NegX) => Rotation::None,
            (Self::Y,Self::NegY) | (Self::X, Self::NegX) | (Self::NegY, Self::Y) | (Self::NegX, Self::X) => Rotation::Half,
            (Self::Z, _) => panic!("Z shouldn't be a starting tangent!"),
            (Self::NegZ, _) => panic!("-Z shouldn't be a starting tangent!"),
            (Self::Zero, _) | (_, Self::Zero) => panic!("Zero should never be a tangent!")
        };
        return rotation;
    }
}

impl Mul for Direction3D { 
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        return match (self, other) {
            (Self::X, Self::Y) | (Self::NegY, Self::X) | (Self::Y, Self::NegX) | (Self::NegX, Self::NegY) => Self::Z,
            (Self::NegX, Self::Y) | (Self::Y, Self::X) | (Self::NegY, Self::NegX) | (Self::X, Self::NegY) => Self::NegZ,
            (Self::Y, Self::Z) | (Self::NegZ, Self::Y) | (Self::Z, Self::NegY) | (Self::NegY, Self::NegZ) => Self::X,
            (Self::NegY, Self::Z) | (Self::Z, Self::Y) | (Self::NegZ, Self::NegY) | (Self::Y, Self::NegZ) => Self::NegX,
            (Self::Z, Self::X) | (Self::NegX, Self::Z) | (Self::X, Self::NegZ) | (Self::NegZ, Self::NegX) => Self::Y,
            (Self::NegZ, Self::X) | (Self::X, Self::Z) | (Self::NegX, Self::NegZ) | (Self::Z, Self::NegX) => Self::NegY,
            (Self::Zero, _) | (_, Self::Zero) => Self::Zero,
            (a, b) => if (a == b) || (a == b.neg()) {Self::Zero} else {panic!("Unexpected combination for cross product: {:?}, {:?}", a, b)},
        }
    }
}

impl Neg for Direction3D {
    type Output = Self;
    fn neg(self) -> Self {
        return match self {
            Self::X => Self::NegX,
            Self::NegX => Self::X,
            Self::Y => Self::NegY,
            Self::NegY => Self::Y,
            Self::Z => Self::NegZ,
            Self::NegZ => Self::Z,
            Self::Zero => Self::Zero,
        }
    }
}


#[derive(Debug,Copy,Clone)]
pub struct Orientation {
    face: Direction3D,
    left: Direction3D,
    right: Direction3D,
    up: Direction3D,
    down: Direction3D,
}

impl Orientation {
    pub fn new() -> Self {
        return Self{
            face: Direction3D::Z,
            left: Direction3D::from2d_as_rotation_axis_rel_z(&Direction::Left),
            right: Direction3D::from2d_as_rotation_axis_rel_z(&Direction::Right),
            up: Direction3D::from2d_as_rotation_axis_rel_z(&Direction::Up),
            down: Direction3D::from2d_as_rotation_axis_rel_z(&Direction::Down),
        }
    }

    pub fn rotate(&self, rotation_axis: &Direction3D) -> Self {
        return Self{
            face: self.face.rotate_around(rotation_axis),
            left: self.left.rotate_around(rotation_axis),
            right: self.right.rotate_around(rotation_axis),
            up: self.up.rotate_around(rotation_axis),
            down: self.down.rotate_around(rotation_axis),
        }
    }

    pub fn get_rotation_axis_for_direction(&self, direction: &Direction) -> Direction3D {
        return match direction {
            Direction::Up => self.up,
            Direction::Down => self.down,
            Direction::Left => self.left,
            Direction::Right => self.right,
        };
    }

    pub fn get_face_direction(&self) -> Direction3D {
        return self.face;
    }
}



