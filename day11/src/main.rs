use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    day11("input.txt");
}

#[derive(Debug)]
enum Var {
    Old,
    Value(i128),
}

#[derive(Debug)]
enum Operation {
    Add(Var, Var),
    Mult(Var, Var),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i128>,
    operation: Operation,
    test: i128,
    test_true: usize,
    test_false: usize,
    items_inspected: i128,
}

fn day11(filename: &str) {
    let file = File::open(filename).unwrap();

    let mut monkeys: Vec<Monkey> = Vec::new();

    // Ingest the data
    let mut line_count = 0;
    let mut lines: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            continue;
        }
        lines.push(line);
        line_count += 1;

        if line_count == 6 {
            let starting = lines[1].clone();
            let list = starting.split("Starting items: ").collect::<Vec<&str>>()[1];
            let items: Vec<i128> = list
                .split(", ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|v| v.to_string().parse::<i128>().unwrap())
                .collect();

            let equation = lines[2].split("Operation: new = ").collect::<Vec<&str>>()[1];
            let equation_parts: Vec<&str> = equation.split(" ").collect();

            let first = match equation_parts[0] {
                "old" => Var::Old,
                _ => panic!("Unknown first value"),
            };

            let second = match equation_parts[2] {
                "old" => Var::Old,
                n => Var::Value(n.to_string().parse::<i128>().unwrap()),
            };

            let operation = match equation_parts[1] {
                "*" => Operation::Mult(first, second),
                "+" => Operation::Add(first, second),
                _ => panic!("Unknown operation"),
            };

            let test = lines[3].split("divisible by ").collect::<Vec<&str>>()[1]
                .parse::<i128>()
                .unwrap();

            let if_true = lines[4]
                .split(" ")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();
            let if_false = lines[5]
                .split(" ")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();

            monkeys.push(Monkey {
                items: items,
                operation: operation,
                test: test,
                test_true: if_true,
                test_false: if_false,
                items_inspected: 0,
            });

            line_count = 0;
            lines.clear();
        }
    }

    let divisor: i128 = monkeys
        .iter()
        .map(|m| m.test)
        .collect::<Vec<i128>>()
        .iter()
        .product();

    println!("Divisor: {}", divisor);

    for _ in 0..10000 {
        for m_index in 0..monkeys.len() {
            for i in 0..monkeys[m_index].items.len() {
                let old = monkeys[m_index].items[i];
                let new = match &monkeys[m_index].operation {
                    Operation::Add(a, b) => (get_val(&a, old) + get_val(&b, old)) % divisor,
                    Operation::Mult(a, b) => (get_val(&a, old) * get_val(&b, old)) % divisor,
                };
                //let new = (new as f32 / 3.0).floor() as i128;
                let remainder = new % monkeys[m_index].test;

                let m_true = monkeys[m_index].test_true;
                let m_false = monkeys[m_index].test_false;
                if remainder == 0 {
                    monkeys[m_true].items.push(new);
                } else {
                    monkeys[m_false].items.push(new);
                }
            }
            // This monkey is now clear
            monkeys[m_index].items_inspected += monkeys[m_index].items.len() as i128;
            monkeys[m_index].items.clear();
        }
    }

    let mut inspection_count = monkeys
        .iter()
        .map(|i| i.items_inspected)
        .collect::<Vec<i128>>();
    inspection_count.sort();
    println!(
        "Answer is {}",
        inspection_count[inspection_count.len() - 1] * inspection_count[inspection_count.len() - 2]
    )
}

fn get_val(v: &Var, old: i128) -> i128 {
    match v {
        Var::Old => old,
        Var::Value(v) => *v,
    }
}

#[cfg(test)]
mod tests {
    use crate::day11;

    #[test]
    fn example() {
        day11("input_example.txt");
    }
}
