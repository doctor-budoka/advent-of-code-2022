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

    // pub fn evaluate_variable(&mut self, variable_name: &String) -> Result<Rational, &str> {
    //     let mut variable_formula: Formula = self.table.get(variable_name).unwrap().create_copy();
    //     match variable_formula.evaluate() {
    //         Err(_) => {
    //             let variables_to_evaluate: Vec<String> = variable_formula.get_variable_names();
    //             for name in &variables_to_evaluate {
    //                 let ans = self.evaluate_variable(name);
    //                 if let Ok(num) = ans {
    //                     variable_formula.sub_value(name, num);
    //                 }
    //                 else {
    //                     return Err("Formula can't currently be evaluated as a rational.");
    //                 }
    //             }
    //             return variable_formula.evaluate();
    //         },
    //         Ok(num) => return Ok(num),
    //     };
    // }

    pub fn evaluate_variable(&mut self, variable_name: &String) -> Result<Rational, &str> {
        let reduced: Result<LinearVector, &str> = self.reduce_variable(variable_name, &(NO_VAR.to_string()));
        return match reduced {
            Ok(lin_vec) => Ok(lin_vec.get_constant()),
            Err(msg) => Err(msg),
        };
    }

    pub fn reduce_variable(&mut self, variable_name: &String, subject: &String) -> Result<LinearVector, &str> {
        let mut variable_formula: Formula = self.table.get(variable_name).unwrap().create_copy();
        match variable_formula.reduce_to_linear_vector(subject) {
            Err(_) => {
                let variables_to_evaluate: Vec<String> = variable_formula.get_variable_names();
                for name in &variables_to_evaluate {
                    if name == subject {continue;}
                    let ans = self.reduce_variable(name, subject);
                    if let Ok(num) = ans {
                        variable_formula.sub_linear_vec(name, num);
                    }
                    else {
                        return Err("Formula currently not reducible in terms of the given subject");
                    }
                }
                match variable_formula.reduce_to_linear_vector(subject) {
                    Ok(_) => (),
                    Err(_) => return Err("Variale not currently reducible in terms of the given subject"),
                };
                return Ok(variable_formula.get_reduces_to().unwrap());
            },
            Ok(lin_vec) => return Ok(lin_vec),
        };
    }
    
    pub fn solve_for_symbol(&mut self, subject: &String) -> Result<Rational, &str> {
        let constraint = self.constraint.as_ref().unwrap().create_copy();
        let left: LinearVector;
        let right: LinearVector;
        if let Token::Variable(left_name) = constraint.get_formula()[0].create_copy() {
            self.reduce_variable(&left_name, subject);
            left = self.table.get(&left_name).unwrap().get_reduces_to().unwrap();
        }else {return Err("Malformed formula");}
        if let Token::Variable(right_name) = constraint.get_formula()[2].create_copy() {
            self.reduce_variable(&right_name, subject);
            right = self.table.get(&right_name).unwrap().get_reduces_to().unwrap();
        }else {return Err("Malformed formula");}
        let equals_zero = left - right;
        return Ok(-equals_zero.get_constant()/equals_zero.get_coeff());
    }

    // pub fn reduce_variable(&mut self, var_name, subject: Token) -> Formula {
    //     return Formula::new(vec![Token::Constant(0)]);
    // }
}
