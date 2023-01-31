use std::env;
use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(PartialEq,Clone,Copy)]
enum Sign {
    Neg,
    Zero,
    Pos,
}

impl Sign {
    fn as_scalar(&self) -> i32 {
        return match self {
            Sign::Neg => -1,
            Sign::Zero => 0,
            Sign::Pos => 1,
        }
    }

    fn get_sign(num: i32) -> Sign {
        return match num.cmp(&0) {
            Ordering::Less => Sign::Neg,
            Ordering::Equal => Sign::Zero,
            Ordering::Greater => Sign::Pos,
        }
    }
}


struct CircularVector {
    vector: HashMap<usize, i32>,
    length: usize,
}

impl CircularVector {
    pub fn new() -> CircularVector {
        return CircularVector {vector: HashMap::new(), length: 0};
    }

    pub fn insert(&mut self, item: i32) {
        let new_key: usize = self.length;
        self.vector.insert(new_key, item);
        self.length += 1;
    }

    pub fn move_key_num_places(&mut self, key: i32, num: i32) {
        let sign: Sign = Sign::get_sign(num);
        let sign_as_scalar: i32 = *&sign.as_scalar();
        let distance: i32 = num.abs();
        for i in 0..distance {
            self.swap(key + (sign_as_scalar * i), sign);
        }
    }

    pub fn swap(&mut self, key: i32, sign: Sign) {
        if sign == Sign::Zero {
            return ();
        }
        let displacement = sign.as_scalar();
        let move_to = key + displacement;
        let current_to_value = self.get(move_to);
        let to_key = self.get_circular_key(move_to);

        let current_from_value = self.get(key);
        let from_key = self.get_circular_key(key);

        self.vector.insert(to_key, current_from_value);
        self.vector.insert(from_key, current_to_value);

    }

    pub fn get(&self, key: i32) -> i32{
        let circular_key = self.get_circular_key(key);
        return *self.vector.get(&circular_key).unwrap();
    }

    pub fn get_circular_key(&self, key: i32) -> usize {
        return (key % (self.length as i32)) as usize; 
    }

    pub fn get_from_circular_key(&self, key: usize) -> i32 {
        return *self.vector.get(&key).unwrap();
    }
}


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);

    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let original_vector: Vec<i32> = Vec::new();
    let new_vector: CircularVector = CircularVector::new();
    for line in input.trim().lines().collect::<Vec<&str>>() {
        let item: i32 = line.trim().parse().unwrap();
        println!("{}", item);
    }
} 
