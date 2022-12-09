use array_tool::vec::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    day8("input.txt", 99);
}

#[derive(Clone, Debug)]
struct Tree {
    pub height: i32,
    pub visible_top: bool,
    pub visible_right: bool,
    pub visible_bottom: bool,
    pub visible_left: bool,
    pub scenic: i32,
}

fn day8(filename: &str, size: usize) {
    let file = File::open(filename).unwrap();

    let mut grid_raw = vec![
        Tree {
            height: 0,
            visible_top: false,
            visible_right: false,
            visible_bottom: false,
            visible_left: false,
            scenic: 0,
        };
        size * size
    ];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(size).collect();
    let grid = grid_base.as_mut_slice();

    // Ingest the data
    let mut x = 0;
    let mut y = 0;
    for line in BufReader::new(file).lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            continue;
        }
        for t in line.chars().map(|c| c.to_string().parse::<i32>().unwrap()) {
            grid[x][y].height = t;
            x = x + 1;
        }
        x = 0;
        y = y + 1;
    }

    for a in 0..size {
        let mut tallest_top = -1;
        let mut tallest_left = -1;
        let mut tallest_right = -1;
        let mut tallest_bottom = -1;
        for b in 0..size {
            let b_rev = size - 1 - b;
            // b goes 0 to 4

            // x is fixed, b changes up
            if grid[a][b].height > tallest_top {
                grid[a][b].visible_top = true;
                tallest_top = grid[a][b].height;
            }

            // x is fixed, b_rev goes down
            if grid[a][b_rev].height > tallest_bottom {
                grid[a][b_rev].visible_bottom = true;
                tallest_bottom = grid[a][b_rev].height;
            }

            // y is fixed, b changes up
            if grid[b][a].height > tallest_left {
                grid[b][a].visible_left = true;
                tallest_left = grid[b][a].height;
            }

            // y is fixed, b_rev goes down
            if grid[b_rev][a].height > tallest_right {
                grid[b_rev][a].visible_right = true;
                tallest_right = grid[b_rev][a].height;
            }
        }
    }

    let mut count = 0;

    for y in 0..size {
        for x in 0..size {
            if grid[x][y].visible_top
                || grid[x][y].visible_right
                || grid[x][y].visible_bottom
                || grid[x][y].visible_left
            {
                print!("v");
                count += 1;
            } else {
                print!(".");
            }
        }
        print!("\n");
    }

    println!("Count is {}", count);

    let vec_left = |x: usize, y: usize, grid: &mut [&mut [Tree]]| -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        let mut xx: i32 = x as i32 - 1;
        while xx >= 0 {
            ret.push(grid[xx as usize][y].height);
            xx -= 1;
        }
        return ret;
    };
    let vec_right = |x: usize, y: usize, size: usize, grid: &mut [&mut [Tree]]| -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        let mut xx: i32 = x as i32 + 1;
        while xx < size as i32 {
            ret.push(grid[xx as usize][y].height);
            xx += 1;
        }
        return ret;
    };

    let vec_top = |x: usize, y: usize, grid: &mut [&mut [Tree]]| -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        let mut yy: i32 = y as i32 - 1;
        while yy >= 0 {
            ret.push(grid[x][yy as usize].height);
            yy -= 1;
        }
        return ret;
    };
    let vec_bottom = |x: usize, y: usize, size: usize, grid: &mut [&mut [Tree]]| -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        let mut yy: i32 = y as i32 + 1;
        while yy < size as i32 {
            ret.push(grid[x][yy as usize].height);
            yy += 1;
        }
        return ret;
    };

    let get_visible_count = |height: i32, list: &Vec<i32>| -> i32 {
        if list.len() == 0 {
            return 0;
        }

        if list.len() == 1 {
            return 1;
        }

        let mut count = 0;
        for index in 0..list.len() {
            if list[index] >= height {
                count += 1;
                break;
            } else {
                count += 1;
            }
        }

        return count;
    };

    let mut scenic : Vec<i32> = Vec::new();

    for x in 0..size {
        for y in 0..size {
            let cur = grid[x][y].height;
            let mut top_dist: i32 = 0;
            let mut bottom_dist: i32 = 0;
            let mut left_dist: i32 = 0;
            let mut right_dist: i32 = 0;

            let left_vec = vec_left(x, y, grid);
            let right_vec = vec_right(x, y, size, grid);
            let top_vec = vec_top(x, y, grid);
            let bottom_vec = vec_bottom(x, y, size, grid);


            left_dist = get_visible_count(cur, &left_vec);
            right_dist = get_visible_count(cur, &right_vec);
            top_dist = get_visible_count(cur, &top_vec);
            bottom_dist = get_visible_count(cur, &bottom_vec);
            

            grid[x][y].scenic = (top_dist * right_dist * bottom_dist * left_dist) as i32;
            scenic.push(grid[x][y].scenic);
            println!(
                "x: {}, y: {}, scenic: {}, t: {}, l: {}, b: {}, r: {}",
                x, y, grid[x][y].scenic, top_dist, left_dist, bottom_dist, right_dist
            );
        }


    }

    println!("{}", scenic.iter().max().unwrap());
}

#[cfg(test)]
mod tests {
    use crate::day8;

    #[test]
    fn example() {
        day8("input_example.txt", 5);
    }
}
