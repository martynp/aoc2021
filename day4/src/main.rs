use array_tool::vec::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    day4("input.txt");
}

#[derive(Debug)]
struct Pair {
    a: Vec<u32>,
    b: Vec<u32>,
}

fn day4(filename: &str) {
    let file = File::open(filename).unwrap();

    let mut pairs: Vec<Pair> = Vec::new();

    // Ingest the data
    for line in BufReader::new(file).lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            continue;
        }

        let halfs: Vec<&str> = line.split(",").collect();

        let range = |string: &str| -> Vec<u32> {
            let parts: Vec<&str> = string.split("-").collect();
            let r = parts[0].parse::<u32>().unwrap()..parts[1].parse::<u32>().unwrap() + 1;
            return r.map(|x| x).collect::<Vec<u32>>();
        };

        pairs.push({
            Pair {
                a: range(halfs[0]),
                b: range(halfs[1]),
            }
        })
    }

    let mut completely_overlapping = 0;
    let mut overlapping = 0;
    for pair in pairs.iter() {
        let i = intersect(&pair.a, &pair.b);
        if i.len() == pair.a.len() {
            // A is in B
            completely_overlapping = completely_overlapping + 1;
        } else if i.len() == pair.b.len() {
            // B is in A
            completely_overlapping = completely_overlapping + 1;
        }

        if i.len() > 0 {
            overlapping = overlapping + 1;

        }
    }

    println!("There are {} completely overlapping pairs", &completely_overlapping);
    println!("There are {} overlapping pairs", &overlapping);
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

#[cfg(test)]
mod tests {
    use crate::day4;

    #[test]
    fn example() {
        day4("input_example.txt");
    }
}
