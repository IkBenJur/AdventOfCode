use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // Solution 1
    let solution1_input = File::open("./src/input1.txt").expect("Failed to find file");
    let solution1_reader = BufReader::new(solution1_input);
    let mut sum_of_game_ids = 0;

    for line in solution1_reader.lines() {
        let game_string = match line {
            Ok(calibration_value) => calibration_value,
            Err(error) => panic!("Error: {}", error),
        };

        let game_id: &str = game_string.split(":").collect::<Vec<_>>()[0].split(" ").collect::<Vec<_>>()[1];
        let subsets: Vec<&str> = game_string.split(":").collect::<Vec<_>>()[1].trim().split(";").collect();

        if let Some(game) =  check_if_game_possible(game_id, subsets) {
            sum_of_game_ids += game;
        }
    }

    println!("{}", sum_of_game_ids);

    // Solution 2
    let solution2 = File::open("./src/input1.txt").expect("Failed to find file");
    let solution2_reader = BufReader::new(solution2);
    let mut sum_of_game_id_power = 0;

    for line in solution2_reader.lines() {
        let game_string = match line {
            Ok(calibration_value) => calibration_value,
            Err(error) => panic!("Error: {}", error),
        };

        let subsets: Vec<&str> = game_string.split(":").collect::<Vec<_>>()[1].trim().split(";").collect();

        sum_of_game_id_power += find_power_of_cubset(subsets);
        
    }

    println!("{}", sum_of_game_id_power);
}

fn check_if_game_possible(game_id: &str, subsets: Vec<&str>) -> Option<i32> {
    let possible_values = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    for game in subsets {
        for cubes_value_pair in game.trim().split(",") {
            let mut parts = cubes_value_pair.split_whitespace();
            
            let cube_value = parts.next().expect("Missing cube value");
            let cube_colour = parts.next().expect("Missing cube color");
            
            if possible_values[cube_colour] < cube_value.parse::<i32>().unwrap() {
                return None;
            }
        }
    }

    return Some(game_id.parse::<i32>().unwrap());
}

fn find_power_of_cubset(subsets: Vec<&str>) -> i32 {
    let mut fewest_possible_cube_values = HashMap::from([
        ("red", 0),
        ("green", 0),
        ("blue", 0),
    ]);

    for game in subsets {
        for cubes_value_pair in game.trim().split(",") {
            let mut parts = cubes_value_pair.split_whitespace();
            
            let cube_value: &str = parts.next().expect("Missing cube value");
            let cube_colour: &str = parts.next().expect("Missing cube color");

            if fewest_possible_cube_values[cube_colour] < cube_value.parse::<i32>().unwrap() {
                *fewest_possible_cube_values.get_mut(cube_colour).unwrap() = cube_value.parse::<i32>().unwrap();
            }
        }
    }
    
    return calculate_power_of_game(fewest_possible_cube_values);
}

fn calculate_power_of_game(game: HashMap<&str, i32>) -> i32 {
    game.values().fold(1, |acc, &x| acc * x)
}
