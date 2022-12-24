use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    
    let mut output = 0;
    if let Ok(lines) = read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(val) = line {
                let knapsack_priority = get_knapsack_priority(val);
                output += knapsack_priority;
            }
        }
    }
    println!("Output: {}", output);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn get_knapsack_priority(val: String) -> i32 {
    let (compartment_1, compartment_2) = val.split_at(val.len()/2);
    let common: char = find_common_letter(compartment_1, compartment_2);
    return get_char_priority(common);
}


fn find_common_letter(str_1: &str, str_2: &str) -> char {
    let mut exists = vec![false; 52];
    for c in str_1.chars() {
        exists[get_char_priority(c) as usize - 1] = true;
    }
    for c in str_2.chars() {
        let is_common = exists[get_char_priority(c) as usize - 1];
        if is_common {
            return c;
        }
    }
    panic!("No common characters!")
}

fn get_char_priority(letter: char) -> i32 {
    if letter.is_lowercase() {
        return (letter as i32) - ('a' as i32) + 1;
    }
    else {
        return (letter as i32) - ('A' as i32) + 27;
    }
}
