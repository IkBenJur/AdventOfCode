use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Clone)]
struct PartNumber {
    num: i64,
    points: HashSet<(i64, i64)>,
}

#[derive(Debug)]
struct Gear {
    point: (i64, i64),
    part_numbers: Vec<PartNumber>,
}

impl Gear {
    fn new(row: i64, col: i64) -> Gear {
        Self {
            point: (row, col),
            part_numbers: Vec::new(),
        }
    }

    fn add_part(&mut self, part: PartNumber) {
        self.part_numbers.push(part);
    }
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
    let mut gears: Vec<Gear> = Vec::new();
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
                '*' => {
                    gears.push(Gear::new(row as i64, col as i64));

                    if let Some(number) = current_number {
                        parts.push(number);
                    }
                    current_number = None;

                    symbols.insert((row as i64, col as i64), char);
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

    let parts_surrounded_by_symbol: Vec<PartNumber> = parts.clone()
        .into_iter()
        .filter(|part| part_is_surrounded(&part, &symbols))
        .collect();

    let sum_of_parts: i64 = parts_surrounded_by_symbol
        .into_iter()
        .map(|part| part.num)
        .sum();

    println!("Solution 1: {}", sum_of_parts);

    let gears = find_all_numbers_and_gears(parts, gears);

    let solution2: i64 = gears
        .iter()
        .filter(|gear| gear.part_numbers.len() == 2)
        .map(|gear| gear.part_numbers.iter().map(|part| part.num).product::<i64>())
        .sum();

    println!("Solution 2: {}", solution2);

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

fn find_all_numbers_and_gears(parts: Vec<PartNumber>, mut gears: Vec<Gear>) -> Vec<Gear> {
    for gear_index in 0..gears.len() {
        let gear_point = gears[gear_index].point;
        for part in parts
            .iter()
            .filter(|part| part.points.contains(&gear_point))
        {
            gears[gear_index].add_part(part.clone());
        }
    }

    gears
}
