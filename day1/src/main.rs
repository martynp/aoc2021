use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Inventory {
    items: Vec<u32>,
    total_calories: u32,
}

#[derive(Debug)]
struct Elf {
    inventory: Inventory,
}

fn main() {
    let file = File::open("./input_data.txt").unwrap();

    let mut elves: Vec<Elf> = Vec::new();

    let mut current_elf = Elf {
        inventory: Inventory {
            items: Vec::new(),
            total_calories: 0,
        },
    };

    // Ingest the data
    for line in BufReader::new(file).lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = Elf {
                inventory: Inventory {
                    items: Vec::new(),
                    total_calories: 0,
                },
            };
            continue;
        }
        current_elf
            .inventory
            .items
            .push(line.parse::<u32>().unwrap());
    }

    // Store the last elf
    elves.push(current_elf);

    for elf in elves.iter_mut() {
        elf.inventory.total_calories = elf.inventory.items.iter().sum();
    }

    let max = find_max_calories(&elves);
    println!("Elf {} has the most calories ({})", max.0, max.1);

    elves.sort_by(|a, b| {
        if a.inventory.total_calories < b.inventory.total_calories {
            return std::cmp::Ordering::Greater;
        } else {
            return  std::cmp::Ordering::Less;
        }
    });

    let top_three = &elves[0..3];
    let total_top_three : u32 = top_three.iter().map(|e| e.inventory.total_calories).sum();

    println!("Total top three: {}", total_top_three);

}

fn find_max_calories(elves: &Vec<Elf>) -> (usize, u32) {
    let mut max_calories: (usize, u32) = (0, elves[0].inventory.total_calories);

    for (index, elf) in elves.iter().enumerate() {
        if elf.inventory.total_calories > max_calories.1 {
            max_calories = (index, elf.inventory.total_calories);
        }
    }

    max_calories
}
