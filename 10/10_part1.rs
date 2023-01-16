use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const CHECK_START_CYCLE: usize = 20;
const CHECK_CYCLE_FREQUENCY: usize = 40;

#[allow(dead_code)]
struct State {
    register: i32,
    cycle: u32,
}

struct Clock {
    current_register: i32,
    current_cycle: u32,
    history: Vec<State>,
}

impl Clock {
    fn new() -> Clock {
        return Clock {current_register: 1, current_cycle: 0, history: vec![State {register: 1, cycle: 0}]};
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
        self.current_cycle += 1;
        let new_state = State {register: self.current_register, cycle: self.current_cycle};
        self.history.push(new_state);
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

    println!("Max cycle: {}", clock.current_cycle);
    let mut output: i32 = 0;
    let mut check_cycle: usize = CHECK_START_CYCLE;
    loop {
        let cycle_register = clock.history[check_cycle].register;
        let signal_strength: i32 = (check_cycle as i32) * cycle_register;
        println!("{}, {}, {}", check_cycle, cycle_register, signal_strength);
        output += signal_strength;
        check_cycle += CHECK_CYCLE_FREQUENCY;
        if check_cycle as u32 > clock.current_cycle {
            break;
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

