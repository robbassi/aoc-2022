use aoc::io::*;

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Old,
    Const(u64),
}

#[derive(Debug, Clone)]
enum Operation {
    Add(Value, Value),
    Mul(Value, Value),
}

impl Operation {
    fn eval(&self, old: u64) -> u64 {
        let eval_value = |value: &Value| match value {
            Value::Old => old,
            Value::Const(n) => *n,
        };
        match self {
            Operation::Add(a, b) => eval_value(a) + eval_value(b),
            Operation::Mul(a, b) => eval_value(a) * eval_value(b),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    monkey_false: usize,
    monkey_true: usize,
}

fn parse_items(line: &str) -> Vec<u64> {
    let mut items = Vec::new();
    let parts: Vec<&str> = line.split(": ").collect();
    match parts[..] {
        ["  Starting items", item_list] => {
            let item_list_parts: Vec<&str> = item_list.split(", ").collect();
            for item in item_list_parts {
                items.push(item.parse::<u64>().unwrap());
            }
        }
        _ => panic!(),
    }
    items
}

fn parse_operation(line: &str) -> Operation {
    let parts: Vec<&str> = line.split(": new = ").collect();
    match parts[..] {
        ["  Operation", operation_definition] => {
            let operation_parts: Vec<&str> = operation_definition.split(" ").collect();
            let left = match operation_parts[0] {
                "old" => Value::Old,
                n => Value::Const(n.parse().unwrap()),
            };
            let right = match operation_parts[2] {
                "old" => Value::Old,
                n => Value::Const(n.parse().unwrap()),
            };
            match operation_parts[1] {
                "+" => Operation::Add(left, right),
                "*" => Operation::Mul(left, right),
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}

fn parse_divisor(line: &str) -> u64 {
    let parts: Vec<&str> = line.split(": divisible by ").collect();
    match parts[..] {
        ["  Test", n] => n.parse().unwrap(),
        _ => panic!(),
    }
}

fn parse_monkey_true(line: &str) -> usize {
    let parts: Vec<&str> = line.split(": throw to monkey ").collect();
    match parts[..] {
        ["    If true", n] => n.parse().unwrap(),
        _ => panic!(),
    }
}

fn parse_monkey_false(line: &str) -> usize {
    let parts: Vec<&str> = line.split(": throw to monkey ").collect();
    match parts[..] {
        ["    If false", n] => n.parse().unwrap(),
        _ => panic!(),
    }
}

fn parse_monkeys(input: &Vec<String>) -> Vec<Monkey> {
    let monkey_lines: Vec<&str> = input
        .iter()
        .map(|line| line.as_str())
        .filter(|line| *line != "")
        .collect();

    monkey_lines
        .chunks(6)
        .fold(Vec::new(), |mut monkeys, lines| {
            let items = parse_items(lines[1]);
            let operation = parse_operation(lines[2]);
            let divisor = parse_divisor(lines[3]);
            let monkey_true = parse_monkey_true(lines[4]);
            let monkey_false = parse_monkey_false(lines[5]);
            monkeys.push(Monkey {
                items,
                operation,
                divisor,
                monkey_false,
                monkey_true,
            });
            monkeys
        })
}

fn simulate_rounds(mut monkeys: Vec<Monkey>, n: u64, new_worry_level: impl Fn(u64) -> u64) -> u64 {
    let mut inspection_counts = vec![0; monkeys.len()];
    for _ in 0..n {
        let mut i = 0;
        while i < monkeys.len() {
            inspection_counts[i] += monkeys[i].items.len() as u64;
            while let Some(item) = monkeys[i].items.pop() {
                let worry_level = new_worry_level(monkeys[i].operation.eval(item));
                if worry_level % monkeys[i].divisor == 0 {
                    let next = monkeys[i].monkey_true;
                    monkeys[next].items.push(worry_level);
                } else {
                    let next = monkeys[i].monkey_false;
                    monkeys[next].items.push(worry_level);
                }
            }
            i += 1;
        }
    }
    inspection_counts.sort_by(|a, b| b.cmp(a));
    inspection_counts[0] * inspection_counts[1]
}

fn part1(monkeys: Vec<Monkey>) -> u64 {
    simulate_rounds(monkeys, 20, |n| n / 3)
}

fn part2(monkeys: Vec<Monkey>) -> u64 {
    let max_worry_level = monkeys.iter().fold(1, |acc, m| acc * m.divisor);
    simulate_rounds(monkeys, 10000, |n| n % max_worry_level)
}

fn main() {
    let input: Vec<String> = read_input();
    let monkeys = parse_monkeys(&input);
    println!("part 1 = {:?}", part1(monkeys.clone()));
    println!("part 2 = {:?}", part2(monkeys));
}
