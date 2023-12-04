use core::panic;
use std::fs;

fn main() {
    let file_path = "./src/data";
    let s = fs::read_to_string(file_path).expect("File not found !!!");

    println!("{}", part1(&s))
}

// Max allowed values
const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

fn part1(s: &String) -> usize {
    let mut sum = 0;
    for line in s.lines().enumerate() {
        let row = &line.1[line.1.find(':').expect("No : found !") + 2..];
        let states = row.split("; ").collect::<Vec<&str>>();

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
            sum += line.0 + 1
        };
    }
    sum
}
