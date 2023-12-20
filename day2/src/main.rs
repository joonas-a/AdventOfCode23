use core::panic;
use std::fs;

fn main() {
    let file_path = "./src/data.txt";
    let s = fs::read_to_string(file_path).expect("File not found!");

    println!("{}", part1(&s));
    println!("{}", part2(&s));
}

// Max allowed values
const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

fn part1(s: &String) -> usize {
    let mut sum = 0;
    for (index, line) in s.lines().enumerate() {
        let states = &line[line.find(':').expect("No : found !") + 2..]
            .split("; ")
            .collect::<Vec<&str>>();

        let mut is_valid = true;

        for state in states {
            let value = state
                .split(", ")
                .map(|x| x.split_once(' ').expect("No space between color and count"))
                .collect::<Vec<_>>();

            let mut r = 0;
            let mut g = 0;
            let mut b = 0;

            for t in value {
                match t.1 {
                    "red" => r += t.0.parse::<i32>().unwrap(),
                    "green" => g += t.0.parse::<i32>().unwrap(),
                    "blue" => b += t.0.parse::<i32>().unwrap(),
                    _ => panic!("Invalid color!"),
                }
            }
            if r > RED || g > GREEN || b > BLUE {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            sum += index + 1
        };
    }
    sum
}

fn part2(s: &String) -> i32 {
    let mut sum = 0;
    for line in s.lines() {
        let states = &line[line.find(':').expect("No : found !") + 2..]
            .split("; ")
            .collect::<Vec<&str>>();

        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for state in states {
            let value = state
                .split(", ")
                .map(|x| x.split_once(' ').expect("No space between color and count"))
                .collect::<Vec<_>>();

            for t in value {
                match t.1 {
                    "red" => r = r.max(t.0.parse::<i32>().unwrap()),
                    "green" => g = g.max(t.0.parse::<i32>().unwrap()),
                    "blue" => b = b.max(t.0.parse::<i32>().unwrap()),
                    _ => panic!("Invalid color!"),
                }
            }
        }
        sum += r * g * b;
    }
    sum
}
