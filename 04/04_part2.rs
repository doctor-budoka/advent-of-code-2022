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
                let line_output = get_line_output(val) as i32;
                output += line_output;
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


fn get_line_output(val: String) -> bool {
    let vec = val.split(",").collect::<Vec<&str>>();
    let range_1 = get_tuple_range_from_str_range(vec[0]);
    let range_2 = get_tuple_range_from_str_range(vec[1]);
    return is_full_intersection(range_1, range_2);
}


fn get_tuple_range_from_str_range(str_range: &str) -> (i32, i32) {
    let range_edges = str_range.split("-").collect::<Vec<&str>>();
    let min_range = range_edges[0].parse::<i32>().unwrap();
    let max_range = range_edges[1].parse::<i32>().unwrap();
    return (min_range, max_range);
}

fn is_full_intersection(range_1: (i32, i32), range_2: (i32, i32)) -> bool {
    if range_1.0 == range_2.0 {
        return true;
    }
    else if range_1.0 < range_2.0 {
        return range_1.1 >= range_2.1;
    }
    else {
        return range_1.1 <= range_2.1;
    }
}
