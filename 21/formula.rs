use std::collections::HashMap;

use rational::{Rational,R1};
use token::{Token};
use linear_vector::LinearVector;

#[derive(Debug)]
pub struct Formula {
    formula: Vec<Token>,
    substitutions: HashMap<String, Rational>,
    evaluates_to: Option<Rational>,
}

impl Formula {
    pub fn new(formula: Vec<Token>) -> Self {
        return Self {formula: formula, substitutions: HashMap::new(), evaluates_to: None};
    }

    pub fn from_string(formula_str: &String) -> Self {
        let clean_str: String = formula_str.trim().to_string();
        let tokens: Vec<&str> = clean_str.split_whitespace().collect();
        let token_vector: Vec<Token>;
        if tokens.len() == 1 {
            let new_token = Token::from_string(&tokens[0].to_string());
            token_vector = match new_token {
                Token::Constant(_) => vec![new_token],
                Token::Variable(name) => vec![Token::Term(R1, name)],
                _ => panic!("'{:?}' is not a valid formula", &formula_str),
            };
        }
        else if tokens.len() == 3 {
            token_vector = tokens.iter().map(|x| Token::from_string(&x.to_string())).collect();
        }
        else {
            panic!("'{}' isn't a valid formula", formula_str);
        }

        return Self::new(token_vector);
    }

    pub fn get_formula(&self) -> &Vec<Token> {
        return &self.formula;
    }

    pub fn get_variable_names(&self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for token in &self.formula {
            match token {
                Token::Variable(name) => output.push(name.to_string()),
                _ => (),
            };
        }
        return output;
    }

    pub fn sub_value(&mut self, variable: &String, value: Rational) {
        self.substitutions.insert(variable.to_string(), value);
    }

    pub fn evaluate(&mut self) -> Option<Rational> {
        if let Some(num) = self.evaluates_to {
            return Some(num);
        }

        let variable_names = self.get_variable_names();
        for name in variable_names {
            if !self.substitutions.contains_key(&name) {
                return None;
            }
        }
        return Some(self.evaluate_if_variables_known());
    }

    pub fn evaluate_if_variables_known(&mut self) -> Rational {
        if self.formula.len() == 1 {
            let ans = self.evaluate_term(&self.formula[0]);
            self.evaluates_to = Some(ans);
            return ans;
        }
        else if self.formula.len() == 3 {
            let left = self.evaluate_term(&self.formula[0]);
            let right = self.evaluate_term(&self.formula[2]);
            let ans = match &self.formula[1] {
                Token::Op(operation) => operation.evaluate(left, right),
                other => panic!("Middle token should be a term, not {:?}", other),
            };
            self.evaluates_to = Some(ans);
            return ans;
        }
        else {
            panic!("Formula isn't valid!");
        }

    }

    pub fn evaluate_term(&self, term: &Token) -> Rational {
        return match term {
            Token::Constant(num) => *num,
            Token::Variable(name) => *self.substitutions.get(name).unwrap(),
            Token::Op(op) => panic!("'{:?} isn't a valid term for evaluation'", op),
            Token::Term(num, name) => (*num) * (*self.substitutions.get(name).unwrap()),
        };
    }

    pub fn create_copy(&self) -> Self {
        let values_copy: HashMap<String, Rational> = self.substitutions.iter().map(|(x, y)| ((&x).to_string(), *y)).collect();
        return Self {
            formula: self.formula.iter().map(|x| x.create_copy()).collect(),
            substitutions: HashMap::from(values_copy),
            evaluates_to: self.evaluates_to,
        }
    }
}
