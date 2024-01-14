use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn load_file_into_vec(input_file: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(input_file).expect("Failed to find file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("Line not found"))
        .collect()
}

fn load_records_from_lines(lines: Vec<String>) -> Vec<Record> {
    let times: Vec<&str> = lines[0]
        .split("Time:")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .collect();
    let distances: Vec<&str> = lines[1]
        .split("Distance:  ")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .collect();
    let mut records: Vec<Record> = Vec::new();

    for i in 0..=times.len() - 1 {
        records.push(Record::new(
            times[i].parse().unwrap(),
            distances[i].parse().unwrap(),
        ));
    }

    records
}

#[derive(Debug)]
struct Record {
    time: i64,
    distance: i64,
}

impl Record {
    fn new(time: i64, distance: i64) -> Self {
        Self { time, distance }
    }

    fn find_possible_number_of_times_to_beat_record(&self) -> i64 {
        let mut amount_of_times_can_win = 0;

        //Tot dat incrementer bij time is
        //Calculate distance for elke mogelijkheid
        //Als distannce > record_distance dan plus 1

        for i in 0..=self.time - 1 {
            // i == speed
            // time left == time - i
            //distance == time_left * speed
            let time_left = self.time - i;
            let distance = time_left * i;

            if distance > self.distance {
                amount_of_times_can_win += 1;
            }
        }

        amount_of_times_can_win
    }
}

fn main() {
    let lines = load_file_into_vec("./src/input.txt");
    let records = load_records_from_lines(lines);
    let solution1: i64 = records
        .into_iter()
        .map(|record| record.find_possible_number_of_times_to_beat_record())
        .product();
    println!("{}", solution1);
}
