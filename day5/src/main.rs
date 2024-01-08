use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Clone)]
struct Seed {
    seed_value: i64,
    destination: Option<i64>,
}

impl Seed {
    fn new(seed_value: i64) -> Self {
        Self {
            seed_value,
            destination: Some(seed_value) 
        }
    }

    fn find_next_destination_for_seed(&mut self, map: &Map) {
        if let Some(destination_value) = self.destination {
            if destination_value >= map.destination && destination_value <= map.find_destination_max() {
                // Calculate the offset from the map's destination range
                let offset = (destination_value - map.destination).abs();
                
                // Find the corresponding value in the map's source range
                let source_value = map.source + offset;
    
                // Update the destination with the calculated source value
                self.destination = Some(source_value);
            } else {
                // If destination is outside the map's range, keep the current destination
                self.destination = Some(destination_value);
            }
        } else {
            if self.seed_value >= map.destination && self.seed_value <= map.find_destination_max()  {
                let remainder = map.destination - self.seed_value;
                self.destination = Some(map.source + remainder);
            } else {
                self.destination = Some(self.seed_value);
            }
        }
    }
}

#[derive(Debug)]
struct Map {
    destination: i64,
    source: i64,
    range: i64,
}

impl Map {
    fn new(destination: i64, source: i64, range: i64) -> Self {
        Self {
            destination,
            source,
            range,
        }
    }

    fn find_destination_max(&self) -> i64 {
        self.destination + self.range
    }
}

fn main() {
    let file_lines = load_file_into_vec("./src/inuput.txt");

    let mut seeds = extract_seeds(&file_lines);
    let seed_to_soil_map = extract_map(&file_lines, "seed-to-soil");
    let soil_to_fertilizer_map = extract_map(&file_lines, "soil-to-fertilizer");
    let fertilizer_to_water_map = extract_map(&file_lines, "fertilizer-to-water");
    let water_to_light_map = extract_map(&file_lines, "water-to-light");
    let light_to_temperature_map = extract_map(&file_lines, "light-to-temperature");
    let temperature_to_humidity_map = extract_map(&file_lines, "temperature-to-humidity");
    let humidity_to_location_map = extract_map(&file_lines, "humidity-to-location");

    let journey = vec![
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidity_to_location_map,
    ];

    // println!("{:?}", seed_to_soil_map);

    // for seed in &mut seeds {
    //     for map in &seed_to_soil_map {
    //         seed.find_next_destination_for_seed(&map);
    //     }
    // }

    for seed in &mut seeds {
        for destination in &journey {
            for map in destination {
                seed.find_next_destination_for_seed(map);
            }
        }
    }

    let destination_values: Vec<i64> = seeds.iter().map(|seed| seed.destination.unwrap()).collect();

    println!("{:#?}", seeds);
}

fn load_file_into_vec(input_file: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(input_file).expect("Failed to find file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("Line not found"))
        .collect()
}

fn extract_map(lines: &Vec<String>, map_to_extract: &str) -> Vec<Map> {
    let mut maps: Vec<Map> = Vec::new();
    let mut line_iter = lines.into_iter().cloned();

    while let Some(line) = line_iter.next() {
        if line.contains(map_to_extract) {
            while let Some(map_values) = line_iter.next() {
                if map_values.is_empty() {
                    break;
                }

                let values: Vec<i64> = map_values
                    .split_whitespace()
                    .map(|value| value.parse().unwrap())
                    .collect();

                maps.push(Map::new(values[0], values[1], values[2]))
            }
        }
    }

    return maps;
}

fn extract_seeds(lines: &Vec<String>) -> Vec<Seed> {
    lines
        .iter()
        .nth(0)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|seed_value| Seed::new(seed_value.parse().unwrap()))
        .collect()
}
