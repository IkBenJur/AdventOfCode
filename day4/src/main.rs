use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Clone)]
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

    fn calculate_winning_numbers(&self) -> i32 {
        let mut winning_numbers = 0;

        for winning_num in &self.winning_numbers {
            if self.own_numbers.iter().any(|number| number == winning_num) {
                winning_numbers += 1
            }
        }

        return winning_numbers;
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

fn copies_of_scratchcards(cards: &Vec<Scratchcard>) -> i32 {
    let mut number_of_copies: HashMap<i32, Vec<Scratchcard>> = HashMap::new();

    for card in cards {
        number_of_copies.insert(card.card_number, vec![card.clone()]);
    }

    let mut current_card_number = 1;
    loop {
        if let Some(card_copies) = number_of_copies.get(&current_card_number).cloned() {
            for card in card_copies {
                let amount_of_winning_numbers = card.calculate_winning_numbers();

                let card_number_to_increment_to = card.card_number + amount_of_winning_numbers + 1;

                for card_number in card.card_number + 1..card_number_to_increment_to {
                    // println!("{}", card_number);
                    let copies_of_card_number = number_of_copies
                        .entry(card_number)
                        .or_insert(vec![cards[(card_number - 1) as usize].clone()]);
                    copies_of_card_number.push(cards[(card_number - 1) as usize].clone());
                }
            }
        } else {
            break;
        }
        current_card_number += 1;
    }

    return number_of_copies.values().map(|v| v.len() as i32).sum();
}

fn main() {
    let scratchcards = load_file_into_vector("./src/input1.txt");

    let mut sum = 0;
    let amount_of_scratchcards = copies_of_scratchcards(&scratchcards);

    for card in scratchcards {
        if let Some(score) = card.calculate_score() {
            sum += score;
        }
    }

    println!("Solution 1: {}", sum);

    //HashMap to keep track of cards and copies <CardNum, CopyNumbers>
    //CardNum om entry aan te maken als nog niet bestaat anders toevoegen aan copynum
    //While score not none

    println!("Solution 2: {}", amount_of_scratchcards);
}
