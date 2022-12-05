use array_tool::vec::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut stacks: Vec<Vec<char>> = Vec::new();

    stacks.resize(9, vec![]);
    stacks[0] = vec!['B', 'L', 'D', 'T', 'W', 'C', 'F', 'M'];
    stacks[1] = vec!['N', 'B', 'L'];
    stacks[2] = vec!['J', 'C', 'H', 'T', 'L', 'V'];
    stacks[3] = vec!['S', 'P', 'J', 'W'];
    stacks[4] = vec!['Z', 'S', 'C', 'F', 'T', 'L', 'R'];
    stacks[5] = vec!['W', 'D', 'G', 'B', 'H', 'N', 'Z'];
    stacks[6] = vec!['F', 'M', 'S', 'P', 'V', 'G', 'C', 'N'];
    stacks[7] = vec!['W', 'Q', 'R', 'J', 'F', 'V', 'C', 'Z'];
    stacks[8] = vec!['R', 'P', 'M', 'L', 'H'];

    day5("input.txt", &mut stacks);

    stacks.resize(9, vec![]);
    stacks[0] = vec!['B', 'L', 'D', 'T', 'W', 'C', 'F', 'M'];
    stacks[1] = vec!['N', 'B', 'L'];
    stacks[2] = vec!['J', 'C', 'H', 'T', 'L', 'V'];
    stacks[3] = vec!['S', 'P', 'J', 'W'];
    stacks[4] = vec!['Z', 'S', 'C', 'F', 'T', 'L', 'R'];
    stacks[5] = vec!['W', 'D', 'G', 'B', 'H', 'N', 'Z'];
    stacks[6] = vec!['F', 'M', 'S', 'P', 'V', 'G', 'C', 'N'];
    stacks[7] = vec!['W', 'Q', 'R', 'J', 'F', 'V', 'C', 'Z'];
    stacks[8] = vec!['R', 'P', 'M', 'L', 'H'];

    day5_2("input.txt", &mut stacks);
}

#[derive(Debug)]
struct Move {
    start: usize,
    count: usize,
    end: usize,
}

fn day5(filename: &str, stacks: &mut Vec<Vec<char>>) {
    let file = File::open(filename).unwrap();

    let mut diagram_lines: Vec<char> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();

    // Ingest the data
    let mut diagram = true;
    for line in BufReader::new(file).lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            if diagram == true {
                diagram = false;
            } else {
                panic!("Unexpected input");
            }
            continue;
        }

        if diagram {
        } else {
            // move 1 from 2 to 1
            let parts: Vec<&str> = line.split(" ").collect();
            moves.push(Move {
                start: parts[3].parse::<usize>().unwrap(),
                count: parts[1].parse::<usize>().unwrap(),
                end: parts[5].parse::<usize>().unwrap(),
            })
        }
    }

    for m in &moves {
        for _ in 0..m.count {
            let v = stacks[m.start - 1].pop().unwrap();
            stacks[m.end - 1].push(v);
        }
    }

    let top: Vec<char> = stacks.iter().map(|s| s.last().unwrap().clone()).collect();
    println!("{:?}", top);
}

fn day5_2(filename: &str, stacks: &mut Vec<Vec<char>>) {
    let file = File::open(filename).unwrap();

    let mut moves: Vec<Move> = Vec::new();

    // Ingest the data
    let mut diagram = true;
    for line in BufReader::new(file).lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            if diagram == true {
                diagram = false;
            } else {
                panic!("Unexpected input");
            }
            continue;
        }

        if diagram {
        } else {
            // move 1 from 2 to 1
            let parts: Vec<&str> = line.split(" ").collect();
            moves.push(Move {
                start: parts[3].parse::<usize>().unwrap(),
                count: parts[1].parse::<usize>().unwrap(),
                end: parts[5].parse::<usize>().unwrap(),
            })
        }
    }

    for m in &moves {
        let mut c: Vec<char> = Vec::new();
        for _ in 0..m.count {
            c.push(stacks[m.start - 1].pop().unwrap());
        }
        c.reverse();
        for v in c {
            stacks[m.end - 1].push(v);
        }
    }

    let top: Vec<char> = stacks.iter().map(|s| s.last().unwrap().clone()).collect();
    println!("{:?}", top);
}

#[cfg(test)]
mod tests {
    use crate::{day5, day5_2};

    #[test]
    fn example() {
        let mut stacks: Vec<Vec<char>> = Vec::new();

        stacks.resize(3, vec![]);
        stacks[0] = vec!['Z', 'N'];
        stacks[1] = vec!['M', 'C', 'D'];
        stacks[2] = vec!['P'];

        day5("input_example.txt", &mut stacks);

        let mut stacks: Vec<Vec<char>> = Vec::new();

        stacks.resize(3, vec![]);
        stacks[0] = vec!['Z', 'N'];
        stacks[1] = vec!['M', 'C', 'D'];
        stacks[2] = vec!['P'];

        day5_2("input_example.txt", &mut stacks);
    }
}
