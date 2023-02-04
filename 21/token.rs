use rational::{StdInt,Rational};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Equals,
}

impl Operation {
    pub fn evaluate(&self, left: Rational, right: Rational) -> Rational {
        return match self {
            Self::Addition => left + right,
            Self::Subtraction => left - right,
            Self::Multiplication => left * right,
            Self::Division => left / right,
            Self::Equals => Rational::from_int((left == right) as StdInt),
        }
    }

    pub fn from_char(op_char: char) -> Result<Self, char> {
        return match op_char {
            '+' => Ok(Self::Addition),
            '-' => Ok(Self::Subtraction),
            '*' => Ok(Self::Multiplication),
            '/' => Ok(Self::Division),
            '=' => Ok(Self::Equals),
            other => Err(other),
        };
    }

    pub fn from_string(op_string: &String) -> Result<Self, String> {
        if op_string.len() != 1 {
            return Err(op_string.to_string());
        }
        let first_char: char = op_string.chars().next().unwrap();
        if let Ok(op) = Self::from_char(first_char) {
            return Ok(op);
        }
        else {
            return Err(op_string.to_string());
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum Token {
    Term(Rational, String),
    Variable(String),
    Constant(Rational),
    Op(Operation),
}

impl Token {
    pub fn from_string(string: &String) -> Token {
        if let Ok(int) = string.parse::<StdInt>() {
            return Token::Constant(Rational::from_int(int));
        }
        else if let Ok(op) = Operation::from_string(string) {
            return Token::Op(op);
        }
        else {
            return Token::Variable(string.to_string());
        }
    }

    pub fn create_copy(&self) -> Self {
        return match self {
            Self::Op(op) => Self::Op(*op),
            Self::Constant(num) => Self::Constant(*num),
            Self::Variable(string) => Self::Variable((&string).to_string()),
            Self::Term(num, string) => Self::Term(*num, (&string).to_string()),
        }
    }
}
