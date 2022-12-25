use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


struct CrateStack {
    crates: Vec<char>,
}

impl CrateStack {
    fn put_crate_on_stack(&mut self, val: char) {
        self.crates.push(val);
    }

    fn pull_crate_off_stack(&mut self) -> char {
        return self.crates.pop().expect("There should be a crate here!");
    }

    fn new() -> CrateStack {
        return CrateStack {crates: Vec::new()};
    }

    fn reverse_stack_after_init(&mut self) {
        self.crates.reverse();
    }
}


struct CrateStacks {
    stacks:  Vec<CrateStack>,
    num_stacks: i32,
}

impl CrateStacks {
    fn put_crate_on_stack_a(&mut self, a:i32, val: char) {
        self.stacks[a as usize].put_crate_on_stack(val);
    }

    fn pull_crate_off_stack_a(&mut self, a:i32) -> char {
        return self.stacks[a as usize].pull_crate_off_stack();
    }

    fn move_crate_from_a_to_b(&mut self, a: i32, b: i32) {
        let elf_crate: char = self.pull_crate_off_stack_a(a);
        self.put_crate_on_stack_a(b, elf_crate);
    }

    fn move_n_crates_from_a_to_b(&mut self, n: i32, a: i32, b: i32) {
        for _ in 1..n+1 {
            self.move_crate_from_a_to_b(a, b);
        }
    }

    fn read_top_crates(&self) {
        let mut output: String = "".to_string();
        for i in 0..self.num_stacks {
            let new_char = self.stacks[i as usize].crates.last().expect("Stack should have a crate!"); 
            output += &new_char.to_string();
        }
        println!("{}", output);
    }

    fn reverse_stacks_after_init(&mut self) {
        for i in 0..self.num_stacks {
            self.stacks[i as usize].reverse_stack_after_init();
        }
    }

    fn new(num_stacks: i32) -> CrateStacks {
        let mut stacks: Vec<CrateStack> = Vec::new();
        for _ in 0..num_stacks {
            stacks.push(CrateStack::new());
        }
        return CrateStacks {stacks: stacks, num_stacks: num_stacks};
    }
}

#[derive(PartialEq)]
enum InputState {
    Initialise,
    Rearrange,
}


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    
    let mut line_num = 0;
    let mut state = InputState::Initialise;
    let mut stacks: Option<CrateStacks> = None;
    if let Ok(lines) = read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(val) = line {
                if line_num == 0 {
                    stacks = initialise_stacks(val);
                }
                else if val == "" {
                    state = InputState::Rearrange;
                }
                else if state == InputState::Initialise {
                    stacks = update_init_stack(stacks, val);
                }
                else {
                    stacks = update_stack(stacks, val);
                }
                line_num += 1;
                // if state == InputState::Rearrange {&stacks.expect("Stack should exist!").read_top_crates();}
            }
        }
    }
    stacks.expect("Stack should exist!").read_top_crates();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn initialise_stacks(top_slice: String) -> Option<CrateStacks> {
    let length: i32 = top_slice.len() as i32;
    let num_stacks = (length + 1)/4;

    let mut stacks = Some(CrateStacks::new(num_stacks));
    stacks = update_init_stack(stacks, top_slice);
    return stacks;
}


fn update_init_stack(option_stacks: Option<CrateStacks>, stack_slice: String) -> Option<CrateStacks> {
    let mut stacks = option_stacks.expect("Should have a stack here!");
    for i in 0..stacks.num_stacks {
        let crate_ind: i32 = i*4 + 1;
        let crate_value: char = stack_slice.chars().nth(crate_ind as usize).unwrap();
        if crate_value == '1' {
            stacks.reverse_stacks_after_init();
            return Some(stacks);
        }
        else if crate_value == ' ' {

        }
        else {
            stacks.put_crate_on_stack_a(i, crate_value);
        }
    }
    return Some(stacks);
}

fn update_stack(option_stacks: Option<CrateStacks>, instructions: String) -> Option<CrateStacks> {
    let mut stacks = option_stacks.expect("Should have a stack here!");
    let instr_vec: Vec<&str> = instructions.split(" ").collect();
    let num_crates: i32 = instr_vec[1].parse::<i32>().unwrap();
    let src_stack: i32 = instr_vec[3].parse::<i32>().unwrap() - 1;
    let dest_stack: i32 = instr_vec[5].parse::<i32>().unwrap() -1;

    stacks.move_n_crates_from_a_to_b(num_crates, src_stack, dest_stack);
    return Some(stacks);
}

