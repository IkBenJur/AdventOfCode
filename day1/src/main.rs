use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};
fn main() {
    //First solutionsum_of_second_solution
    let solution1_input = File::open("./src/input.txt").expect("Failed to find file");
    let solution1_reader = BufReader::new(solution1_input);
    let mut sum_of_first_solution: i32 = 0;

    for line in solution1_reader.lines() {
        let calibration_value = match line {
            Ok(calibration_value) => calibration_value,
            Err(error) => panic!("Error: {}", error),
        };

        sum_of_first_solution += find_first_and_last_digit(&calibration_value);
    }

    println!("First solution: {sum_of_first_solution}");

    //Second solution
    let solution2_input = File::open("./src/input2.txt").expect("Failed to find file");
    let solution2_reader = BufReader::new(solution2_input);
    let mut sum_of_second_solution: i32 = 0;

    for line in solution2_reader.lines() {
        let calibration_value = match line {
            Ok(calibration_value) => calibration_value,
            Err(error) => panic!("Error: {}", error),
        };

        sum_of_second_solution += find_first_and_last_written_digit(&calibration_value);
    }

    println!("Second solution: {sum_of_second_solution}");
}

fn find_first_and_last_digit(calibration_value: &String) -> i32 {
    let re = Regex::new(r"\d").unwrap();
    let digits: Vec<&str> = re
        .find_iter(calibration_value)
        .map(|m| m.as_str())
        .collect();

    let sum_of_first_and_last: String;
    if digits.len() == 1 {
        sum_of_first_and_last = format!("{}{}", digits[0], digits[0]);
    } else {
        sum_of_first_and_last = format!(
            "{}{}",
            digits[0],
            digits.last().expect("No last value found")
        );
    }

    return sum_of_first_and_last.parse::<i32>().unwrap();
}

fn find_first_and_last_written_digit(calibration_value: &String) -> i32 {
    let mut digits: Vec<i32> = Vec::new();

    for i in 0..calibration_value.len() {
        let string_check = check_if_string_starts_with_digit(&calibration_value[i..]);
        if let Some(digit) = string_check {
            digits.push(digit);
        }
    }
    
    let sum_of_first_and_last: String;
    if digits.len() == 1 {
        sum_of_first_and_last = format!(
            "{}{}",
            digits[0],
            digits[0]
        );
    } else {
        sum_of_first_and_last = format!(
            "{}{}",
            digits[0],
            digits.last().expect("No last value found")
        );
    }

    return sum_of_first_and_last.parse::<i32>().unwrap();
}

fn check_if_string_starts_with_digit(string_slice: &str) -> Option<i32> {
    let digit_strings = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    for (key, value) in digit_strings.iter() {
        if string_slice.starts_with(key) {
            return Some(*value);
        }
    }
    
    return None;
}
