use array_tool::vec::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    day3("input.txt");
}

#[derive(Debug)]
struct Rucksack {
    all_items: Vec<char>,
    compartment_one: Vec<char>,
    compartment_two: Vec<char>,
    intersection: Vec<char>,
    priority: u32,
}

fn day3(filename: &str) {
    let file = File::open(filename).unwrap();

    let mut rucksacks: Vec<Rucksack> = Vec::new();

    // Ingest the data
    for line in BufReader::new(file).lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            continue;
        }
        let halfs = line.split_at(line.len() / 2);

        let r = Rucksack {
            all_items: line.chars().collect(),
            compartment_one: halfs.0.chars().collect(),
            compartment_two: halfs.1.chars().collect(),
            intersection: Vec::new(),
            priority: 0,
        };

        rucksacks.push(r);
    }

    for rucksack in rucksacks.iter_mut() {
        let intersect = intersect(&rucksack.compartment_one, &rucksack.compartment_two);
        assert_eq!(intersect.len(), 1);
        rucksack.priority = get_priority(intersect[0]);
        rucksack.intersection = intersect;
    }

    println!(
        "Priority sum: {}",
        rucksacks.iter().map(|r| r.priority as i32).sum::<i32>()
    );

    assert_eq!(rucksacks.len() % 3, 0);
    let mut badges: Vec<u32> = Vec::new();

    for i in 0..rucksacks.len() / 3 {
        let first_intersect: Vec<char> = intersect(
            &rucksacks[i * 3 + 0].all_items,
            &rucksacks[i * 3 + 1].all_items,
        );
        let intersect: Vec<char> = intersect(&first_intersect, &rucksacks[i * 3 + 2].all_items);
        assert_eq!(intersect.len(), 1);
        badges.push(get_priority(intersect[0]));
    }

    println!("Grouped badge priority: {}", badges.iter().sum::<u32>())
}

fn intersect<T: PartialEq + Clone>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let mut intersection = Vec::new();

    for item in a {
        for other_item in b {
            if item == other_item {
                intersection.push(other_item.clone());
                break;
            }
        }
    }

    return intersection.unique();
}

fn get_priority(item: char) -> u32 {
    if item >= 65 as char && item <= 90 as char {
        return item as u32 - 65 + 27;
    }

    if item >= 97 as char && item <= 122 as char {
        return item as u32 - 97 + 1;
    }

    panic!("Unknown");
}

#[cfg(test)]
mod tests {
    use crate::day3;

    #[test]
    fn example() {
        day3("input_example.txt");
    }
}
