use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[derive(PartialEq)]
enum RPSOption {
    Rock,
    Paper,
    Scissors,
}

struct RPSMove {
    move_type: RPSOption
}

impl RPSMove {
    fn from_letter(letter: char) -> RPSMove {
        let option_enum = match letter {
            'A' => RPSOption::Rock, 
            'B' => RPSOption::Paper,
            'C' => RPSOption::Scissors,
            'X' => RPSOption::Rock,
            'Y' => RPSOption::Paper,
            'Z' => RPSOption::Scissors,
            _ => panic!("Incorrect value for RPS Option {}", letter),
        };
        return RPSMove{move_type: option_enum};
    }

    fn move_score(&self) -> i32 {
        match self.move_type {
            RPSOption::Rock => 1,
            RPSOption::Paper => 2,
            RPSOption::Scissors => 3,
        }
    }

    fn result(&self, other: &Self) -> i32 {
        if self.move_type == other.move_type {
            return 3;
        }
        let self_option = &self.move_type;
        let other_option = &other.move_type;
        let result = match (self_option, other_option) {
            (RPSOption::Rock, RPSOption::Scissors) => 6,
            (RPSOption::Scissors, RPSOption::Paper) => 6,
            (RPSOption::Paper, RPSOption::Rock) => 6,
            _ => 0,
        };
        return result;
    }
}


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    
    let mut total_score = 0;
    if let Ok(lines) = read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(val) = line {
                let round_score = get_round_score(val);
                total_score += round_score;
            }
        }
    }
    println!("Total score: {}", total_score);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn get_round_score(line: String) -> i32 {
    let opponent_letter: char = line.chars().nth(0).unwrap();
    let opponent_move = RPSMove::from_letter(opponent_letter);
    let strat_letter = line.chars().nth(2).unwrap();
    let strat_move = RPSMove::from_letter(strat_letter);

    let move_score = strat_move.move_score();
    let round_score = strat_move.result(&opponent_move);
    return move_score + round_score;
}
