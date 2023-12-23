use num_integer::lcm;
use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "./src/data.txt";
    let s = fs::read_to_string(file_path).expect("Could not read file");

    solve(&s);
}

fn is_start(key: &str) -> bool {
    key.ends_with('A')
}

fn is_end(value: &str) -> bool {
    value.ends_with('Z')
}

fn solve(input: &str) {
    let (instructions, steps) = input.split_once("\n\n").unwrap();

    let network: HashMap<&str, (&str, &str)> = steps
        .lines()
        .map(|line| {
            let key = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            (key, (left, right))
        })
        .collect();

    part1(instructions, network.clone());
    part2(instructions, network);
}

fn part1(instructions: &str, network: HashMap<&str, (&str, &str)>) {
    let mut index = 0usize;
    let length = instructions.chars().count();
    let mut res = 0;
    let (mut key, mut value) = network.get_key_value("AAA").unwrap();
    while key != &"ZZZ" {
        let next_key = match instructions.chars().nth(index).unwrap() {
            'L' => value.0,
            'R' => value.1,
            _ => panic!("Unexpected instruction"),
        };
        (key, value) = network.get_key_value(next_key).unwrap();
        res += 1;
        index = (index + 1) % length;
    }
    println!("Part 1 ans: {res}");
}

fn part2(instructions: &str, network: HashMap<&str, (&str, &str)>) {
    let starting_nodes: Vec<_> = network.iter().filter(|entry| is_start(entry.0)).collect();
    let mut cycle_lengths: Vec<i32> = Vec::new();

    // for each starting point, find the number of steps for a complete cycle
    let length = instructions.chars().count();
    for node in starting_nodes.into_iter() {
        let (mut key, mut value) = network.get_key_value(node.0).unwrap();
        let mut index = 0;
        let mut counter = 0;
        let mut cycle_size = 0;
        loop {
            if is_end(key) {
                if cycle_size == counter {
                    cycle_lengths.push(counter);
                    break;
                }
                cycle_size = counter;
                counter = 0;
            }
            let next_key = match instructions.chars().nth(index).unwrap() {
                'L' => value.0,
                'R' => value.1,
                _ => panic!("Unexpected instruction"),
            };
            (key, value) = network.get_key_value(next_key).unwrap();
            counter += 1;
            index = (index + 1) % length;
        }
    }
    let result = cycle_lengths
        .iter()
        .fold(1i64, |acc, &num| lcm(acc, num as i64));
    println!("Part 2 ans: {result}");
}
