use std::fs;

// Very expensive and slow brute-force approach
fn main() {
    let file_path = "./src/data.txt";
    let s = fs::read_to_string(file_path).expect("File not found!");

    println!("Answer: {}", part2(&s));
}

fn part2(s: &str) -> i64 {
    let seeds = s
        .lines()
        .take(1)
        .flat_map(|e| e.split_whitespace())
        .filter_map(|e| e.parse::<i64>().ok())
        .collect::<Vec<_>>();

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        let start = seeds[i];
        let end = seeds[i] + seeds[i + 1] - 1;

        let mut merged = false;

        for j in 0..ranges.len() {
            let (range_start, range_end) = ranges[j];

            if start <= range_end && end >= range_start {
                ranges[j] = (start.min(range_start), end.max(range_end));
                merged = true;
                break;
            }
        }

        if !merged {
            ranges.push((start, end));
        }
    }

    println!("Ranges initialized");
    let maps: Vec<(i64, i64, i64)> = s
        .lines()
        .skip(2)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut values = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap_or(0));
            (
                values.next().unwrap_or(0),
                values.next().unwrap_or(0),
                values.next().unwrap_or(0),
            )
        })
        .collect();

    let mut smallest = i64::MAX;
    println!("Maps initialized, start bruteforcing :)");
    for (start, end) in &ranges {
        for seed in *start..=*end {
            let mut source = seed;
            let mut found = false;
            for map in maps.iter() {
                if map == &(0, 0, 0) {
                    found = false;
                    continue;
                }
                if found {
                    continue;
                }
                if &source >= &map.1 && &source <= &(&map.1 + &map.2 - 1) {
                    source = map.0 + (source - &map.1);
                    found = true;
                }
            }
            smallest = smallest.min(source);
        }
    }
    smallest
}
