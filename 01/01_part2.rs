use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    
    let mut top_3 = vec![-1, -1, -1];
    let mut this_cals = 0;
    if let Ok(lines) = read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(val) = line {
                if val == "" {
                    top_3.push(this_cals);
                    top_3.sort_by(|a, b| b.cmp(a));
                    top_3.truncate(3);

                    this_cals = 0;
                }
                else {
                    let int_val = val.parse::<i64>().unwrap();
                    this_cals = this_cals + int_val;
                }
            }
        }
    }
    println!("1: {}, 2: {}, 3: {}", top_3[0], top_3[1], top_3[2]);
    let total: i64 = top_3.iter().sum();
    println!("total: {}", total);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
