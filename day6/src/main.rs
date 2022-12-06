use array_tool::vec::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    day6("input.txt");
}

fn day6(filename: &str) {
    let file = File::open(filename).unwrap();

    let mut chars: Vec<char> = Vec::new();

    // Ingest the data
    let mut diagram = true;
    for line in BufReader::new(file).lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            continue;
        }
        chars = line.chars().collect();
    }

    for start in 0..chars.len() - 4 {
        let test = chars[start..start + 4].to_vec();
        if test.unique().len() == 4 {
            println!("Start is {}", start + 4);
            break;
        }
    }

    for start in 0..chars.len() - 14 {
        let test = chars[start..start + 14].to_vec();
        if test.unique().len() == 14 {
            println!("Start of message is {}", start + 14);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day6;

    #[test]
    fn example() {
        day6("input_example.txt");
    }
}
