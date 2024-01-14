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

#[derive(Debug, Clone, Copy)]
struct Seed2 {
    start: u64,
    to: u64,
}

impl Seed2 {
    fn new(seed_value: u64, to: u64) -> Self {
        Self {
            start: seed_value,
            to,
        }
    }
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
                return new_destination;
            }
        }
        self.destination
    }
}

struct Map2 {
    ranges: Vec<Range>,
}

impl Map2 {
    fn new(ranges: Vec<Range>) -> Self {
        Self { ranges }
    }

    fn find_new_destinations_for_seeds(self, seeds: Vec<Seed2>) -> Vec<Seed2> {
        let mut transformed_seeds: Vec<Seed2> = Vec::new();

        for seed in seeds {
            let mut seed_transformed = false;

            for map in &self.ranges {
                // Condition 1: Seeds entirely outside the range
                if (seed.to < map.source) || (seed.start > map.source + map.range) {
                    continue; // Skip to the next map
                }

                // Condition 2: Seed entirely within the range
                if (seed.start >= map.source) && (seed.to <= map.source + map.range) {
                    let new_start = seed.start - map.source + map.destination;
                    let new_to = seed.to - map.source + map.destination;
                    transformed_seeds.push(Seed2::new(new_start, new_to));
                    seed_transformed = true;
                    break; // Seed transformed, move to the next seed
                }

                // Condition 3: Seed starts before the range and ends within it
                if (seed.start <= map.source) && (seed.to < map.source + map.range) {
                    let new_start = map.source;
                    let new_to = seed.to - map.source + map.destination;
                    transformed_seeds.push(Seed2::new(new_start, new_to));

                    // Also add the untransformed part before the range
                    let untransformed_to = map.source - 1;
                    transformed_seeds.push(Seed2::new(seed.start, untransformed_to));

                    seed_transformed = true;
                    break; // Seed transformed, move to the next seed
                }

                // Condition 4: Seed starts before and ends after the range
                if (seed.start < map.source) && (seed.to > map.source + map.range) {
                    let untransformed_to1 = map.source - 1;
                    transformed_seeds.push(Seed2::new(seed.start, untransformed_to1));

                    let new_start = map.source;
                    let new_to = map.source + map.range;

                    
                    // let new_start = seed.start - map.source + map.destination;
                    // let new_to = seed.to - map.source + map.destination;

                    transformed_seeds.push(Seed2::new(new_start, new_to));

                    let untransformed_start2 = map.source + map.range + 1;
                    transformed_seeds.push(Seed2::new(untransformed_start2, seed.to));

                    seed_transformed = true;
                    break; // Seed transformed, move to the next seed
                }

                // Condition 5: Seeds start within but end outside the range
                if (seed.start >= map.source) && (seed.to > map.source + map.range) {
                    let new_start = seed.start - map.source + map.destination;
                    let new_to = map.source + map.range;
                    transformed_seeds.push(Seed2::new(new_start, new_to));

                    // Also add the untransformed part after the range
                    let untransformed_start = map.source + map.range + 1;
                    transformed_seeds.push(Seed2::new(untransformed_start, seed.to));

                    seed_transformed = true;
                    break; // Seed transformed, move to the next seed
                }
            }

            // If the seed wasn't transformed, add it as is
            if !seed_transformed {
                transformed_seeds.push(seed);
            }
        }

        transformed_seeds
    }
    // for seed in seeds {
    //     let mut seed_transformed = false;

    //     for map in &self.ranges {
    //         // Condition 1: Seeds don't match the current map
    //         if (seed.to < map.source) || seed.start > map.source + map.range {
    //             continue; // Skip to the next map
    //         }
    //         // Condition 2: Seed entirely within the range
    //         else if (seed.start >= map.source) && (seed.to <= map.source + map.range) {
    //             updated_seeds.push(Seed2::new(
    //                 seed.start - map.source + map.destination,
    //                 seed.to - map.source + map.destination,
    //             ));
    //             seed_transformed = true;
    //             break; // Seed transformed, move to the next seed
    //         }
    //         // Condition 3: Seed starts before the range and ends within it
    //         else if (seed.start <= map.source) && (seed.to < map.source + map.range) {
    //             updated_seeds.push(Seed2::new(
    //                 seed.start - map.source + map.destination,
    //                 seed.to + map.destination - map.source,
    //             ));
    //             updated_seeds.push(Seed2::new(
    //                 seed.to + map.destination - map.source + 1,
    //                 seed.to + map.destination,
    //             ));
    //             seed_transformed = true;
    //             break; // Seed transformed, move to the next seed
    //         }
    //         // Condition 4: Seed starts before and ends after the range
    //         else if (seed.start < map.source) && (seed.to > map.source + map.range) {
    //             updated_seeds.push(Seed2::new(map.destination, map.destination + map.range));
    //             updated_seeds.push(Seed2::new(seed.start, map.source));
    //             updated_seeds.push(Seed2::new(map.source + map.range, seed.to));
    //             seed_transformed = true;
    //             break; // Seed transformed, move to the next seed
    //         }
    //         // Condition 5: Seeds start within but end outside the range
    //         else if (seed.start >= map.source) && (seed.to > map.source + map.range) {
    //             updated_seeds.push(Seed2::new(
    //                 map.destination,
    //                 map.destination + (map.range - (seed.start - map.source)),
    //             ));
    //             updated_seeds.push(Seed2::new(map.source + map.range, seed.to));
    //             seed_transformed = true;
    //             break; // Seed transformed, move to the next seed
    //         }
    //     }

    //     // If the seed is not transformed by any map, add it to the updated seeds
    //     if !seed_transformed {
    //         updated_seeds.push(seed);
    //     }
    // }

