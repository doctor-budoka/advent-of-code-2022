use std::env;
use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::io::stdout;
use std::io::Write;

const DECRYPT_KEY: i64 = 811589153;

#[derive(PartialEq,Clone,Copy)]
enum Sign {
    Neg,
    Zero,
    Pos,
}

impl Sign {
    fn as_scalar(&self) -> i64 {
        return match self {
            Sign::Neg => -1,
            Sign::Zero => 0,
            Sign::Pos => 1,
        }
    }

    fn get_sign(num: i64) -> Sign {
        return match num.cmp(&0) {
            Ordering::Less => Sign::Neg,
            Ordering::Equal => Sign::Zero,
            Ordering::Greater => Sign::Pos,
        }
    }
}

#[derive(Debug)]
struct CircularVector {
    vector: HashMap<usize, i64>,
    new_inds: HashMap<usize, usize>,
    original_inds: HashMap<usize, usize>,
    length: usize,
    anchor: Option<usize>,
}

impl CircularVector {
    pub fn new() -> CircularVector {
        return CircularVector {
            vector: HashMap::new(),
            new_inds: HashMap::new(),
            original_inds: HashMap::new(),
            length: 0,
            anchor: None,
        };
    }

    pub fn insert(&mut self, item: i64) {
        let new_key: usize = self.length;
        self.vector.insert(new_key, item);
        self.new_inds.insert(new_key, new_key);
        self.original_inds.insert(new_key, new_key);
        self.length += 1;
        if item == 0 {
            self.anchor = Some(new_key);
        }
    }

    pub fn move_key_num_places(&mut self, key: i64, num: i64) {
        let sign: Sign = Sign::get_sign(num);
        let sign_as_scalar: i64 = *&sign.as_scalar();
        let distance: i64 = num.abs() % (self.length as i64);
        for i in 0..distance {
            self.swap(key + (sign_as_scalar * i), sign);
        }
    }

    pub fn swap(&mut self, key: i64, sign: Sign) {
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

        let original_from_value_key: usize = *self.original_inds.get(&from_key).unwrap();
        let original_to_value_key: usize = *self.original_inds.get(&to_key).unwrap();
        self.original_inds.insert(to_key, original_from_value_key);
        self.original_inds.insert(from_key, original_to_value_key);
        self.new_inds.insert(original_from_value_key, to_key);
        self.new_inds.insert(original_to_value_key, from_key);

        if current_to_value == 0 {
            self.anchor = Some(from_key);
        }
        else if current_from_value == 0 {
            self.anchor = Some(to_key);
        }
    }

    pub fn get(&self, key: i64) -> i64 {
        let circular_key = self.get_circular_key(key);
        return self.get_from_circular_key(circular_key);
    }

    pub fn get_circular_key(&self, key: i64) -> usize {
        let modded = key % (self.length as i64);
        return if modded < 0 {modded + (self.length as i64)} else {modded} as usize;
    }

    pub fn get_from_circular_key(&self, key: usize) -> i64 {
        return *self.vector.get(&key).unwrap();
    }

    pub fn render_as_vec(&self) {
        print!("[");
        for i in 0..self.length {
            print!("{}", self.get(i as i64));
            if i < self.length - 1 {print!(", ");}
        }
        print!("]\n");
        stdout().flush().expect("This should print to screen");
    }

    pub fn get_new_ind_from_original(&self, original_ind: usize) -> usize {
        return *self.new_inds.get(&original_ind).unwrap();
    }
}


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);

    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let mut original_vector: Vec<i64> = Vec::new();
    let mut new_vector: CircularVector = CircularVector::new();
    for line in input.trim().lines().collect::<Vec<&str>>() {
        let item: i64 = line.trim().parse().unwrap();
        original_vector.push(item * DECRYPT_KEY);
        new_vector.insert(item * DECRYPT_KEY);
    }

    println!("Total elements to run through: {}", &original_vector.len());
    for i in 0..10 {
        for (ind, value) in original_vector.iter().enumerate() {
            let new_ind: i64 = new_vector.get_new_ind_from_original(ind) as i64;
            new_vector.move_key_num_places(new_ind, *value);
            if (ind + 1) % 100 == 0 {
                println!("Round {} {}/{} done", i + 1, ind + 1, &original_vector.len());
            }
        }
    }

    if let Some(base_key) = new_vector.anchor {
        let grove_coord1 = new_vector.get(base_key as i64 + 1000);
        let grove_coord2 = new_vector.get(base_key as i64 + 2000);
        let grove_coord3 = new_vector.get(base_key as i64 + 3000);
        println!("Coords: {} {} {}. Sum: {}", &grove_coord1, &grove_coord2, &grove_coord3, grove_coord1 + grove_coord2 + grove_coord3);
    }
    else {
        panic!("No 0 found!");
    }
} 
