use std::env;
use std::fs;

const MARKER_LENGTH: i32 = 4;


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");
    
    let mut current_window: Vec<char> = Vec::new();
    let mut char_ind: i32 = 0;
    for c in input.chars() {
        current_window.push(c);
        if current_window.len() as i32 == MARKER_LENGTH {
            let is_unique = check_chars_for_uniqueness(&current_window);
            if is_unique {
                println!("Output: {}", char_ind + 1);
                break;
            }
            current_window.remove(0);
        }
        char_ind += 1;
    } 
}


fn check_chars_for_uniqueness(window: &Vec<char>) -> bool {
    return get_num_letters(&window) == MARKER_LENGTH;
}


fn get_num_letters(in_vec: &Vec<char>) -> i32 {
    let mut exists = vec![0; 52];
    for c in in_vec {
        exists[get_char_priority(*c) as usize - 1] = 1;
    }
    return exists.iter().sum();
}


fn get_char_priority(letter: char) -> i32 {
    if letter.is_lowercase() {
        return (letter as i32) - ('a' as i32) + 1;
    }
    else {
        return (letter as i32) - ('A' as i32) + 27;
    }
}
