use std::{collections::HashSet, fs};

fn main() {
    let file_path = "./src/data";
    let s = fs::read_to_string(file_path).expect("File not found");

    part1(&s);
    part2(&s);
}

fn part1(input: &str) {
    let ans = input.lines().fold(0i32, |acc, mut line| {
        (_, line) = line.split_once(':').unwrap();
        let (left, right) = line.split_once('|').unwrap();
        let lset: HashSet<&str> = HashSet::from_iter(left.split_ascii_whitespace());
        let rset: HashSet<&str> = HashSet::from_iter(right.split_ascii_whitespace());
        let isect_len = rset.intersection(&lset).count();
        acc + match isect_len {
            1 => 1,
            n if n > 1 => 2i32.pow(n as u32 - 1),
            _ => 0,
        }
    });
    println!("Part 1 ans: {ans}");
}

fn part2(input: &str) {
    let mut multipliers = vec![1; input.lines().count()];
    let rows = multipliers.len();
    input.lines().enumerate().for_each(|(row_id, mut line)| {
        (_, line) = line.split_once(':').unwrap();
        let (left, right) = line.split_once('|').unwrap();
        let lset: HashSet<&str> = HashSet::from_iter(left.split_ascii_whitespace());
        let rset: HashSet<&str> = HashSet::from_iter(right.split_ascii_whitespace());
        let isect_len = rset.intersection(&lset).count();

        let current_multiplier = *multipliers.get(row_id).unwrap();
        multipliers[(row_id + 1)..=(row_id + isect_len).min(rows)]
            .iter_mut()
            .for_each(|x| *x += current_multiplier);
    });
    println!("Part 2 ans: {:?}", multipliers.iter().sum::<i32>());
}
