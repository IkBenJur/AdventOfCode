use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug, PartialEq)]
enum CardType {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    High,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: i64,
}

impl Hand {
    fn new(cards: String, bid: i64) -> Self {
        Self { cards, bid }
    }

    fn worth_of_card(&self) -> CardType {
        let mut char_count: HashMap<char, i32> = HashMap::new();

        for ch in self.cards.chars() {
            let counter = char_count.entry(ch).or_insert(0);
            *counter += 1;
        }

        if (char_count.contains_key(&'J')) && (char_count.get(&'J').unwrap() != &5) {
            let (highest_key, max_value) = char_count.iter().filter(|&(&key, _)| key != 'J').max_by_key(|&(_, value)| value).unwrap();

            char_count.insert(*highest_key, max_value + char_count.get(&'J').unwrap_or(&0));
            char_count.insert('J', 0);
        }

        if char_count.values().any(|&count| count == 5) {
            return CardType::Five;
        }

        if char_count.values().any(|&count| count == 4) {
            return CardType::Four;
        }

        if char_count.values().any(|&count| count == 3) && char_count.values().any(|&count| count == 2){
            return CardType::Full;
        }

        if char_count.values().any(|&count| count == 3) {
            return CardType::Three;
        }

        if char_count.values().any(|&count| count == 2) {
            let amount_of_pairs = char_count.values().filter(|&&value| value == 2).count();

            if amount_of_pairs == 2 {
                return CardType::Two;
            } else {
                return CardType::One;
            }
        }

        return CardType::High;
    }
}

fn load_file_into_vec(input_file: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(input_file).expect("Failed to find file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("Line not found"))
        .collect()
}

fn load_hands_from_lines(lines: Vec<String>) -> Vec<Hand> {
    lines
        .into_iter()
        .map(|line| {
            let parts = line.split_whitespace();
            return Hand::new(
                parts.clone().nth(0).unwrap().parse().unwrap(),
                parts.clone().nth(1).unwrap().parse().unwrap(),
            );
        })
        .collect()
}

fn return_if_card_is_higher(hand_a: &Hand, hand_b: &Hand) -> Ordering {
    let worth_of_card_a = hand_a.worth_of_card();
    let worth_of_card_b = hand_b.worth_of_card();

    if worth_of_card_a == worth_of_card_b {
        let order_of_cards = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
        for i in 0..=hand_a.cards.len() {
            let char_a = hand_a.cards.chars().nth(i).unwrap();
            let char_b = hand_b.cards.chars().nth(i).unwrap();
            
            if char_a == char_b {
                continue;
            }

            let index_char_a = order_of_cards.iter().position(|&card_num| card_num == char_a).unwrap();
            let index_char_b = order_of_cards.iter().position(|&card_num| card_num == char_b).unwrap();
            
            if index_char_a < index_char_b {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
    }

    match (worth_of_card_a, worth_of_card_b) {
        (CardType::Five, _) => Ordering::Greater,
        (_, CardType::Five) => Ordering::Less,
        (CardType::Four, _) => Ordering::Greater,
        (_, CardType::Four) => Ordering::Less,
        (CardType::Full, _) => Ordering::Greater,
        (_, CardType::Full) => Ordering::Less,
        (CardType::Three, _) => Ordering::Greater,
        (_, CardType::Three) => Ordering::Less,
        (CardType::Two, _) => Ordering::Greater,
        (_, CardType::Two) => Ordering::Less,
        (CardType::One, _) => Ordering::Greater,
        (_, CardType::One) => Ordering::Less,
        (CardType::High, _) => Ordering::Greater,
        (_, _) => Ordering::Less,
    }
}

fn main() {
    let lines = load_file_into_vec("./src/input.txt");
    let mut hands = load_hands_from_lines(lines);

    hands.sort_by(|a, b| return_if_card_is_higher(a, b));
    let mut total_winnings: i64 = 0;

    for i in 0..=hands.len()-1 {
        total_winnings += hands[i].bid * (i + 1) as i64;
    }


    println!("Solution 1 {}", total_winnings);
}
