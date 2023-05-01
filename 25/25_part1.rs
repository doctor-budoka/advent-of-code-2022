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
    let total_snafu: String = decimal_to_snafu(total);
    println!("Total in SNAFU: {}", total_snafu);
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

fn decimal_to_snafu(decimal: i32) -> String {
    let base5: String = decimal_to_base5(decimal);
    return base5_to_snafu(base5);
}

fn decimal_to_base5(decimal: i32) -> String {
    let length: usize = (decimal as f32).log(5.0) as usize + 1;
    let mut out_vec: Vec<i32> = Vec::new();
    let mut leftover: i32 = decimal;
    for i in (0..length).rev() {
        let divisor = 5_i32.pow(i as u32);
        let new_digit = leftover / divisor;
        leftover -= new_digit * divisor;
        out_vec.push(new_digit);
    }
    return out_vec.iter().map(|x| char::from_digit((*x) as u32, 5).unwrap()).collect::<String>();
}

fn base5_to_snafu(base5: String) -> String {
    return base5;
}
