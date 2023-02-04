use std::collections::HashMap;

use rational::Rational;
use token::{Operation,Token};
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

    pub fn evaluate_variable(&mut self, variable_name: &String) -> Option<Rational> {
        let mut variable_formula: Formula = self.table.get(variable_name).unwrap().create_copy();
        match variable_formula.evaluate() {
            None => {
                let variables_to_evaluate: Vec<String> = variable_formula.get_variable_names();
                for name in &variables_to_evaluate {
                    let ans = self.evaluate_variable(name);
                    if let Some(num) = ans {
                        variable_formula.sub_value(name, num);
                    }
                    else {
                        return None;
                    }
                }
                return variable_formula.evaluate();
            },
            Some(num) => return Some(num),
        };
    }
    
    // pub fn solve_for_symbol(&mut self, symbol: &String) -> Rational {
    //     let constraint = self.constraint.as_ref().unwrap().create_copy();
    //     let left = self.reduce_variable(constraint.formula[0].create_copy());
    //     let right = self.reduce_variable(constraint.formula[2].create_copy());
    //     let symbol_formula = self.rearrange_equation(left, right, symbol);
    //     return symbol_formula.evaluate();
    // }

    // pub fn reduce_variable(&mut self, symbol: Token) -> Formula {
    //     return Formula::new(vec![Token::Constant(0)]);
    // }
}
