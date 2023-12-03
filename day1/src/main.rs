use std::fs;

fn part1(s: &String) -> i32 {
    let mut count = 0;
    for line in s.lines() {
        let first = line.chars().find(|c| c.is_numeric()).unwrap_or('0');
        let last = line.chars().rev().find(|c| c.is_numeric()).unwrap_or('0');
        count += format!("{}{}", first, last).parse::<i32>().unwrap();
    }
    count
}

fn main() {
    let file_path = "./src/data";
    let s = fs::read_to_string(file_path).expect("File not found !!!");

    println!("Part 1 result: {}", part1(&s));
}
