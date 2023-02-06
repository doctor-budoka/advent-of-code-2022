use std::collections::HashMap;

use rational::{Rational,R0,R1};
use token::{Token};
use linear_vector::{LinearVector,NO_VAR};

#[derive(Debug)]
pub struct Formula {
    formula: Vec<Token>,
    substitutions: HashMap<String, LinearVector>,
    reduces_to: Option<LinearVector>,
    evaluates_to: Option<Rational>,
}

impl Formula {
    pub fn new(formula: Vec<Token>) -> Self {
        return Self{formula: formula, substitutions: HashMap::new(), reduces_to: None, evaluates_to: None};
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

    pub fn sub_linear_vec(&mut self, variable: &String, value: LinearVector) {
        self.substitutions.insert(variable.to_string(), value);
    }

    pub fn sub_value(&mut self, variable: &String, value: Rational) {
        self.substitutions.insert(variable.to_string(), LinearVector::constant_from_rational(value, &(NO_VAR.to_string())));
    }

    fn set_reduces_to(&mut self, value: &LinearVector){
        self.reduces_to = Some(value.clone());
        if (&value).get_coeff() == R0 {self.evaluates_to = Some(value.get_constant());}
    }

    fn set_evaluates_to(&mut self, value: &Rational){
        self.evaluates_to = Some(*value);
    }

    pub fn get_reduces_to(&self) -> Option<LinearVector> {
        return self.reduces_to.as_ref().cloned();
    }

    pub fn reduce_to_linear_vector(&mut self, subject: &String) -> Result<LinearVector, &str> {
        if let Some(num) = self.evaluates_to {
            let lin_vec: LinearVector = LinearVector::constant_from_rational(num, &subject);
            self.set_reduces_to(&lin_vec);
            return Ok(lin_vec);
        }
        else if let Some(lin_vec) = &self.reduces_to {
            return Ok(lin_vec.clone());
        }

        let variable_names = self.get_variable_names();
        for name in variable_names {
            if (name != subject.to_string()) && !self.substitutions.contains_key(&name) {
                return Err("Formula contains unknown variable");
            }
        }
        if self.formula.len() == 1 {
            let ans = self.reduce_token(&self.formula[0], &subject).unwrap();
            self.set_reduces_to(&ans);
            return Ok(ans);
        }
        else if self.formula.len() == 3 {
            let left = self.reduce_token(&self.formula[0], &subject).unwrap();
            let right = self.reduce_token(&self.formula[2], &subject).unwrap();
            let ans = match &self.formula[1] {
                Token::Op(operation) => operation.evaluate(left, right),
                other => panic!("Middle token should be a term, not {:?}", other),
            };
            self.set_reduces_to(&ans);
            return Ok(ans);
        }
        else {
            panic!("Formula isn't valid!");
        }
    }

    pub fn reduce_token(&self, term: &Token, subject: &String) -> Result<LinearVector,&str> {
        return match term {
            Token::Constant(num) => Ok(LinearVector::constant_from_rational(*num, subject)),
            Token::Variable(name) => Ok(self.substitutions.get(name).unwrap().clone()),
            Token::Op(_) => Err("Operations aren't valid terms for evaluation on their own isn't a valid term for evaluation"),
            Token::Term(num, name) => Ok(LinearVector::constant_from_rational(*num, subject) * (self.substitutions.get(name).unwrap().clone())),
        };
    }

    pub fn evaluate(&mut self) -> Result<Rational, &str> {
        if let Some(num) = self.evaluates_to {
            return Ok(num);
        }
        else if let Ok(ans) = self.reduce_to_linear_vector(&(NO_VAR.to_string())) {
            return Ok(ans.get_constant());
        }
        else {return Err("Formula isn't currently reducible to a rational")}
    }


    pub fn create_copy(&self) -> Self {
        let values_copy: HashMap<String, LinearVector> = self.substitutions.iter().map(|(x, y)| ((&x).to_string(), y.clone())).collect();
        return Self {
            formula: self.formula.iter().map(|x| x.create_copy()).collect(),
            substitutions: HashMap::from(values_copy),
            reduces_to: match &self.reduces_to {
                Some(lin_vec) => Some(lin_vec.clone()),
                None => None, 
            },
            evaluates_to: self.evaluates_to,
        }
    }
}
