use std::collections::HashMap;

pub type StdInt = i32;

#[derive(Debug)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Operation {
    pub fn evaluate(&self, left: StdInt, right: StdInt) -> StdInt {
        return match self {
            Self::Addition => left + right,
            Self::Subtraction => left - right,
            Self::Multiplication => left * right,
            Self::Division => left / right,
        }
    }

    pub fn from_char(op_char: char) -> Result<Self, char> {
        return match op_char {
            '+' => Ok(Self::Addition),
            '-' => Ok(Self::Subtraction),
            '*' => Ok(Self::Multiplication),
            '/' => Ok(Self::Division),
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

#[derive(Debug)]
pub enum Token {
    Variable(String),
    Constant(StdInt),
    Op(Operation),
}

impl Token {
    pub fn from_string(string: &String) -> Token {
        if let Ok(int) = string.parse::<StdInt>() {
            return Token::Constant(int);
        }
        else if let Ok(op) = Operation::from_string(string) {
            return Token::Op(op);
        }
        else {
            return Token::Variable(string.to_string());
        }
    }
}

#[derive(Debug)]
pub struct Formula {
    formula: Vec<Token>,
    evaluates_to: Option<StdInt>,
}

impl Formula {
    pub fn new(formula: Vec<Token>) -> Self {
        return Self {formula: formula, evaluates_to: None}
    }

    pub fn from_string(formula_str: &String) -> Self {
        let clean_str: String = formula_str.trim().to_string();
        let tokens: Vec<&str> = clean_str.split_whitespace().collect();
        let token_vector: Vec<Token>;
        if tokens.len() == 1 {
            token_vector = vec![Token::from_string(&tokens[0].to_string())];
        }
        else if tokens.len() == 3 {
            token_vector = tokens.iter().map(|x| Token::from_string(&x.to_string())).collect();
        }
        else {
            panic!("'{}' isn't a valid formula", formula_str);
        }

        return Self::new(token_vector);
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    table: HashMap<String, Formula>,
}

impl SymbolTable {
    pub fn new() -> Self {
        return Self {table: HashMap::new()};
    }

    pub fn add_symbol(&mut self, name: &String, formula: Formula) {
        self.table.insert(name.to_string(), formula);
    }

    pub fn add_symbol_from_string(&mut self, equation_str: &String) {
        let vec_str: Vec<String> = equation_str.split(": ").map(|x| x.to_string()).collect();
        let symbol_name: &String = &vec_str[0];
        let formula_str: &String = &vec_str[1];
        let formula: Formula = Formula::from_string(&formula_str);
        println!("{:?}", formula);
        self.add_symbol(symbol_name, formula);
    }
}