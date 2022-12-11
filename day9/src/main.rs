use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    day9("input.txt");
}

const SIZE: usize = 1000;

#[derive(Debug, Clone)]
pub enum Contains {
    Nothing,
    Head,
    Tail,
}

#[derive(Debug, Clone)]
pub struct Point {
    contains: Contains,
    visited: bool,
}

enum Directions {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    direction: Directions,
    value: usize,
}

struct Knot {
    x: usize,
    y: usize,
}

fn day9(filename: &str) {
    let file = File::open(filename).unwrap();

    let mut grid_raw = vec![
        Point {
            contains: Contains::Nothing,
            visited: false,
        };
        SIZE * SIZE
    ];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(SIZE).collect();
    let grid = grid_base.as_mut_slice();

    let mut moves: Vec<Move> = Vec::new();

    // Ingest the data
    for line in BufReader::new(file).lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(" ").collect();

        moves.push(Move {
            direction: match parts[0] {
                "U" => Directions::Up,
                "D" => Directions::Down,
                "L" => Directions::Left,
                "R" => Directions::Right,
                _ => panic!("Unknown direction"),
            },
            value: parts[1].to_string().parse::<usize>().unwrap(),
        });
    }

    // Start in the middle
    let mut head_x = SIZE / 2;
    let mut head_y = SIZE / 2;

    let mut knots: Vec<Knot> = Vec::new();
    for _ in 0..9 {
        knots.push(Knot {
            x: SIZE / 2,
            y: SIZE / 2,
        });
    }

    grid[head_x][head_x].visited = true;

    for m in moves {
        let direction = m.direction;
        for _ in 0..m.value {
            match direction {
                Directions::Up => {
                    head_y = head_y + 1;
                }
                Directions::Down => {
                    head_y = head_y - 1;
                }
                Directions::Left => {
                    head_x = head_x - 1;
                }
                Directions::Right => {
                    head_x = head_x + 1;
                }
            }
            let mut x = head_x;
            let mut y = head_y;

            for knot in knots.iter_mut() {
                if knots_touching(x, y, knot) == false {
                    *knot = move_knot(x, y, knot, grid);
                }
                x = knot.x;
                y = knot.y;
            }
            grid[knots.last().unwrap().x][knots.last().unwrap().y].visited = true;
        }
    }

    let mut count = 0;
    for y in 0..SIZE {
        for x in 0..SIZE {
            if grid[x][y].visited {
                count += 1;
            } else {
            }
        }
    }

    println!("Count is {}", count);
}

fn display(grid: &mut [&mut [Point]], x: usize, y: usize, len: usize) {
    for y in y..y + len {
        for x in x..x + len {
            if grid[x][y].visited {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn knots_touching(head_x: usize, head_y: usize, knot: &Knot) -> bool {
    let x = knot.x;
    let y = knot.y;
    if x < head_x - 1 || x > head_x + 1 {
        return false;
    }

    if y < head_y - 1 || y > head_y + 1 {
        return false;
    }

    return true;
}

fn move_knot(head_x: usize, head_y: usize, knot: &Knot, grid: &mut [&mut [Point]]) -> Knot {
    let mut x: usize = knot.x;
    let mut y: usize = knot.y;

    if x < head_x {
        x = x + 1;
    } else if x > head_x {
        x = x - 1;
    }

    if y < head_y {
        y = y + 1;
    } else if y > head_y {
        y = y - 1;
    }

    return Knot { x: x, y: y };
}

#[cfg(test)]
mod tests {
    use crate::day9;

    #[test]
    fn example() {
        day9("input_example.txt");
    }
    #[test]
    fn example2() {
        day9("input_example2.txt");
    }
}
