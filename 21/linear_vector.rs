use std::ops::{Add,Sub,Mul,Div};
use std::collections::HashMap;

use token::{StdInt,Operation,Token};
use formula::Formula;

// This method assumes we have only one unknown (which is the case for now)
// This method assumes we won't end up with higher order terms for that one variable)
#[derive(Debug)]
pub struct LinearVector {
    name: String,
    constant: StdInt,
    coeff: StdInt,
}

impl LinearVector {
    pub fn new(constant: StdInt, coeff: StdInt, name: &String) -> Self {
        return Self {constant: constant, coeff: coeff, name: name.to_string()};
    }

    pub fn to_formula(&self) -> Formula {
        if (self.coeff == 0) && (self.constant == 0) {
            return Formula::new(vec![Token::Constant(0)]);
        }
        else if self.coeff == 0 {
            return Formula::new(vec![Token::Constant(self.constant)]);
        }
        else if self.constant == 0 {
            return Formula::new(vec![Token::Term(self.coeff, (&self.name).to_string())]);
        }
        else {
            return Formula::new(vec![Token::Term(self.coeff, (&self.name).to_string()), Token::Op(Operation::Addition), Token::Constant(self.constant)])
        }
    }

    pub fn from_3_term_formula(formula: Formula, name: &String) -> Self {
        if formula.get_variable_names().len() > 1 {panic!("Can't create a reduced formula when there is more than one variable unknown!");}
        if formula.get_formula().len() != 3 {panic!{"This method only handles 3 term formulae!"};}
        if let Token::Op(operation) = formula.get_formula()[1] {
            let term1 = Self::from_1_term_formula(Formula::new(vec![formula.get_formula()[0].create_copy()]), name);
            let term2 = Self::from_1_term_formula(Formula::new(vec![formula.get_formula()[2].create_copy()]), name);
            return match operation {
                Operation::Addition => term1 + term2,
                Operation::Subtraction => term1 - term2,
                Operation::Multiplication => term1 * term2,
                Operation::Division => term1 / term2,
                Operation::Equals => panic!("We don't operate on formulae using =")
            };
        }
        else {
            panic!("3-term formulae should have an operator");
        }

    }

    pub fn from_1_term_formula(formula: Formula, name: &String) -> Self {
        let var_names = formula.get_variable_names();
        if var_names.len() > 1 {panic!("Can't create a reduced formula when there is more than one variable unknown!");}
        else if (var_names.len() == 1) && (var_names[0] != name.to_string()) {panic!("Variable name introduced is different from what was specified: {}, {}", name, var_names[0]);}
        if formula.get_formula().len() != 1 {panic!{"This method only handles 1 term formulae!"};}
        return match formula.get_formula()[0] {
            Token::Constant(num) => Self::new(num, 0, name),
            Token::Term(num, _) => Self::new(0, num, name),
            Token::Variable(_) => Self::new(0, 1, name),
            Token::Op(_) => panic!("Can't create a reduced formula from an operation"),
        };
    }
}

impl Add for LinearVector { 
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if self.name != other.name {panic!{"Multiple variables not supported!"};}
        return Self::new(self.constant + other.constant, self.coeff + other.coeff, &self.name)
    }
}

impl Sub for LinearVector { 
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        if self.name != other.name {panic!{"Multiple variables not supported!"};}
        return Self::new(self.constant - other.constant, self.coeff - other.coeff, &self.name)
    }
}

impl Mul for LinearVector { 
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        if self.name != other.name {panic!{"Multiple variables not supported!"};}
        if (self.coeff != 0) && (other.coeff != 0) {panic!("LinearVector only supports linear terms!");}
        return Self::new(self.constant * other.constant, self.coeff*other.constant + other.coeff*self.constant, &self.name)
    }
}

impl Div for LinearVector { 
    type Output = Self;
    fn div(self, other: Self) -> Self {
        if self.name != other.name {panic!{"Multiple variables not supported!"};}
        if (self.coeff != 0) && (other.coeff != 0) {panic!("LinearVector only supports linear terms!");}
        if other.coeff == 0 {
            return Self {
                constant: self.constant / other.constant, 
                coeff: self.coeff / other.constant, 
                name: self.name,
            }
        }
        else {
            panic!("We're assuming we never have to divide by the unknown variable (big assumption, I know)");
        }
    }
}
