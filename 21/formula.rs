use std::collections::HashMap;

use token::{StdInt,Operation,Token};
use linear_vector::LinearVector;

#[derive(Debug)]
pub struct Formula {
    formula: Vec<Token>,
    substitutions: HashMap<String, StdInt>,
    evaluates_to: Option<StdInt>,
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
                Token::Variable(name) => vec![Token::Term(1, name)],
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

    pub fn sub_value(&mut self, variable: &String, value: StdInt) {
        self.substitutions.insert(variable.to_string(), value);
    }

    pub fn evaluate(&mut self) -> Option<StdInt> {
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

    pub fn evaluate_if_variables_known(&mut self) -> StdInt {
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

    pub fn evaluate_term(&self, term: &Token) -> StdInt {
        return match term {
            Token::Constant(num) => *num,
            Token::Variable(name) => *self.substitutions.get(name).unwrap(),
            Token::Op(op) => panic!("'{:?} isn't a valid term for evaluation'", op),
            Token::Term(num, name) => (*num) * (*self.substitutions.get(name).unwrap()),
        };
    }

    pub fn create_copy(&self) -> Self {
        let values_copy: HashMap<String, StdInt> = self.substitutions.iter().map(|(x, y)| ((&x).to_string(), *y)).collect();
        return Self {
            formula: self.formula.iter().map(|x| x.create_copy()).collect(),
            substitutions: HashMap::from(values_copy),
            evaluates_to: self.evaluates_to,
        }
    }

    // pub fn reduce(&self) -> Formula {
    //     if let Some(num) = self.reduces_to {
    //         return Some(num);
    //     }
    //     for variable_name in self.get_variable_names() {

    //     }


    //     if self.formula.len() == 1 {
    //         return self.formula.create_copy();
    //     }
    //     else if self.formula.len() == 3 {
    //         return match (self.formula[0], self.formula[1], self.formula[2]) {
    //             (Token::Constant(num1), Token::Operation(op), Token::Constant(cum2)) => vec![Token::Constant(op.evaluate(num1, num2))],
    //             (Token::Variable(var1), Token::Operation(op), Token::Variable(var2)) => 
    //             (Token::Constant(num), Token::Operation(op), Token::Variable(var)) | (Token::Variable(var), Token::Operation(op), Token::Constant(num)) => ,
    //             (Token::Term(num, name), Token::Operation(op), )
    //             (_, _, _)
    //         }
    //     }
    //     else {
    //         panic!("Formula not reduceable: {:?}", self.formula);
    //     }
    // }

    // pub fn reduce_3_term_formula_without_variables(&mut self, formula: Formula) -> Formula {
    //     if formula.len() != 3 {
    //         panic!("This method only takes 3 term formulae!");
    //     } 
    //     if formula.get_variable_names().len() > 0 {
    //         panic!("This method doesn't take formulae with variables");
    //     }
        
    // }

    // pub fn add_reduction(&self, reduction: Formula) {
    //     for term in &reduction {
    //         if let Token::Variable(name) = term {
    //             panic!("Reduced formulae don't have variables! Formula: {:?}", reduction);
    //         }
    //     }
    //     if reduction.len() == 1 {
    //         match reduction[0] {
    //             Token::Constant(num) => {
    //                 self.evaluates_to = num;
    //                 self.reduces_to = reduction;
    //             },
    //             Token::Term(num, name) => self.reduces_to = reduction,
    //             _ => panic!("This formula isn't reduced: {:?}", reduction);
    //         }
    //         self.
    //     }
    //     else if reduction.len() == 3 {
    //         match (reduction[0], reduction[1], reduction[2]) {
    //             (Token::Term(_, _), _, Token::Term(_, _)) => panic!("Can't have two terms in a reduced formula: {:?}", reduction),
    //             (Token::Term(_, _), Token::Op(Operation::Addition), Token::Constant())
    //         }
    //     }
    // }

    // pub fn reduce_term(&mut self, term: &Token) -> Formula {
    //     return match term {
    //         Token::Term(num, string) => Formula::new(vec![Token::Term(*num, *string)]),
    //         Token::Constant(num) => Token::Constant(*num),
    //         Token::Variable(name) => *self.values.get(name).unwrap().reduce(),
    //         Token::Op(op) => panic!("'{:?} isn't a valid term for reduction'", op),
    //     }
    // }
}