    // updated_seeds

    // let mut updated_seeds: Vec<Seed2> = Vec::new();

    // for seed in seeds {
    //     for range in &self.ranges {
    //         // Condition 1: Seeds don't match any range.
    //         if (seed.to < range.source) || (seed.start > range.source + range.range) {
    //             updated_seeds.push(seed);
    //         }
    //         // Condition 2: Seed entirely within the range.
    //         else if (seed.start >= range.source) && (seed.to <= range.source + range.range) {
    //             updated_seeds.push(Seed2::new(
    //                 seed.start - range.source + range.destination,
    //                 seed.to - range.source + range.destination,
    //             ));
    //         }
    //         // Condition 3: Seed starts before the range and ends within it.
    //         else if (seed.start <= range.source) && (seed.to < range.source + range.range) {
    //             updated_seeds.push(Seed2::new(
    //                 seed.start - range.source + range.destination,
    //                 seed.to + range.destination - range.source,
    //             ));
    //             updated_seeds.push(Seed2::new(
    //                 seed.to + range.destination - range.source + 1,
    //                 seed.to + range.destination,
    //             ));
    //         }
    //         // Condition 4: Seed starts before the range and ends after it.
    //         else if (seed.start < range.source) && (seed.to > range.source + range.range) {
    //             updated_seeds.push(Seed2::new(
    //                 range.destination,
    //                 range.destination + range.range,
    //             ));
    //             updated_seeds.push(Seed2::new(
    //                 seed.start,
    //                 range.source,
    //             ));
    //             updated_seeds.push(Seed2::new(
    //                 range.source + range.range,
    //                 seed.to,
    //             ));
    //         }
    //         // Condition 5: Seeds start within the range but end outside it.
    //         else if (seed.start >= range.source) && (seed.to > range.source + range.range) {
    //             updated_seeds.push(Seed2::new(
    //                 range.destination,
    //                 range.destination + (range.range - (seed.start - range.source)),
    //             ));
    //             updated_seeds.push(Seed2::new(
    //                 range.source + range.range,
    //                 seed.to,
    //             ));
    //         }
    //     }
    // }

    // updated_seeds
}

#[derive(Copy, Clone)]
struct Range {
    destination: u64,
    source: u64,
    range: u64,
}

impl Range {
    fn new(destination: u64, source: u64, range: u64) -> Self {
        Self {
            destination,
            source,
            range,
        }
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

    let lowest_destination = seeds
        .iter()
        .map(|seed| {
            let mut temp_seed = *seed;

            for maps in &journey {
                temp_seed.destination = temp_seed.find_next_destination_for_seed(&maps);
            }

            temp_seed.destination
        })
        .min()
        .unwrap();

    println!("{}", lowest_destination);

    // Solution 2

    let mut seeds2 = extract_seeds2(&file_lines);
    let seed_to_soil_map2 = extract_map2(&file_lines, "seed-to-soil");
    let soil_to_fertilizer_map2 = extract_map2(&file_lines, "soil-to-fertilizer");
    let fertilizer_to_water_map2 = extract_map2(&file_lines, "fertilizer-to-water");
    let water_to_light_map2 = extract_map2(&file_lines, "water-to-light");
    let light_to_temperature_map2 = extract_map2(&file_lines, "light-to-temperature");
    let temperature_to_humidity_map2 = extract_map2(&file_lines, "temperature-to-humidity");
    let humidity_to_location_map2 = extract_map2(&file_lines, "humidity-to-location");

    let journey = vec![
        seed_to_soil_map2,
        soil_to_fertilizer_map2,
        fertilizer_to_water_map2,
        water_to_light_map2,
        light_to_temperature_map2,
        temperature_to_humidity_map2,
        humidity_to_location_map2,
    ];
    // println!("{:#?}", seeds2);
    // seeds2 = seed_to_soil_map2.find_new_destinations_for_seeds(seeds2);
    for map in journey {
        seeds2 = map.find_new_destinations_for_seeds(seeds2);
    }

    println!("{:#?}", seeds2);

    // let lowest_destination = seeds2
    //     .iter()
    //     .map(|seed| {
    //         let mut temp_seed = seed.clone();

    //         for maps in &journey {
    //             temp_seed.destination = temp_seed.find_next_destination_for_seed(&maps);
    //         }

    //         temp_seed
    //     })
    //     .flatten() // Flatten the vectors of destinations into a single iterator
    //     .min()
    //     .unwrap();

    // let new_dist: Vec<Seed2> = seeds2.iter().map(|seed| {
    //     let mut temp_seed = seed.clone();
    //     for map in &journey {
    //         temp_seed.destination = seed.find_next_destination_for_seed(&map);
    //     }
    //     temp_seed
    // }).collect();
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

fn extract_map2(lines: &Vec<String>, map_to_extract: &str) -> Map2 {
    let mut ranges: Vec<Range> = Vec::new();
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

                ranges.push(Range::new(values[0], values[1], values[2]))
            }
        }
    }

    return Map2::new(ranges);
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

fn extract_seeds2(lines: &Vec<String>) -> Vec<Seed2> {
    let seed_values: Vec<&str> = lines
        .iter()
        .nth(0)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect();

    let mut seed_iter = seed_values.iter();
    let mut seeds: Vec<Seed2> = Vec::new();

    while let Some(seed_value_str) = seed_iter.next() {
        let seed_range_str = seed_iter.next().unwrap(); // Handle the case where there's an odd number of values
        let seed_value: u64 = seed_value_str.parse().unwrap();
        let seed_range: u64 = seed_range_str.parse().unwrap();

        seeds.push(Seed2::new(seed_value, seed_value + seed_range - 1));
    }

    seeds
}
