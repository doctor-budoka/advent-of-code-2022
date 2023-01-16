use std::env;
use std::fs;

#[derive(Debug)]
enum FormulaValue {
    FormulaVar,
    Constant(u32),
}

struct Monkey {
    items: Vec<u32>,
    rule_val_a: FormulaValue,
    rule_val_b: FormulaValue,
    rule_op: char, 
    throw_divisibility: u32,
    true_monkey: usize,
    false_monkey: usize,
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    
    let monkeys: Vec<Monkey> = initialise_monkeys(file_name);

}


fn initialise_monkeys(file_name: &String) -> Vec<Monkey> {
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let mut monkeys = Vec::new();

    for monkey_string in input.split("\n\n").collect::<Vec<&str>>() {
        let new_monkey = initialise_monkey(monkey_string);
        monkeys.push(new_monkey);
    }
    return monkeys;
}


fn initialise_monkey(input: &str) -> Monkey {
    let mut items: Option<Vec<u32>> = None;
    let items_start = "Starting items: ";

    let mut rule_val_a: Option<FormulaValue> = None;
    let mut rule_val_b: Option<FormulaValue> = None;
    let mut rule_operation: Option<char> = None;
    let operation_start = "Operation: new = ";

    let mut divisible_by: Option<u32> = None;
    let divisible_start = "Test: divisible by ";
    let mut true_monkey: Option<usize> = None;
    let true_start = "If true: throw to monkey ";
    let mut false_monkey: Option<usize> = None;
    let false_start = "If false: throw to monkey ";

    for line in input.lines() {
        if line.starts_with("Monkey") {}
        else if line.trim().starts_with(items_start) {
            items = Some(line.trim().replace(items_start, "").split(", ").map(|x| x.parse::<u32>().unwrap()).collect());
        }
        else if line.trim().starts_with(operation_start){
            let expr: String = line.trim().replace(operation_start, "");
            let expr_vec: Vec<&str> = expr.split(" ").collect();
            rule_val_a = match expr_vec[0].parse::<u32>() {
                Ok(number) => Some(FormulaValue::Constant(number)),
                Err(_) => Some(FormulaValue::FormulaVar),
            };
            rule_val_b = match expr_vec[2].parse::<u32>() {
                Ok(number) => Some(FormulaValue::Constant(number)),
                Err(_) => Some(FormulaValue::FormulaVar),
            };
            rule_operation = Some(expr_vec[1].chars().nth(0).unwrap());

        }
        else if line.trim().starts_with(divisible_start) {
            divisible_by = Some(line.trim().replace(divisible_start, "").parse().unwrap());
        }
        else if line.trim().starts_with(true_start) {
            true_monkey = Some(line.trim().replace(true_start, "").parse().unwrap());
        }
        else if line.trim().starts_with(false_start) {
            false_monkey = Some(line.trim().replace(false_start, "").parse().unwrap());
        }
        else {
            panic!("'{}' was unexpected!", line.trim());
        }
    }

    return Monkey {
        items: items.expect("items should be initialised"), 
        rule_val_a: rule_val_a.expect("rule_val_a should be initialised"), 
        rule_val_b: rule_val_b.expect("rule_val_b should be initialised"),
        rule_op: rule_operation.expect("rule_op should be initialised"),
        throw_divisibility: divisible_by.expect("throw_divisibility should be initialised"),
        true_monkey: true_monkey.expect("true_monkey should be initialised"),
        false_monkey: false_monkey.expect("false_monkey should be initialised"),
    };
}
