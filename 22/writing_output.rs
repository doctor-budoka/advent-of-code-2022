use std::fs;
use space::{Marker};

pub fn write_steps(steps: Vec<Marker>, file: &String) {
    let mut body: String = "".to_string();
    for (i, step) in steps.iter().enumerate() {
        let position = step.get_position();
        let direction_char = step.get_direction().as_char();
        let new_line: String = format!("{i}: {position}, {direction_char}\n");
        body += &new_line;
    }
    fs::write(file, body).expect("Unable to write file");
}