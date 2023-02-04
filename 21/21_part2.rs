use std::env;
use std::fs;

mod token;
use token::Operation;
mod linear_vector;
mod formula;
mod symbol_table;
use symbol_table::SymbolTable;


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);

    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");

    let mut symbol_table: SymbolTable = SymbolTable::new();
    for line in input.trim().lines().collect::<Vec<&str>>() {
        if line.starts_with(&String::from("humn")) {
            symbol_table.add_symbol_from_string(&String::from("humn: humn"));
        }
        if line.starts_with(&String::from("root")) {
            symbol_table.add_symbol_from_string(&line.trim().replace("+", "=").to_string());
        }
        else {
            symbol_table.add_symbol_from_string(&line.trim().to_string());
        }
    }
    // println!("{:?}", &symbol_table);
    println!("Data loaded. Evaluating humn...");

    // let ans = symbol_table.solve_for_symbol(&String::from("humn"));
    // println!("humn answer: {}", ans);
} 


fn rearrange_equation(current_subject: &String, current_formula: Vec<String>, new_subject: &String) -> String {
    if current_formula.len() != 3 {
        panic!(
            "Equation is incorrect! current_subject: {}, current_formula: {:?}, new_subject: {}", 
            current_subject, &current_formula, new_subject
        )
    }
    let var1: String = (&current_formula[0]).to_string();
    let op: String = (&current_formula[1]).to_string();
    let var2: String = (&current_formula[2]).to_string();
    let other_var: String = if var1 == new_subject.to_string() {var2} else {(&var1).to_string()};
    let beginning: String = vec![new_subject.to_string(), ": ".to_string()].join("");
    return match (var1 == new_subject.to_string(), Operation::from_string(&op).unwrap()) {
            (_, Operation::Addition) => vec![beginning.to_string(), current_subject.to_string(), " - ".to_string(), other_var].join(""),
            (_, Operation::Multiplication) => vec![beginning.to_string(), current_subject.to_string(), " / ".to_string(), other_var].join(""),
            (true, Operation::Subtraction) => vec![beginning.to_string(), current_subject.to_string(), " + ".to_string(), other_var].join(""),
            (false, Operation::Subtraction) => vec![beginning.to_string(), other_var, " - ".to_string(), current_subject.to_string()].join(""),
            (true, Operation::Division) => vec![beginning.to_string(), current_subject.to_string(), " * ".to_string(), other_var].join(""),
            (false, Operation::Division) => vec![beginning.to_string(), other_var, " / ".to_string(), current_subject.to_string()].join(""),
            (_, Operation::Equals) => panic!("Equals operations aren't rearranged"),
    };
}
