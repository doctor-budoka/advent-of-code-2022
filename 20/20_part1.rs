use std::env;
use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::io::stdout;
use std::io::Write;

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

#[derive(Debug)]
struct CircularVector {
    vector: HashMap<usize, i32>,
    length: usize,
    anchor: Option<usize>,
}

impl CircularVector {
    pub fn new() -> CircularVector {
        return CircularVector {vector: HashMap::new(), length: 0, anchor: None};
    }

    pub fn insert(&mut self, item: i32) {
        let new_key: usize = self.length;
        self.vector.insert(new_key, item);
        self.length += 1;
        if item == 0 {
            self.anchor = Some(new_key);
        }
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


        if current_to_value == 0 {
            self.anchor = Some(from_key);
        }
        else if current_from_value == 0 {
            self.anchor = Some(to_key);
        }
    }

    pub fn get(&self, key: i32) -> i32{
        let circular_key = self.get_circular_key(key);
        return self.get_from_circular_key(circular_key);
    }

    pub fn get_circular_key(&self, key: i32) -> usize {
        let modded = key % (self.length as i32);
        return if modded < 0 {modded + (self.length as i32)} else {modded} as usize;
    }

    pub fn get_from_circular_key(&self, key: usize) -> i32 {
        return *self.vector.get(&key).unwrap();
    }

    pub fn render_as_vec(&self) {
        print!("[");
        for i in 0..self.length {
            print!("{}", self.get(i as i32));
            if i < self.length - 1 {print!(", ");}
        }
        print!("]\n");
        stdout().flush().expect("This should print to screen");
    }
}


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);

    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let mut original_vector: Vec<i32> = Vec::new();
    let mut new_vector: CircularVector = CircularVector::new();
    for line in input.trim().lines().collect::<Vec<&str>>() {
        let item: i32 = line.trim().parse().unwrap();
        original_vector.push(item);
        new_vector.insert(item);
    }

    new_vector.render_as_vec();
    for (ind, value) in original_vector.iter().enumerate() {
        new_vector.move_key_num_places(ind as i32, *value);
        new_vector.render_as_vec();
    }
    println!("{:?}", &original_vector);
    println!("{:?}", &new_vector);
    if let Some(base_key) = new_vector.anchor {
        let grove_coord1 = new_vector.get(base_key as i32 + 1000);
        let grove_coord2 = new_vector.get(base_key as i32 + 2000);
        let grove_coord3 = new_vector.get(base_key as i32 + 3000);
        println!("Coords: {} {} {}. Sum: {}", &grove_coord1, &grove_coord2, &grove_coord3, grove_coord1 + grove_coord2 + grove_coord3);
    }
    else {
        panic!("No 0 found!");
    }
} 
