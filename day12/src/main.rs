use std::fs;

fn main() {
    let file_path = "./src/data.txt";
    let s = fs::read_to_string(file_path).expect("Could not read file");

    solve(&s);
}

fn solve(input: &str) {
    let mut result = 0;
    for line in input.lines() {
        let (puzzle, values) = line.split_once(' ').unwrap();

        let permutations = generate_permutations(puzzle);
        result += count_valid(
            &permutations,
            values.split(',').map(|v| v.parse().unwrap()).collect(),
        );
    }
    println!("Part 1 ans: {result}");
}

fn generate_permutations(template: &str) -> Vec<String> {
    let mut permutations: Vec<String> = vec!["".to_string()];

    for c in template.chars() {
        let mut next_permutations = Vec::new();

        for permutation in permutations {
            match c {
                '?' => {
                    next_permutations.push(format!("{}#", permutation));
                    next_permutations.push(format!("{}.", permutation));
                }
                _ => {
                    next_permutations.push(format!("{}{}", permutation, c));
                }
            }
        }
        permutations = next_permutations;
    }
    permutations
}

fn count_valid(input: &[String], values: Vec<i32>) -> i32 {
    let mut res = 0;
    'outer: for x in input.iter().map(|x| x.trim_matches('.')) {
        if x.is_empty() {
            continue;
        }
        let mut streak = 0;
        let mut i = 0;

        for (c_id, c) in x.char_indices() {
            if c == '#' {
                streak += 1
            } else if streak > 0 {
                if values.get(i).is_some_and(|v| *v == streak) {
                    i += 1;
                    streak = 0;
                } else {
                    continue 'outer;
                }
            }
            if c_id == x.len() - 1
                && !(values.get(i).is_some_and(|v| *v == streak) && i == values.len() - 1)
            {
                continue 'outer;
            }
        }
        res += 1
    }
    res
}
