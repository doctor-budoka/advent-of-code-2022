use std::ops::{Add,Sub,Mul,Div};
use std::cmp::Ordering;
use std::fmt;

pub type StdInt = i64;

pub fn gcd(x: StdInt, y: StdInt) -> StdInt {
    if x == 0 {
        return y;
    } 
    else if y == 0 {
        return x;
    }
    if x == y {
        return x;
    }
    let bigger: StdInt = if x > y {x} else {y};
    let smaller: StdInt = if x > y {y} else {x};

    let remainder: StdInt = bigger % smaller;
    return gcd(smaller, remainder);
}


#[derive(Debug,Copy,Clone)]
pub struct Rational {
    num: StdInt,
    denom: StdInt,
}

impl Rational {
    pub fn new(num: StdInt, denom: StdInt) -> Self {
        let gcd: StdInt = gcd(num, denom);
        let new_num: StdInt = num / gcd;
        let new_denom: StdInt = denom / gcd;
        if new_denom > 0 {
            return Self {num: new_num, denom: new_denom};
        }
        else if new_denom < 0 {
            return Self {num: -new_num, denom: -new_denom};
        }
        else {
            panic!("Can't divide by zero!");   
        }
    }

    pub fn from_int(num: StdInt) -> Self {
        return Self::new(num, 1);
    }

    pub fn from_bool(value: bool) -> Self {
        return Self::from_int(value as StdInt);
    }
}

impl Add for Rational { 
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let new_num: StdInt = (self.num * other.denom) + (self.denom * other.num);
        let new_denom: StdInt = self.denom * other.denom;
        return Self::new(new_num, new_denom);
    }
}

impl Sub for Rational { 
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let new_num: StdInt = (self.num * other.denom) - (self.denom * other.num);
        let new_denom: StdInt = self.denom * other.denom;
        return Self::new(new_num, new_denom);
    }
}

impl Mul for Rational { 
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        return Self::new(self.num * other.num, self.denom * other.denom);
    }
}

impl Div for Rational { 
    type Output = Self;
    fn div(self, other: Self) -> Self {
        return Self::new(self.num * other.denom, self.denom * other.num);
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {return Some(self.cmp(other));}
}
impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}
impl Eq for Rational {}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        return (self.num * other.denom).cmp(&(other.num * self.denom));
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denom == 1 {
            return write!(f, "{}", self.num);
        }
        return write!(f, "{}/{}", self.num, self.denom);
    }
}

pub const R0: Rational = Rational{num: 0, denom: 1};
pub const R1: Rational = Rational{num: 1, denom: 1};
