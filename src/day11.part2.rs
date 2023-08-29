use std::cell::RefCell;

use regex::Regex;

enum Operation {
    Multiply(u64),
    Add(u64),
    SelfMultiply,
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: (u64, usize, usize),
    items_inspected: usize,
}

const MONKEY_EXPRESSION: &str = r"Starting items: (.+)
\s+Operation: new = old (.+)
\s+Test: divisible by (\d+)
\s+If true: throw to monkey (\d+)
\s+If false: throw to monkey (\d+)";

fn main() {
    const CONTENT: &str = include_str!("day11.input");

    let monkey_pattern = Regex::new(MONKEY_EXPRESSION).unwrap();

    let mut monkeys = monkey_pattern
        .captures_iter(CONTENT)
        .map(|captures| {
            let items = captures[1]
                .split(",")
                .map(|item| item.trim().parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            let (operator, operand) = captures[2].split_once(' ').unwrap();

            let operation = if let Ok(value) = operand.parse::<u64>() {
                match operator {
                    "+" => Operation::Add(value),
                    "*" => Operation::Multiply(value),
                    _ => panic!("Invalid operation"),
                }
            } else {
                Operation::SelfMultiply
            };

            let test = (
                captures[3].parse().unwrap(),
                captures[4].parse().unwrap(),
                captures[5].parse().unwrap(),
            );

            // Using RefCell to avoid cloning the whole struct in the loops below
            RefCell::new(Monkey {
                items,
                operation,
                test,
                items_inspected: 0,
            })
        })
        .collect::<Vec<_>>();

    let lcm = monkeys.iter().fold(1, |partial_lcm, monkey| {
        lcm(partial_lcm, monkey.borrow().test.0)
    });

    for _ in 0..10000 {
        for mut monkey in monkeys.iter().map(|m| m.borrow_mut()) {
            monkey.items_inspected += monkey.items.len();

            for item in monkey.items.iter() {
                let mut new_item = match monkey.operation {
                    Operation::Add(value) => item + value,
                    Operation::Multiply(value) => item * value,
                    Operation::SelfMultiply => item * item,
                };
                new_item = new_item % lcm;

                let (divisor, true_monkey, false_monkey) = monkey.test;
                monkeys[if new_item % divisor == 0 {
                    true_monkey
                } else {
                    false_monkey
                }]
                .borrow_mut()
                .items
                .push(new_item);
            }

            monkey.items.clear();
        }
    }

    // The sum of the inspected item for the two monkeys with the most inspected items
    monkeys.sort_by(|a, b| b.borrow().items_inspected.cmp(&a.borrow().items_inspected));
    println!(
        "{}",
        monkeys[0].borrow().items_inspected * monkeys[1].borrow().items_inspected
    );
    println!("{}", lcm);
}

// Just for fun
fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
