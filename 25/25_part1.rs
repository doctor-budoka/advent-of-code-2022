use std::env;
use std::fs;

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file"); 
    
    let mut total = 0;
    for (i, line) in input.lines().enumerate() {
        let decimal_ver: i64 = snafu_to_decimal(&(line.to_string()));
        println!("line {}: Original: {}, Decimal: {}", i, &line, decimal_ver);
        total += decimal_ver;
    }

    println!("Total in decimal: {}", total);
    let total_snafu: String = decimal_to_snafu(total);
    println!("Total in SNAFU: {}", total_snafu);
} 

fn snafu_to_decimal(line: &String) -> i64 {
    let mut output = 0;
    for (ind, character) in line.trim().chars().rev().enumerate() {
        let digit_value: i64 = snafu_digit_to_decimal(character);
        let power: u32 = ind as u32;
        let new_value: i64 = digit_value * 5_i64.pow(power);
        output += new_value;
    }
    return output;
}

fn snafu_digit_to_decimal(digit: char) -> i64 {
    return match digit {
        '=' => -2,
        '-' => -1,
        a => a.to_digit(10).expect("Should have been able to parse input") as i64,
    };
}

fn decimal_to_snafu(decimal: i64) -> String {
    let base5: String = decimal_to_base5(decimal);
    println!("base5: {}", base5);
    return base5_to_snafu(base5);
}

fn decimal_to_base5(decimal: i64) -> String {
    let length: usize = (decimal as f64).log(5.0) as usize + 1;
    let mut out_vec: Vec<i64> = Vec::new();
    let mut leftover: i64 = decimal;
    for i in (0..length).rev() {
        let divisor = 5_i64.pow(i as u32);
        let new_digit = leftover / divisor;
        leftover -= new_digit * divisor;
        out_vec.push(new_digit);
    }
    return out_vec.iter().map(|x| char::from_digit((*x) as u32, 5).unwrap()).collect::<String>();
}

fn base5_to_snafu(base5: String) -> String {
    let length: usize = base5.len();
    let mut snafu_vec: Vec<char> = vec!['0'; length];
    let mut carry: char = '0';
    for (ind, digit) in base5.chars().rev().enumerate() {
        let (_, snafu_val): (char, char) = sum_snafu_digits(snafu_vec[ind], carry);
        let (this_carry, new_val): (char, char) = sum_snafu_digits(snafu_val, digit);
        carry = this_carry;
        snafu_vec[ind] = new_val;
    }
    return snafu_vec.into_iter().rev().collect::<String>();
}

fn sum_snafu_digits(digit1: char, digit2: char) -> (char, char) {
    let dec1: i64 = snafu_digit_to_decimal(digit1);
    let dec2: i64 = snafu_digit_to_decimal(digit2);
    let result_dec: i64 = dec1 + dec2;
    return base5_digit_to_snafu(result_dec);
}

fn base5_digit_to_snafu(digit: i64) -> (char, char) {
    return match digit {
        5 => ('1', '0'),
        4 => ('1', '-'),
        3 => ('1', '='),
        a => ('0', char::from_digit(a as u32, 10).unwrap()),
    };
}
