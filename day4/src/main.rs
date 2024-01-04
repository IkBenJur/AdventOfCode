use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
struct Scratchcard {
    card_number: i32,
    winning_numbers: Vec<i32>,
    own_numbers: Vec<i32>,
}

impl Scratchcard {
    fn new(card_number: i32, winning_numbers: Vec<i32>, own_numbers: Vec<i32>) -> Self {
        Self {
            card_number,
            winning_numbers,
            own_numbers,
        }
    }

    fn calculate_score(&self) -> Option<i32> {
        let mut score = None;

        for winning_num in &self.winning_numbers {
            if self.own_numbers.iter().any(|number| number == winning_num) {
                if let Some(current_score) = score {
                    score = Some(current_score * 2);
                } else {
                    score = Some(1);
                }
            }
        }

        return score;
    }
}

fn load_file_into_vector(filename: impl AsRef<Path>) -> Vec<Scratchcard> {
    let file = File::open(filename).expect("Failed to find file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            let line_str = line.unwrap();
            let mut line = line_str.split(':');

            let card_num: i32 = line
                .nth(0)
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();

            let numbers_str = line.next().unwrap();
            let mut numbers = numbers_str.split('|');

            let winning_numbers: Vec<i32> = numbers
                .clone()
                .nth(0)
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            let own_numbers: Vec<i32> = numbers
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            Scratchcard::new(card_num, winning_numbers, own_numbers)
        })
        .collect()
}

fn main() {
    let scratchcards = load_file_into_vector("./src/input1.txt");

    let mut sum = 0;

    for card in scratchcards {
        if let Some(score) = card.calculate_score() {
            sum += score;
        }
    }

    println!("Solution 1: {}", sum);
}
