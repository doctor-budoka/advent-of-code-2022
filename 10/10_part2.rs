use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::stdout;
use std::io::Write;

type Position = (u32, u32);

struct Clock {
    current_register: i32,
    current_pixel: Position,
    current_cycle: u32,
}

impl Clock {
    fn new() -> Clock {
        return Clock {current_register: 1, current_cycle: 0, current_pixel: (0, 0)};
    }

    fn noop(&mut self) {
        self.increment_cycle();
    }

    fn addx(&mut self, value: i32) {
        self.increment_cycle();
        self.increment_cycle();
        self.current_register += value;
    }

    fn increment_cycle(&mut self) {
        self.current_pixel.0 = self.current_cycle % 40;
        self.current_pixel.1 = (self.current_cycle - self.current_pixel.0 as u32)/40;
        self.current_cycle += 1;

        if self.current_pixel.0 == 0 {
            print!("\n");
            stdout().flush().expect("This should print");
        }
        if (self.current_register - self.current_pixel.0 as i32).abs() <= 1 {
            print!("{}", '#');
        }
        else {
            print!("{}", '.');
        }
    }
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    
    let mut clock =  Clock::new();
    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(val) = line {
                if val == "noop".to_string() {
                    clock.noop();
                }
                else if val.starts_with("addx ") {
                    let instructions: Vec<&str> = val.split(" ").collect();
                    let to_add: i32  = instructions[1].trim().parse().unwrap();
                    clock.addx(to_add);
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

