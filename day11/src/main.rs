use std::cmp::{max, min};
use std::fs;

fn main() {
    let file_path = "./src/data.txt";
    let s = fs::read_to_string(file_path).expect("Could not read file");

    solve(&s);
}

fn solve(input: &str) {
    let row_width = input.find('\n').unwrap();

    // find which rows and cols have no #
    let rows: Vec<_> = input
        .lines()
        .enumerate()
        .filter(|&(_, row)| row.chars().all(|c| c == '.'))
        .map(|(i, _)| i)
        .collect();

    let cols: Vec<_> = (0..row_width)
        .filter(|&i| {
            input
                .lines()
                .all(|line| line.chars().nth(i).unwrap() == '.')
        })
        .collect();

    let points: Vec<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(row_i, row)| {
            row.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(col_i, _)| (row_i, col_i))
        })
        .collect();

    println!("Part 1 ans: {}", distances(&points, &rows, &cols, 2));
    println!("Part 2 ans: {}", distances(&points, &rows, &cols, 1000000))
}

fn distances(
    points: &[(usize, usize)],
    rows: &[usize],
    cols: &[usize],
    multiplier: usize,
) -> usize {
    let mut res = 0;

    // iterate over every pair once
    // calculate manhattan distance between the points, and return the sum
    for (n, &(row_1, col_1)) in points.iter().enumerate().skip(1) {
        let mut current = 0;
        for &(row_2, col_2) in points.iter().take(n) {
            for r in min(row_1, row_2)..max(row_1, row_2) {
                current += match rows.contains(&r) {
                    true => multiplier,
                    false => 1,
                }
            }
            for c in min(col_1, col_2)..max(col_1, col_2) {
                current += match cols.contains(&c) {
                    true => multiplier,
                    false => 1,
                }
            }
        }
        res += current
    }
    res
}
