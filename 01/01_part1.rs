use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    
    let mut max_elf = 0;
    let mut max_cals = 0;
    let mut this_elf = 0;
    let mut this_cals = 0;
    if let Ok(lines) = read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(val) = line {
                if val == "" {
                    if this_cals > max_cals {
                        max_elf = this_elf;
                        max_cals = this_cals;
                    }
                    this_elf = this_elf + 1;
                    this_cals = 0;
                }
                else {
                    let int_val = val.parse::<i64>().unwrap();
                    this_cals = this_cals + int_val;
                }
            }
        }
    }

    println!("Total cals: {}, carried by: elf {}", max_cals, max_elf)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
