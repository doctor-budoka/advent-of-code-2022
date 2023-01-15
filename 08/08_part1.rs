use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


type Visibility = Option<bool>;

#[derive(Copy, Clone)]
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

    // from left and right
    for i in 0..length {
        // from left
        let mut tallest = forest[i][0].height;
        forest[i][0].visibility_array.0 = Some(true);
        for j in 1..width {
            let mut this_tree = &mut forest[i][j];
            if this_tree.height > tallest {
                this_tree.visibility_array.0 = Some(true);
                tallest = this_tree.height;
            }
            else {
                this_tree.visibility_array.0 = Some(false);
            }
        }

        //from right
        tallest = forest[i][width - 1].height;
        forest[i][width - 1].visibility_array.1 = Some(true);
        for j in 1..width {
            let mut this_tree = &mut forest[i][width - 1 - j];
            if this_tree.height > tallest {
                this_tree.visibility_array.1 = Some(true);
                tallest = this_tree.height;
            }
            else {
                this_tree.visibility_array.1 = Some(false);
            }
        }
    }

    // from top and bottom
    for j in 0..width {
        let mut tallest = forest[0][j].height;
        forest[0][j].visibility_array.2 = Some(true);
        for i in 1..length {
            let mut this_tree = &mut forest[i][j];
            if this_tree.height > tallest {
                this_tree.visibility_array.2 = Some(true);
                tallest = this_tree.height;
            }
            else {
                this_tree.visibility_array.2 = Some(false);
            }
        }

        //from bottom
        tallest = forest[length - 1][j].height;
        forest[length - 1][j].visibility_array.3 = Some(true);
        for i in 1..length {
            let mut this_tree = &mut forest[length - 1 - i][j];
            if this_tree.height > tallest {
                this_tree.visibility_array.3 = Some(true);
                tallest = this_tree.height;
            }
            else {
                this_tree.visibility_array.3 = Some(false);
            }
        }
    }

    // counting
    let mut visible_count = 0;
    for i in 0..length {
        for j in 0..width {
            if forest[i][j].is_visible().expect("is_visible should be fully calculable!") {
                visible_count += 1;
            }
        }
    }
    println!("Number of visible trees: {}", visible_count);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

