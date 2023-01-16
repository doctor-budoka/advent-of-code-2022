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
    num_inspections: u32,
}

impl Monkey {
    fn add_item(&mut self, item: u32) {
        self.items.push(item);
    }

    fn take_turn(&mut self) -> Vec<(usize, u32)> {
        let mut throw_instructions: Vec<(usize, u32)> = Vec::new();
        for _ in 0..self.items.len() {
            throw_instructions.push(self.inspect_first_item());
        }
        return throw_instructions;
    }

    fn inspect_first_item(&mut self) -> (usize, u32) {
        self.num_inspections += 1;
        let mut this_item = self.items.remove(0);
        this_item = self.change_worry(this_item);
        let to_monkey = self.get_monkey_to_throw_to(this_item);
        return (to_monkey, this_item);
    }

    fn change_worry(&self, value: u32) -> u32 {
        let value_a: u32 = match self.rule_val_a {
            FormulaValue::Constant(number) => number,
            FormulaValue::FormulaVar => value,
        };
        let value_b: u32 = match self.rule_val_b {
            FormulaValue::Constant(number) => number,
            FormulaValue::FormulaVar => value,
        };
        let new_value = match self.rule_op {
            '+' => value_a + value_b,
            '*' => value_a * value_b,
            c => panic!("Unexpected char received for operation: {}", c),
        };
        return new_value / 3;
    }

    fn get_monkey_to_throw_to(&self, value: u32) -> usize {
        return if value % self.throw_divisibility == 0 {self.true_monkey} else {self.false_monkey};
    }
}

struct Troupe {
    monkeys: Vec<Monkey>,
}

impl Troupe {
    fn new() -> Troupe {
        return Troupe{monkeys: Vec::new()};
    }

    fn add_monkey(&mut self, monkey: Monkey) {
        self.monkeys.push(monkey);
    }

    fn play_round(&mut self) {
        for monkey_ind in 0..self.monkeys.len() {
            self.get_monkey_to_take_turn(monkey_ind);
        }
    }

    fn get_monkey_to_take_turn(&mut self, monkey_ind: usize) {
        let throw_instructions = self.monkeys[monkey_ind].take_turn();
        for (to_monkey, value) in &throw_instructions {
            self.throw(*to_monkey, *value);
        }
    }

    fn throw(&mut self, to_monkey: usize, item: u32) {
        self.monkeys[to_monkey].add_item(item);
    }

    fn get_monkey_business(&self) -> u32 {
        let mut monkey_activity: Vec<u32> = Vec::new();
        for monkey in &self.monkeys {
            monkey_activity.push(monkey.num_inspections);
        }
        monkey_activity.sort_by(|a, b| b.cmp(a));
        return monkey_activity[0] * monkey_activity[1];
    }
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    
    let mut monkeys: Troupe = initialise_monkeys(file_name);
    for _ in 0..20 {
        monkeys.play_round();
    }
    let monkey_business = monkeys.get_monkey_business();
    println!("The level of monkey business is: {}", monkey_business);
}


fn initialise_monkeys(file_name: &String) -> Troupe {
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let mut monkeys = Troupe::new();

    for monkey_string in input.split("\n\n").collect::<Vec<&str>>() {
        let new_monkey = initialise_monkey(monkey_string);
        monkeys.add_monkey(new_monkey);
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
        num_inspections: 0,
    };
}
