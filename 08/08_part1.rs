use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


type Visibility = Option<bool>;

struct Tree {
    height: u32,
    // (from left, from right, from top, from bottom)
    visibility_array: (Visibility, Visibility, Visibility, Visibility)
}

impl Tree {
    fn new(height: u32) -> Tree {
        return Tree {
            height: height, 
            visibility_array: (None, None, None, None),
        } 
    }

    fn is_visible(&self) -> Visibility {
        return match &self.visibility_array {
            (Some(left), Some(right), Some(top), Some(down)) => Some(left | right | top | down),
            _ => None,
        };
    }
}


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    
    let mut forest = Vec::new();
    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(val) = line {
                let mut row = Vec::new();
                for num_char in val.chars() {
                    row.push(Tree::new(num_char as u32 - '0' as u32));
                }
                forest.push(row);
            }
        }
    }
    let width = forest[0].len();
    let length = forest.len();

    println!("forest size: {} {}", width, length);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

