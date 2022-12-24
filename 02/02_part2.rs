use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[derive(PartialEq, Copy, Clone)]
enum RPSOption {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
struct RPSMove {
    move_type: RPSOption
}

impl RPSMove {
    fn from_letter(letter: char) -> RPSMove {
        let option_enum = match letter {
            'A' => RPSOption::Rock, 
            'B' => RPSOption::Paper,
            'C' => RPSOption::Scissors,
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

    fn get_weakness(&self) -> RPSOption {
        match &self.move_type {
            RPSOption::Rock => RPSOption::Paper,
            RPSOption::Paper => RPSOption::Scissors,
            RPSOption::Scissors => RPSOption::Rock,
        }
    }

    fn get_strength(&self) -> RPSOption {
        match &self.move_type {
            RPSOption::Rock => RPSOption::Scissors,
            RPSOption::Paper => RPSOption::Rock,
            RPSOption::Scissors => RPSOption::Paper,
        }
    }
}


#[derive(PartialEq, Copy, Clone)]
enum RoundResultOption {
    Win,
    Loss,
    Draw,
}

struct RoundResult {
    result: RoundResultOption
}

impl RoundResult {
    fn from_letter(letter: char) -> RoundResult {
        let option_enum = match letter {
            'X' => RoundResultOption::Loss, 
            'Y' => RoundResultOption::Draw,
            'Z' => RoundResultOption::Win,
            _ => panic!("Incorrect value for Round Result Option {}", letter),
        };
        return RoundResult{result: option_enum};
    }

    fn round_score(&self) -> i32 {
        match self.result {
            RoundResultOption::Loss => 0,
            RoundResultOption::Draw => 3,
            RoundResultOption::Win => 6,
        }
    }

    fn required_move(&self, &opponent_move: &RPSMove) -> RPSMove {
        let required_result = &self.result;

        let move_option = match required_result {
            RoundResultOption::Win => opponent_move.get_weakness(),
            RoundResultOption::Loss => opponent_move.get_strength(),
            RoundResultOption::Draw => opponent_move.move_type,
        };
        return RPSMove {move_type: move_option};
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
    let strat_result = RoundResult::from_letter(strat_letter);
    let round_score = strat_result.round_score();

    let strat_move = strat_result.required_move(&opponent_move);
    let move_score = strat_move.move_score();

    return move_score + round_score;
}
