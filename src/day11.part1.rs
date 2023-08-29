use std::cell::RefCell;

use regex::Regex;

enum Operation {
    Multiply(i32),
    Add(i32),
    SelfMultiply,
}

struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    test: (i32, usize, usize),
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
                .map(|item| item.trim().parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            let (operator, operand) = captures[2].split_once(' ').unwrap();

            let operation = if let Ok(value) = operand.parse::<i32>() {
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

    for _ in 0..20 {
        for mut monkey in monkeys.iter().map(|m| m.borrow_mut()) {
            monkey.items_inspected += monkey.items.len();

            for item in monkey.items.iter() {
                let mut new_item = match monkey.operation {
                    Operation::Add(value) => item + value,
                    Operation::Multiply(value) => item * value,
                    Operation::SelfMultiply => item * item,
                };
                new_item = new_item / 3;

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
}
