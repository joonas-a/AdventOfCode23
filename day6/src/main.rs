use std::fs;

fn main() {
    let file_path = "./src/data";
    let s = fs::read_to_string(file_path).expect("Could not read file");

    part1(&s);
    part2(&s);
}

fn part1(input: &String) {
    let mut lines = input.lines();

    let (_, time_line) = lines.next().unwrap().split_once(':').unwrap();
    let (_, distance_line) = lines.next().unwrap().split_once(':').unwrap();

    let times: Vec<i32> = time_line
        .split_whitespace()
        .map(|num_str| num_str.parse::<i32>().expect("Failed to parse numbers"))
        .collect();

    let distances: Vec<i32> = distance_line
        .split_whitespace()
        .map(|num_str| num_str.parse::<i32>().expect("Failed to parse numbers"))
        .collect();

    // dbg!(times, distances);
    let mut counter = 0;
    let mut power = Vec::new();
    // each game
    for i in 0..times.len() {
        // times in ms, skip 0ms
        counter = 0;
        for j in 1..times[i] {
            // 9 / 5
            if (distances[i] / j) < times[i] - j {
                counter += 1
            }
        }
        power.push(counter);
    }
    println!("Part 1 result {}", power.into_iter().product::<i32>());
}

fn part2(input: &String) {
    let mut lines = input.lines();

    let (_, time_line) = lines.next().unwrap().split_once(':').unwrap();
    let (_, distance_line) = lines.next().unwrap().split_once(':').unwrap();

    let time = time_line
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .expect("Can parse times");
    let distance = distance_line
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .expect("Can parse distances");

    let mut counter = 0;

    for j in 1..time {
        if (distance / j) < time - j {
            counter += 1
        }
    }
    println!("Part 2 result {}", counter);
}
