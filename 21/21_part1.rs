use std::env;
use std::fs;

mod algebra;
use algebra::SymbolTable;


fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("File name is '{}'. Reading input...", file_name);

    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let mut symbol_table: SymbolTable = SymbolTable::new();
    for line in input.trim().lines().collect::<Vec<&str>>() {
        println!("{}", &line);
        symbol_table.add_symbol_from_string(&line.trim().to_string());
    }
    println!("{:?}", &symbol_table);
} 
