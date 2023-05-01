use std::env;
use std::fs;

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file"); 
    
    let mut total = 0;
    for (i, line) in input.lines().enumerate() {
        let decimal_ver: i32 = snafu_to_decimal(&(line.to_string()));
        println!("line {}: Original: {}, Decimal: {}", i, &line, decimal_ver);
        total += decimal_ver;
    }

    println!("Total in decimal: {}", total);
} 

fn snafu_to_decimal(line: &String) -> i32 {
    let mut output = 0;
    for (ind, character) in line.trim().chars().rev().enumerate() {
        let digit_value: i32 = snafu_digit_to_decimal(character);
        let power: u32 = ind as u32;
        let new_value: i32 = digit_value * 5_i32.pow(power);
        output += new_value;
    }
    return output;
}

fn snafu_digit_to_decimal(digit: char) -> i32 {
    return match digit {
        '=' => -2,
        '-' => -1,
        a => a.to_digit(10).expect("Should have been able to parse input") as i32,
    };
}
