use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Clone, Copy)]
struct Seed {
    seed_value: u64,
    destination: u64,
}

impl Seed {
    fn new(seed_value: u64) -> Self {
        Self {
            seed_value,
            destination: seed_value,
        }
    }
    
    fn find_next_destination_for_seed(&self, maps: &Vec<Map>) -> u64 {
        for map in maps {
            if let Some(new_destination) = map.map_destination_number(self.destination) {
                return new_destination
            }
        }
        self.destination
    }
}

#[derive(Debug)]
struct Map {
    destination: u64,
    source: u64,
    range: u64,
}

impl Map {
    fn new(destination: u64, source: u64, range: u64) -> Self {
        Self {
            destination,
            source,
            range,
        }
    }

    fn map_destination_number(&self, seed_destination: u64) -> Option<u64> {
        if seed_destination >= self.source && seed_destination < self.source + self.range {
            Some(seed_destination - self.source + self.destination)
        } else {
            None
        }
    }
}

fn main() {
    let file_lines = load_file_into_vec("./src/inuput.txt");

    let seeds = extract_seeds(&file_lines);
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

    let lowest_destination = seeds.iter().map(|seed| {
        let mut temp_seed = *seed;
    
        for maps in &journey {
            temp_seed.destination = temp_seed.find_next_destination_for_seed(&maps);
        }

        temp_seed.destination
    }).min().unwrap();
    
    println!("{}", lowest_destination);
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

                let values: Vec<u64> = map_values
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
