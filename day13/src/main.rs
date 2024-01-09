use std::fs;

fn main() {
    let file_path = "./src/data.txt";
    let s = fs::read_to_string(file_path).expect("Could not read file");

    solve(&s);
}

fn solve(input: &str) {
    let mut res1 = 0;
    let mut res2 = 0;
    for puzzle in input.split("\n\n") {
        res1 += find_reflection(puzzle, 0);
        res2 += find_reflection(puzzle, 1);
    }
    println!("Part 1 ans: {res1}");
    println!("Part 2 and: {res2}");
}

fn find_reflection(puzzle: &str, smudge_limit: usize) -> usize {
    let mut matrix: Vec<Vec<char>> = puzzle.lines().map(|line| line.chars().collect()).collect();

    fn mirror_finder(
        matrix: &Vec<Vec<char>>,
        multiplier: usize,
        smudge_limit: usize,
    ) -> Option<usize> {
        for i in 0..matrix.len() - 1 {
            if compare_rows(matrix, matrix.len() - 1, (i, i + 1), smudge_limit) {
                return Some((1 + i) * multiplier);
            }
        }
        None
    }

    // Check for horizontal mirror
    if let Some(res) = mirror_finder(&matrix, 100, smudge_limit) {
        return res;
    }

    // Check for "vertical" mirror by rotating original 90 degrees clockwise
    rotate_matrix(&mut matrix);
    if let Some(res) = mirror_finder(&matrix, 1, smudge_limit) {
        return res;
    }

    0
}

fn compare_rows(
    rows: &[Vec<char>],
    max_id: usize,
    init_match: (usize, usize),
    smudge_limit: usize,
) -> bool {
    let mut step: usize = 0;
    let mut smudges = 0;

    while !(step > init_match.0 || init_match.1 + step > max_id) {
        rows[init_match.0 - step]
            .iter()
            .zip(rows[init_match.1 + step].iter())
            .for_each(|(a, b)| {
                if a != b {
                    smudges += 1
                }
            });
        step += 1;
    }
    smudges == smudge_limit
}

fn rotate_matrix(matrix: &mut Vec<Vec<char>>) {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut transposed = vec![vec!['0'; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j];
        }
    }

    for row in transposed.iter_mut() {
        row.reverse();
    }

    *matrix = transposed;
}
