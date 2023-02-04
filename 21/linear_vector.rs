use std::ops::{Add,Sub,Mul,Div};

use rational::{StdInt,Rational,R0, R1};
use token::{Operation,Token};
use formula::Formula;


// This method assumes we have only one unknown (which is the case for now)
// This method assumes we won't end up with higher order terms for that one variable)
#[derive(Debug)]
pub struct LinearVector {
    name: String,
    constant: Rational,
    coeff: Rational,
}

impl LinearVector {
    pub fn new(constant: Rational, coeff: Rational, name: &String) -> Self {
        return Self {constant: constant, coeff: coeff, name: name.to_string()};
    }

    pub fn from_ints(constant: StdInt, coeff: StdInt, name: &String) -> Self {
        return Self::new(Rational::from_int(constant), Rational::from_int(coeff), name);
    }

    pub fn create_copy(&self) -> Self {
        return Self::new(self.constant, self.coeff, &self.name);
    }

    pub fn to_formula(&self) -> Formula {
        if (self.coeff == R0) && (self.constant == R0) {
            return Formula::new(vec![Token::Constant(R0)]);
        }
        else if self.coeff == R0 {
            return Formula::new(vec![Token::Constant(self.constant)]);
        }
        else if self.constant == R0 {
            return Formula::new(vec![Token::Term(self.coeff, (&self.name).to_string())]);
        }
        else {
            return Formula::new(vec![Token::Term(self.coeff, (&self.name).to_string()), Token::Op(Operation::Addition), Token::Constant(self.constant)])
        }
    }

    pub fn from_simple_formula(formula: Formula, name: &String) -> Result<Self, (String, Formula)> {
        let formula_length = formula.get_formula().len();
        return match formula_length {
            1 => Self::from_1_term_formula(formula, name),
            3 => Self::from_3_term_formula(formula, name),
            2 => Err(("Formula malformed".to_string(), formula)),
            _ => Err(("Formula is too long".to_string(), formula)),
        };
    }

    fn from_3_term_formula(formula: Formula, name: &String) -> Result<Self, (String, Formula)> {
        if formula.get_formula().len() != 3 {panic!{"This method only handles 3 term formulae!"};}
        if formula.get_variable_names().len() > 1 {return Err(("Too many unknowns to reduce".to_string(), formula));}
        if let Token::Op(operation) = formula.get_formula()[1] {
            let result1 = Self::from_1_term_formula(Formula::new(vec![formula.get_formula()[0].create_copy()]), name);
            let result2 = Self::from_1_term_formula(Formula::new(vec![formula.get_formula()[2].create_copy()]), name);
            return match (operation, result1, result2) {
                (Operation::Addition, Ok(term1), Ok(term2)) => Ok(term1 + term2),
                (Operation::Subtraction, Ok(term1), Ok(term2)) => Ok(term1 - term2),
                (Operation::Multiplication, Ok(term1), Ok(term2)) => Ok(term1 * term2),
                (Operation::Division, Ok(term1), Ok(term2)) => Ok(term1 / term2),
                (_, Err((reason, _)), _) | (_, _, Err((reason, _))) => Err(("Issue with one of the terms: ".to_string() + &reason, formula)),
                (Operation::Equals, _, _) => Err(("Formula is an equation!".to_string(), formula)),
            };
        }
        else {
            return Err(("Formula is invalid!".to_string(), formula));
        }

    }

    fn from_1_term_formula(formula: Formula, name: &String) -> Result<Self, (String, Formula)> {
        let var_names = formula.get_variable_names();
        if var_names.len() > 1 {panic!("Can't create a reduced formula when there is more than one variable unknown!");}
        else if (var_names.len() == 1) && (var_names[0] != name.to_string()) {panic!("Variable name introduced is different from what was specified: {}, {}", name, var_names[0]);}
        if formula.get_formula().len() != 1 {panic!{"This method only handles 1 term formulae!"};}
        return match formula.get_formula()[0] {
            Token::Constant(num) => Ok(Self::new(num, R0, name)),
            Token::Term(num, _) => Ok(Self::new(R0, num, name)),
            Token::Variable(_) => Ok(Self::new(R0, R1, name)),
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
        if (self.coeff != R0) && (other.coeff != R0) {panic!("LinearVector only supports linear terms!");}
        return Self::new(self.constant * other.constant, self.coeff*other.constant + other.coeff*self.constant, &self.name)
    }
}

impl Div for LinearVector { 
    type Output = Self;
    fn div(self, other: Self) -> Self {
        if self.name != other.name {panic!{"Multiple variables not supported!"};}
        if (self.coeff != R0) && (other.coeff != R0) {panic!("LinearVector only supports linear terms!");}
        if other.coeff == R0 {
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
