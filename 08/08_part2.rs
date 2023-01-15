use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
                    row.push(num_char as u32 - '0' as u32);
                }
                forest.push(row);
            }
        }
    }

    let width = forest[0].len();
    let length = forest.len();
    let mut best_score: u32 = 0;
    for i in 0..length {
        for j in 0..width {
            let this_score: u32 = calculate_tree_score(&forest, i, j);
            best_score = if this_score > best_score {this_score} else {best_score};
        }
    }
    println!("Highest view score: {}", best_score);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn calculate_tree_score(forest: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    let width = forest[0].len();
    let length = forest.len();
    let current_height: u32 = forest[row][col];
    
    let mut left_count_to_block: u32 = 0;
    if col != 0 {
        for i in 1..(col + 1) {
            left_count_to_block += 1;
            if forest[row][col - i] >= current_height {break;}
        }
    }
    
    let mut right_count_to_block: u32 = 0;
    if col != width - 1 {
        for i in 1..(width - col) {
            right_count_to_block += 1;
            if forest[row][col + i] >= current_height {break;}
        }
    }
    
    let mut up_count_to_block: u32 = 0;
    if row != 0 {
        for i in 1..(row + 1) {
            up_count_to_block += 1;
            if forest[row - i][col] >= current_height {break;}
        }
    }
    
    let mut down_count_to_block: u32 = 0;
    if row != length - 1 {
        for i in 1..(length - row) {
            down_count_to_block += 1;
            if forest[row + i][col] >= current_height {break;}
        }
    }
    return left_count_to_block * right_count_to_block * up_count_to_block * down_count_to_block;
}

