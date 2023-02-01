use std::env;
use std::fs;
use std::cmp::Ordering;

type CanonicalInt = i64;
type ValueIndex = (CanonicalInt, usize);

const DECRYPT_KEY_DEFAULT: CanonicalInt = 811589153;


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    let rounds: usize = if env_args.len() > 2 {env_args[2].parse().unwrap()} else {1};
    let decrypt_key: CanonicalInt = if env_args.len() > 3 {env_args[3].parse().unwrap()} else {DECRYPT_KEY_DEFAULT};
    println!("File name is '{}'. Reading input...", file_name);

    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let mut data: Vec<ValueIndex> = Vec::new();
    for (index, line) in input.trim().lines().collect::<Vec<&str>>().iter().enumerate() {
        let item: CanonicalInt = line.trim().parse().unwrap();
        data.push((item * decrypt_key, index));
    }

    println!("{:?}", data);
    let data_len: usize = data.len();
    println!("Total elements to run through: {}", data_len);
    for i in 0..rounds {
        for ind in 0..data_len {
            let current_index = data.iter().position(|&x| x.1 == ind).unwrap();
            let current_value = data[current_index].0;

            let new_position: CanonicalInt = current_index as CanonicalInt + current_value;
            let new_position_mod: usize = positive_mod(new_position, data_len as CanonicalInt - 1) as usize; 
            match new_position_mod.cmp(&current_index) {
                Ordering::Equal => (),
                Ordering::Less => {
                    data.remove(current_index);
                    data.insert(new_position_mod, (current_value, ind));
                },
                Ordering::Greater => {
                    data.remove(current_index);
                    data.insert(new_position_mod, (current_value, ind));
                },
            };
        }
        println!("After round {}: {:?}", i + 1, data);
    }

    let base_key = data.iter().position(|&x| x.0 == 0).unwrap();
    let grove_coord1 = data[positive_mod(base_key as CanonicalInt + 1000, data_len as CanonicalInt) as usize].0;
    let grove_coord2 = data[positive_mod(base_key as CanonicalInt + 2000, data_len as CanonicalInt) as usize].0;
    let grove_coord3 = data[positive_mod(base_key as CanonicalInt + 3000, data_len as CanonicalInt) as usize].0;
    println!("Coords: {} {} {}. Sum: {}", &grove_coord1, &grove_coord2, &grove_coord3, grove_coord1 + grove_coord2 + grove_coord3);
} 

fn positive_mod(n: CanonicalInt, div: CanonicalInt) -> CanonicalInt {
    let modded = n % div;
    return if modded < 0 {modded + div} else {modded};
}
