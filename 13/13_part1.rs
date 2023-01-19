use std::env;
use std::fs;
use std::cmp::Ordering;

mod packet_data;
use packet_data::Data;

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");

    let mut correct_inds: Vec<u32> = Vec::new();
    let mut ind: u32 = 1;
    for pair in input.split("\n\n").collect::<Vec<&str>>() {
        let pair_vec: Vec<&str> = pair.trim().split("\n").collect();
        let packet_1: Data = Data::from_string(&pair_vec[0].trim().to_string());
        let packet_2: Data = Data::from_string(&pair_vec[1].trim().to_string());
        if packet_1.cmp(&packet_2) == Ordering::Less {
            correct_inds.push(ind);
        }
        ind += 1;
    }
    println!("Correct inds: {:?}", correct_inds);
    println!("Sum of correct inds: {}", correct_inds.iter().sum::<u32>());
} 
