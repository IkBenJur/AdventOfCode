use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Clone)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    fn new(left: &str, right: &str) -> Self {
        Self {
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

fn load_file_into_vec(input_file: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(input_file).expect("Failed to find file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("Line not found"))
        .collect()
}

fn create_node_map(input: Vec<String>) -> HashMap<String, Node> {
    let mut node_map = HashMap::new();

    for item in input {
        // Assuming the format is "RBX = (TMF, KTP)"
        let parts: Vec<&str> = item.split(" ").collect();

        let key = parts[0].to_string();
        let left = parts[2].trim_matches(|c| c == '(' || c == ',').to_string();
        let right = parts[3].trim_matches(|c| c == ')' || c == ',').to_string();

        let node = Node::new(&left, &right);
        node_map.insert(key, node);
    }

    node_map
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn main() {
    let instructions = "LRRLRLRRRLLRLRRRLRLLRLRLRRLRLRRLRRLRLRLLRRRLRRLLRRRLRRLRRRLRRLRLRLLRRLRLRRLLRRRLLLRRRLLLRRLRLRRLRLLRRRLRRLRRRLRRLLRRRLRRRLRRRLRLRRLRLRRRLRRRLRRLRLRRLLRRRLRRLLRRLRRLRLRLRRRLRLLRRRLRRLRRRLLRRLLLLLRRRLRRLLLRRRLRRRLRRLRLLLLLRLRRRLRRRLRLRRLLLLRLRRRLLRRRLRRRLRLRLRRLRRLRRLRLRLLLRLRRLRRLRRRLRRRLLRRRR";
    let file_lines = load_file_into_vec("./src/input.txt");
    let nodes = create_node_map(file_lines);

    let start_keys: Vec<String> = nodes
        .keys()
        .clone()
        .filter(|key| key.ends_with("A"))
        .map(|key| key.to_string())
        .collect();

    let mut steps: Vec<i32> = Vec::new();

    for key in start_keys {
        let mut index = 0;
        let mut node_key = key.clone();
        let mut left_or_right: char;
        let mut step = 0;

        loop {
            println!("{}", node_key);
            if node_key.chars().nth(2).unwrap() == 'Z' {
                break;
            }

            if let Some(instruction) = instructions.chars().nth(index) {
                left_or_right = instruction;
                index += 1;
            } else {
                left_or_right = instructions.chars().nth(0).unwrap();
                index = 1;
            }

            let node = nodes.get(&node_key).unwrap();

            node_key = match left_or_right {
                'L' => node.left.clone(),
                'R' => node.right.clone(),
                _ => {
                    panic!("No matched instruction")
                }
            };

            step += 1;
        }

        steps.push(step);
    }

    let mut result = steps[0] as u64;
    for &num in &steps[1..] {
        result = lcm(result as u64, num as u64);
    }

    println!("{:#?}", result);
}
