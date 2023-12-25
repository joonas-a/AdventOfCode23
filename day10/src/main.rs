use core::panic;
use std::fs;

fn main() {
    let file_path = "./src/data.txt";
    let s = fs::read_to_string(file_path).expect("Could not read file");

    solve(&s);
}

enum Move {
    East,
    West,
    South,
    North,
}

fn move_south(c: (usize, usize), prev_move: &mut Move) -> (usize, usize) {
    *prev_move = Move::South;
    (c.0 + 1, c.1)
}
fn move_north(c: (usize, usize), prev_move: &mut Move) -> (usize, usize) {
    *prev_move = Move::North;
    (c.0 - 1, c.1)
}
fn move_east(c: (usize, usize), prev_move: &mut Move) -> (usize, usize) {
    *prev_move = Move::East;
    (c.0, c.1 + 1)
}
fn move_west(c: (usize, usize), prev_move: &mut Move) -> (usize, usize) {
    *prev_move = Move::West;
    (c.0, c.1 - 1)
}

fn solve(input: &str) {
    let pipe_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut steps = 1;
    // Find coords for start
    let mut c = match pipe_map.iter().enumerate().find_map(|(row_id, row)| {
        row.iter()
            .position(|c| *c == 'S')
            .map(|cell_id| (row_id, cell_id))
    }) {
        Some(location) => location,
        None => panic!("No start loc found"),
    };
    // directions to cell: from current -> to west, east, north, south
    let west = ['-', 'L', 'F'];
    let east = ['-', 'J', '7'];
    let north = ['|', '7', 'F'];
    let south = ['|', 'L', 'J'];

    // find initial move from S
    let mut prev_move = Move::East;
    if west.contains(&pipe_map[c.0][c.1 - 1]) {
        c = move_west(c, &mut prev_move);
    } else if east.contains(&pipe_map[c.0][c.1 + 1]) {
        c = move_east(c, &mut prev_move);
    } else if north.contains(&pipe_map[c.0 - 1][c.1]) {
        c = move_north(c, &mut prev_move);
    } else if south.contains(&pipe_map[c.0 + 1][c.1]) {
        c = move_south(c, &mut prev_move);
    } else {
        panic!("Stuck at the start!")
    }
    // follow path until back at the start
    while pipe_map[c.0][c.1] != 'S' {
        steps += 1;

        match pipe_map[c.0][c.1] {
            '-' => match prev_move {
                Move::East => c = move_east(c, &mut prev_move),
                _ => c = move_west(c, &mut prev_move),
            },
            '|' => match prev_move {
                Move::North => c = move_north(c, &mut prev_move),
                _ => c = move_south(c, &mut prev_move),
            },
            'L' => match prev_move {
                Move::South => c = move_east(c, &mut prev_move),
                _ => c = move_north(c, &mut prev_move),
            },
            'J' => match prev_move {
                Move::South => c = move_west(c, &mut prev_move),
                _ => c = move_north(c, &mut prev_move),
            },
            '7' => match prev_move {
                Move::North => c = move_west(c, &mut prev_move),
                _ => c = move_south(c, &mut prev_move),
            },
            'F' => match prev_move {
                Move::North => c = move_east(c, &mut prev_move),
                _ => c = move_south(c, &mut prev_move),
            },
            _ => panic!("Unexpected cell"),
        };
    }

    println!("Part 1 ans: {}", steps / 2);
}
