use std::ops::{Mul,Div,Neg};

use space::{StdInt,Direction,Rotation};

#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq)]
enum Direction3D {
    X,
    NegX,
    Y,
    NegY,
    Z,
    NegZ,
}

impl Direction3D {
    pub fn rotate_around(&self, axis: Self) -> Self {
        if (axis == self) || (axis == -self) {
            return axis;
        }
        return axis * self;
    }

    pub fn from2d_as_rotation_axis_rel_z(direction: &Direction) -> Self {
        return match direction {
            Direction::Up => Self::NegX,
            Direction::Down => Self::X,
            Direction::Right => Self::NegX,
            Direction::Left => Self::X,
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

    pub fn get_2d_rotation_from_normal_change_on_z_face(start: &Direction3D, end: &Direction3D) {
        return match (start, end) {
            (Self::Z, _) => panic!("Z shouldn't be a starting normal"),
            (Self::NegZ, _) => panic!("-Z shouldn't be a starting normal"),
            (_, Self::NegZ) => Rotation::None,
            (_, Self::Z) => Rotation::Half,
            (Self::Y, Self::NegX) | (Self::NegX, Self::NegY) | (Self::NegY, Self::X) | (Self::X,Self::Y) => Rotation::Left,
            (Self::NegY, Self::NegX) | (Self::X, Self::NegY) | (Self::Y, Self::X) | (Self::NegX,Self::Y) => Rotation::Right,
            (Self::Y,Self::Y) | (Self::X, Self::X) | (Self::NegY, Self::NegY) | (Self::NegX, Self::NegX) => Rogation::None,
            (Self::Y,Self::NegY) | (Self::X, Self::NegX) | (Self::NegY, Self::Y) | (Self::NegX, Self::X) => Rogation::Half,
        };
    }

    // Steps: 1. dfs for faces on 2d surface, noting the axis rotation we move along (by converting from the direction to axis of rotation)
    // 2. When we note that we've found a face that should be glued to our start face: Apply those axis rotations for the relevant normal 
    //     (if we find the left side, NegX) to get the "new orientation" of the normal
    // 3. Get the direction of normal change and invert it to get the direction needed to fix it (giving us glue rotatoin)
}

impl Mul for Direction3D { 
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        return match (self, other) {
            (Self::X, Self::Y) | (Self::NegY, Self::X) | (Self::Y, Self::NegX) | (Self::NegX, Self::NegY) => (Self::Z),
            (Self::NegX, Self::Y) | (Self::Y, Self::X) | (Self::NegY, Self::NegX) | (Self::X, Self::NegY) => (Self::NegZ),
            (Self::Y, Self::Z) | (Self::NegZ, Self::Y) | (Self::Z, Self::NegY) | (Self::NegY, Self::NegZ) => (Self::X),
            (Self::NegY, Self::Z) | (Self::Z, Self::Y) | (Self::NegZ, Self::NegY) | (Self::Y, Self::NegZ) => (Self::NegX),
            (Self::Z, Self::X) | (Self::NegX, Self::Z) | (Self::X, Self::NegZ) | (Self::NegZ, Self::NegX) => (Self::Y),
            (Self::NegZ, Self::X) | (Self::X, Self::Z) | (Self::NegX, Self::NegZ) | (Self::Z, Self::NegX) => (Self::NegY),
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
        }
    }
}



