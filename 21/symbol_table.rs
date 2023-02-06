use std::collections::HashMap;

use rational::Rational;
use token::{Operation,Token};
use linear_vector::{LinearVector,NO_VAR};
use formula::Formula;

#[derive(Debug)]
pub struct SymbolTable {
    table: HashMap<String, Formula>,
    constraint: Option<Formula>,
}

impl SymbolTable {
    pub fn new() -> Self {
        return Self {table: HashMap::new(), constraint: None};
    }

    pub fn add_symbol(&mut self, name: &String, formula: Formula) {
        self.table.insert(name.to_string(), formula);
    }

    pub fn add_constraint(&mut self, formula: Formula) {
        self.constraint = Some(formula);
    }

    pub fn add_symbol_from_string(&mut self, equation_str: &String) {
        let vec_str: Vec<String> = equation_str.split(": ").map(|x| x.to_string()).collect();
        let symbol_name: &String = &vec_str[0];
        let formula_str: &String = &vec_str[1];
        let formula: Formula = Formula::from_string(&formula_str);
        if (formula.get_formula().len() > 1) && formula.get_formula()[1] == Token::Op(Operation::Equals) {
            self.add_constraint(formula);
        }
        else {
            self.add_symbol(symbol_name, formula);
        }
    }

    pub fn evaluate_variable(&mut self, variable_name: &String) -> Result<Rational, &str> {
        let reduced: Result<LinearVector, &str> = self.reduce_variable(variable_name, &(NO_VAR.to_string()));
        return match reduced {
            Ok(lin_vec) => Ok(lin_vec.get_constant()),
            Err(msg) => Err(msg),
        };
    }

    pub fn reduce_variable(&mut self, variable_name: &String, subject: &String) -> Result<LinearVector, &str> {
        println!("Reducing {}...", &variable_name);
        let mut current_formula = self.table.get(variable_name).unwrap().create_copy();
        let variables_to_evaluate: &Vec<String> = &(self.table.get(variable_name).unwrap().get_variable_names());
        for name in variables_to_evaluate {
            if name == subject {continue;}
            let ans = self.reduce_variable(name, subject);
            if let Ok(num) = ans {
                current_formula.sub_linear_vec(name, num);
            }
            else {
                return Err("Formula currently not reducible in terms of the given subject");
            }
        }
        match current_formula.reduce_to_linear_vector(subject) {
            Ok(ans) => {
                println!("{} reduces to {:?} after recursion", &variable_name, &ans);
                self.add_symbol(variable_name, current_formula);
                return Ok(ans);
            },
            Err(_) => return Err("Variable not currently reducible in terms of the given subject"),
        };
    }

    
    pub fn solve_for_symbol(&mut self, subject: &String) -> Result<Rational, &str> {
        let left_symbol = self.constraint.as_ref().expect("No constraint found").get_formula()[0].create_copy();
        let right_symbol = self.constraint.as_ref().expect("No constraint found").get_formula()[2].create_copy();
        let left: LinearVector;
        let right: LinearVector;
        
        if let Token::Variable(left_name) = left_symbol {
            self.reduce_variable(&left_name, subject);
            left = self.table.get(&left_name).unwrap().get_reduces_to().unwrap();
            println!("Solve Left: {:?}", left);
        }else {return Err("Malformed formula");}

        if let Token::Variable(right_name) = right_symbol {
            self.reduce_variable(&right_name, subject);
            right = self.table.get(&right_name).unwrap().get_reduces_to().unwrap();
            println!("Solved Right: {:?}", right);
        }else {return Err("Malformed formula");}
    
        let equals_zero = left - right;
        return Ok(-equals_zero.get_constant()/equals_zero.get_coeff());
    }
}
