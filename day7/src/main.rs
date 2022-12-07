use indextree::{Arena, NodeId};
use std::env::current_exe;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::current;

fn main() {
    day7("input.txt");
}

#[derive(Debug, PartialEq)]
enum NodeType {
    File,
    Dir,
}

fn day7(filename: &str) {
    let file = File::open(filename).unwrap();

    let mut terminal: Vec<String> = Vec::new();

    // Ingest the data
    let mut diagram = true;
    for line in BufReader::new(file).lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            continue;
        }
        terminal.push(line);
    }

    // Create a new arena
    let arena: &mut Arena<(String, NodeType, i32, bool)> = &mut Arena::new();
    let root = arena.new_node(("/".to_string(), NodeType::Dir, 0, false));

    // process commands
    let mut current_node = root;
    let mut parent_nodes: Vec<indextree::NodeId> = Vec::new();

    for line in terminal {
        let parts: Vec<&str> = line.split(" ").collect();
        // Get a command
        if line.starts_with("$") {
            match parts[1] {
                "cd" => match parts[2] {
                    "/" => {
                        current_node = root;
                        parent_nodes = Vec::new();
                    }
                    ".." => {
                        current_node = parent_nodes.pop().unwrap();
                    }
                    _ => {
                        let new_node =
                            arena.new_node((parts[2].to_string(), NodeType::Dir, 0, false));
                        current_node.append(new_node, arena);
                        parent_nodes.push(current_node);
                        current_node = new_node;
                    }
                },
                "ls" => {}
                _ => panic!("Unknown command"),
            }
        } else {
            match parts[0] {
                "dir" => {}
                _ => {
                    let size = parts[0].parse::<i32>().unwrap();
                    let name = parts[1];

                    let new_node = arena.new_node((name.to_string(), NodeType::File, size, false));
                    current_node.append(new_node, arena);
                    arena.get_mut(current_node).unwrap().get_mut().2 += size;
                }
            }
        }
    }

    'outer: loop {
        let mut set: Option<(NodeId, i32)> = None;
        'inner: for element in arena.iter() {
            let id = arena.get_node_id(element).unwrap();

            // Dont do anything which is already done, or which is a file - they are good
            if arena.get(id).unwrap().get().3 == true {
                continue 'inner;
            }
            if arena.get(id).unwrap().get().1 == NodeType::File {
                continue 'inner;
            }

            // Skip if any of the dirs contained are not processed
            for c in id.descendants(arena).skip(1) {
                if arena.get(c).unwrap().get().1 == NodeType::Dir
                    && arena.get(c).unwrap().get().3 == false
                {
                    continue 'inner;
                }
            }

            let child_dir_size: i32 = id
                .descendants(arena)
                .into_iter()
                .filter(|&c| arena.get(c).unwrap().get().1 == NodeType::File)
                .map(|c| arena.get(c).unwrap().get().2)
                .sum();
            set = Some((id, child_dir_size));
            break 'inner;
        }

        match set {
            Some((id, size)) => {
                arena.get_mut(id).unwrap().get_mut().2 = size;
                arena.get_mut(id).unwrap().get_mut().3 = true
            }
            None => {
                break 'outer;
            }
        }
    }

    let sum: i32 = arena
        .iter()
        .skip(1)
        .map(|v| {
            if v.get().1 == NodeType::Dir && v.get().2 <= 100000 {
                return v.get().2;
            } else {
                return 0;
            }
        })
        .sum();

    println!("Answer: {}", sum);

    let total = arena.get(root).unwrap().get().2;
    let remaining = 70000000 - total;
    let to_find = 30000000 - remaining;

    println!(
        "Total: {}\t\tRemaining: {}\t\tTo Find: {}",
        total, remaining, to_find
    );

    let mut sizes_over_to_find: Vec<i32> = arena
        .iter()
        .filter(|v| v.get().1 == NodeType::Dir)
        .filter(|v| v.get().2 > to_find)
        .map(|v| v.get().2)
        .collect();

    sizes_over_to_find.sort();
    println!("{}", sizes_over_to_find[0]);

    //dbg!(root.debug_pretty_print(arena));
}

#[cfg(test)]
mod tests {
    use crate::day7;

    #[test]
    fn example() {
        day7("input_example.txt");
    }
}
