use std::fs;

fn main() {
    let file_path = "./src/data.txt";
    let s = fs::read_to_string(file_path).expect("Could not read file");

    solve(&s);
}

fn solve(input: &str) {
    let mut res = 0;
    for puzzle in input.split("\n\n") {
        res += find_reflection(puzzle);
    }
    println!("Part 1 ans: {res}");
}

fn find_reflection(puzzle: &str) -> usize {
    let mut matrix: Vec<Vec<char>> = puzzle.lines().map(|line| line.chars().collect()).collect();

    fn mirror_finder(matrix: &Vec<Vec<char>>, multiplier: usize) -> Option<usize> {
        for i in 0..matrix.len() - 1 {
            if matrix[i] == matrix[i + 1] && compare_rows(matrix, matrix.len() - 1, (i, i + 1)) {
                return Some((1 + i) * multiplier);
            }
        }
        None
    }

    // Check for horizontal mirror
    if let Some(res) = mirror_finder(&matrix, 100) {
        return res;
    }

    // Check for "vertical" mirror by rotating original 90 degrees clockwise
    rotate_matrix(&mut matrix);
    if let Some(res) = mirror_finder(&matrix, 1) {
        return res;
    }

    0
}

fn compare_rows(rows: &[Vec<char>], max_id: usize, init_match: (usize, usize)) -> bool {
    let mut step: usize = 0;
    while rows[init_match.0 - step] == rows[init_match.1 + step] {
        step += 1;
        if step > init_match.0 || init_match.1 + step > max_id {
            return true;
        }
    }
    false
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
