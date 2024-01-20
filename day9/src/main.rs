use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn vec_diff(input: &[i32]) -> Vec<i32> {
    let vals = input.iter();
    let next_vals = input.iter().skip(1);

    vals.zip(next_vals).map(|(cur, next)| next - cur).collect()
}

fn extrapolate_new_num(mut diffs: Vec<Vec<i32>>) -> i32 {
    //Voeg the laaste num van de laatse arr plus arr daarvoor aan arr daarvoor

    for i in (1..diffs.len()).rev() {
        let arr = diffs[i].clone();
        let next_arr = diffs[i - 1].clone();

        diffs[i - 1].push(next_arr.last().unwrap() + arr.last().unwrap());
    }

    if let Some(extrapolated_num) = diffs.first().unwrap().last() {
        return *extrapolated_num;
    }

    return 0;
}

fn load_file_into_vec(input_file: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(input_file).expect("Failed to find file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("Line not found"))
        .collect()
}

fn main() {
    let lines = load_file_into_vec("./src/input.txt");
    let values: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect())
        .collect();
    let mut sum = 0;

    for line in values {
        let mut diffs: Vec<Vec<i32>> = Vec::from([line]);
    
        while !diffs
            .last()
            .into_iter()
            .all(|val| val.iter().all(|&inner_val| inner_val == 0))
        {
            if let Some(last_value) = diffs.last() {
                diffs.push(vec_diff(last_value));
            }
        }

        sum += extrapolate_new_num(diffs);
    }

    println!("{:#?}", sum);
}
