use std::fs::File;
use std::io::{BufRead, BufReader};

enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    pub fn from_letter(letter: &str) -> Self {
        match letter {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => {
                panic!("Unknown")
            }
        }
    }
}

enum Strategy {
    Rock,
    Paper,
    Scissors,
}

impl Strategy {
    pub fn from_letter(letter: &str) -> Self {
        match letter {
            "X" => Strategy::Rock,
            "Y" => Strategy::Paper,
            "Z" => Strategy::Scissors,
            _ => {
                panic!("Unknown")
            }
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    pub fn from_letter(letter: &str) -> Self {
        match letter {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => {
                panic!("Unknown")
            }
        }
    }
}

fn main() {
    let file = File::open("./input_data.txt").unwrap();
    let mut rounds: Vec<(Play, Strategy, Outcome)> = Vec::new();

    // Ingest the data
    for line in BufReader::new(file).lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            continue;
        }
        let round = line.split_once(" ");
        let p = Play::from_letter(round.unwrap().0);
        let s = Strategy::from_letter(round.unwrap().1);
        let o: Outcome = Outcome::from_letter(round.unwrap().1);

        rounds.push((p, s, o));
    }
    let win = 6;
    let draw = 3;
    let paper = 2;
    let rock = 1;
    let scissors = 3;

    let mut scores: Vec<(i32, i32)> = Vec::new();

    for (play, strategy, _) in &rounds {
        let score = match play {
            Play::Rock => match strategy {
                Strategy::Rock => (rock + draw, rock + draw),
                Strategy::Paper => (rock, paper + win),
                Strategy::Scissors => (rock + win, scissors),
            },
            Play::Paper => match strategy {
                Strategy::Rock => (paper + win, rock),
                Strategy::Paper => (draw + paper, draw + paper),
                Strategy::Scissors => (paper, scissors + win),
            },
            Play::Scissors => match strategy {
                Strategy::Rock => (scissors, rock + win),
                Strategy::Paper => (scissors + win, paper),
                Strategy::Scissors => (draw + scissors, draw + scissors),
            },
        };

        scores.push(score);
    }

    let elf_score: i32 = scores.iter().map(|s| s.0).sum();
    let my_score: i32 = scores.iter().map(|s| s.1).sum();

    println!("Elf scored {}, you scored {}", elf_score, my_score);

    let mut scores: Vec<(i32, i32)> = Vec::new();

    for (play, _, outcome) in &rounds {
        let score = match play {
            Play::Rock => match outcome {
                Outcome::Win => (rock, paper + win),
                Outcome::Lose => (rock + win, scissors),
                Outcome::Draw => (rock + draw, rock + draw),
            },
            Play::Paper => match outcome {
                Outcome::Win => (paper, scissors + win),
                Outcome::Lose => (paper + win, rock),
                Outcome::Draw => (paper + draw, paper + draw),
            },
            Play::Scissors => match outcome {
                Outcome::Win => (scissors, rock + win),
                Outcome::Lose => (scissors + win, paper),
                Outcome::Draw => (scissors + draw, scissors + draw),
            },
        };

        scores.push(score);
    }

    let elf_score: i32 = scores.iter().map(|s| s.0).sum();
    let my_score: i32 = scores.iter().map(|s| s.1).sum();

    println!("Elf scored {}, you scored {}", elf_score, my_score);
}
