use std::fs;

fn main() {
    let file_path = "./src/data.txt";
    let s = fs::read_to_string(file_path).expect("File not found!");

    println!("Part 1 result: {}", part1(&s));
    println!("Part 2 result: {}", part2(&s));
}

fn part1(s: &String) -> i32 {
    let mut count = 0;
    for line in s.lines() {
        let first = line.chars().find(|c| c.is_numeric()).unwrap_or('0');
        let last = line.chars().rev().find(|c| c.is_numeric()).unwrap_or('0');
        count += format!("{}{}", first, last).parse::<i32>().unwrap();
    }
    count
}

fn str_to_numeric(s: &str) -> char {
    match s {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => panic!("Should never happen"),
    }
}

fn part2(s: &String) -> i32 {
    let mut count = 0;
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in s.lines() {
        let first = line
            .char_indices()
            .find(|(_, c)| c.is_numeric())
            .unwrap_or((usize::MAX, '0'));

        let second = line
            .char_indices()
            .rfind(|(_, c)| c.is_numeric())
            .unwrap_or((usize::MIN, '0'));

        let mut smallest = (usize::MAX, '0');
        let mut largest = (usize::MIN, '0');

        for &s in &numbers {
            if let Some(i) = line.find(s) {
                if i < smallest.0 {
                    smallest = (i, str_to_numeric(&s))
                }
            };
            if let Some(i) = line.rfind(s) {
                if i + s.len() - 1 > largest.0 {
                    largest = (i + s.len() - 1, str_to_numeric(&s))
                }
            }
        }

        let x = if first.0 < smallest.0 {
            first.1
        } else {
            smallest.1
        };
        let y = if second.0 >= largest.0 {
            second.1
        } else {
            largest.1
        };

        count += format!("{}{}", x, y).parse::<i32>().unwrap();
    }
    count
}
