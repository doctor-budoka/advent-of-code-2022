use space::{Point, StdInt};

#[derive(Debug,Copy,Clone,Hash)]
pub struct PathPoint {
    pub point: Point,
    pub steps_from_start: StdInt,
    pub distance_from_end: StdInt,
}

impl PathPoint {
    pub fn new_start(point: &Point, distance_from_end: StdInt) -> Self {
        return Self {
            point: *point,
            steps_from_start: 0,
            distance_from_end: distance_from_end,
        }
    }

    pub fn new(point: &Point, steps_from_start: StdInt, distance_from_end: StdInt) -> Self {
        return Self{
            point: *point, 
            steps_from_start: steps_from_start, 
            distance_from_end: distance_from_end
        };
    }

    pub fn estimated_path_length(&self) -> StdInt {
        return self.steps_from_start + self.distance_from_end;
    }
}

impl PartialOrd for PathPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other))
    }
}

impl Ord for PathPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_val = self.estimated_path_length();
        let other_val = other.estimated_path_length();

        return self_val.cmp(&other_val);
    }
}

impl PartialEq for PathPoint {
    fn eq(&self, other: &Self) -> bool {
        return (self.steps_from_start == other.steps_from_start) && (self.point == other.point);
    }
}

impl Eq for PathPoint {}
