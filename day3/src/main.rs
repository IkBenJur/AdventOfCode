use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
struct PartNumber {
    num: i64,
    points: HashSet<(i64, i64)>,
}

impl PartNumber {
    fn new(char: char, row: i64, col: i64) -> PartNumber {
        Self {
            num: char.to_digit(10).unwrap() as i64,
            points: HashSet::from([
                (row - 1, col - 1),
                (row - 1, col),
                (row - 1, col + 1),
                (row, col - 1),
                (row, col + 1),
                (row + 1, col - 1),
                (row + 1, col),
                (row + 1, col + 1),
            ]),
        }
    }

    fn add_digit(&mut self, char: char, row: i64, col: i64) {
        self.num = format!("{}{}", self.num, char).parse::<i64>().unwrap();
        self.points.extend([
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col - 1),
            (row, col + 1),
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col + 1),
        ]);
    }
}

fn main() {
    //Solution 1
    let lines = load_file_into_string("./src/input1.txt");
    let mut symbols: HashMap<(i64, i64), char> = HashMap::new();
    let mut parts: Vec<PartNumber> = Vec::new();

    let mut current_number: Option<PartNumber> = None;
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            match char {
                '0'..='9' => {
                    if let Some(ref mut number) = current_number {
                        number.add_digit(char, row as i64, col as i64);
                    } else {
                        current_number = Some(PartNumber::new(char, row as i64, col as i64));
                    }
                }
                '.' => {
                    if let Some(number) = current_number {
                        parts.push(number);
                    }
                    current_number = None;
                }
                _ => {
                    if let Some(number) = current_number {
                        parts.push(number);
                    }
                    current_number = None;

                    symbols.insert((row as i64, col as i64), char);
                }
            }
        }

        if let Some(number) = current_number {
            parts.push(number);
        }
        current_number = None;
    }

    let parts_surrounded_by_symbol: Vec<PartNumber> = parts
        .into_iter()
        .filter(|part| part_is_surrounded(&part, &symbols))
        .collect();

    let sum_of_parts: i64 = parts_surrounded_by_symbol
        .into_iter()
        .map(|part| part.num)
        .sum();

    println!("Solution 1: {}", sum_of_parts);
}

fn part_is_surrounded(part: &PartNumber, symbols: &HashMap<(i64, i64), char>) -> bool {
    return part
        .points
        .clone()
        .into_iter()
        .any(|point| symbols.contains_key(&point));
}

fn load_file_into_string(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Failed to find file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect()
}
