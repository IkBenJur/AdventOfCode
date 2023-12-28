use regex::Regex;
use std::{fs::File, io::{BufReader, BufRead}};
fn main() {
    let file = File::open("./src/input.txt").expect("Failed to find file");
    let reader = BufReader::new(file);
    let mut sum: i32 = 0;

    for line in reader.lines(){
        let calibration_value = match line {
            Ok(calibration_value) => calibration_value,
            Err(error) => panic!("Error: {}", error)
        };
        
        sum += find_first_and_last_digit(&calibration_value);
    }

    println!("{sum}");
}

fn find_first_and_last_digit(calibration_value: &String) -> i32 {
    let re = Regex::new(r"\d").unwrap();
    let digits: Vec<&str> = re.find_iter(calibration_value).map(|m| m.as_str()).collect();

    let sum_of_first_and_last: String;
    if digits.len() == 1 {
        sum_of_first_and_last = format!("{}{}", digits[0], digits[0]);
    } else {
        sum_of_first_and_last = format!("{}{}", digits[0], digits.last().expect("No last value found"));
    }

    return sum_of_first_and_last.parse::<i32>().unwrap();
}