use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = "./src/data.txt";
    let s = fs::read_to_string(file_path).expect("File not found");

    solve(&s);
}
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Digit {
    value: String,
    location: Vec<(usize, usize)>,
    neighbors: Vec<(usize, usize)>,
}

impl Digit {
    fn new() -> Self {
        Self {
            value: String::new(),
            location: Vec::new(),
            neighbors: Vec::new(),
        }
    }
    fn extend(&mut self, new_value: char, point: (usize, usize), neighbors: Vec<(usize, usize)>) {
        self.value.push(new_value);
        self.location.push(point);
        self.neighbors.extend(neighbors);
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
struct Symbol {
    is_gear: bool,
    location: (usize, usize),
    neighbors: Vec<(usize, usize)>,
}

impl Symbol {
    fn new(is_gear: bool, location: (usize, usize), neighbors: Vec<(usize, usize)>) -> Self {
        Self {
            is_gear,
            location,
            neighbors,
        }
    }
}

fn get_neighbours(point: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for y in -1..=1 {
        for x in -1..=1 {
            let ny = (point.0 as i32 + y) as usize;
            let nx = (point.1 as i32 + x) as usize;

            if nx < width && ny < height && (ny, nx) != point {
                res.push((ny, nx));
            }
        }
    }
    res
}

fn get_part_sum(nums: &HashSet<Digit>, symbols: &HashSet<Symbol>) -> i32 {
    let mut res = 0;
    'num_loop: for num in nums {
        for cell in &num.neighbors {
            if symbols.iter().any(|symb| symb.location == (*cell)) {
                res += num.value.parse::<i32>().unwrap();
                continue 'num_loop;
            }
        }
    }
    res
}

fn get_gear_ratio(nums: &HashSet<Digit>, symbols: &HashSet<Symbol>) -> i32 {
    let mut res = 0;
    let mut nums_cp = nums.clone();
    for gear in symbols.iter().filter(|g| g.is_gear) {
        let mut match_nums: Vec<i32> = Vec::new();
        'gear_cell_loop: for cell in &gear.neighbors {
            for num in nums {
                for num_cell in num.location.iter() {
                    if num_cell == &(cell.clone()) && nums_cp.contains(num) {
                        match_nums.push(num.value.parse().unwrap());
                        nums_cp.remove(num);
                        continue 'gear_cell_loop;
                    }
                }
            }
        }
        if match_nums.len() == 2 {
            res += match_nums.iter().product::<i32>();
        }
        match_nums.clear();
    }
    res
}

fn solve(input: &str) {
    let width = input.lines().take(1).collect::<String>().len();
    let height = input.lines().count();
    let mut nums: HashSet<Digit> = HashSet::new();
    let mut symbols: HashSet<Symbol> = HashSet::new();

    for (line_id, line) in input.lines().enumerate() {
        let mut current_num = Digit::new();

        for (c_id, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                current_num.extend(
                    c,
                    (line_id, c_id),
                    get_neighbours((line_id, c_id), width, height),
                )
            } else if c != '.' {
                symbols.insert(Symbol::new(
                    c == '*',
                    (line_id, c_id),
                    get_neighbours((line_id, c_id), width, height),
                ));
            }
            if (!c.is_ascii_digit() || c_id == width - 1) && !current_num.value.is_empty() {
                nums.insert(current_num);
                current_num = Digit::new();
            }
        }
    }
    println!("Part 1 ans: {}", get_part_sum(&nums, &symbols));
    println!("Part 2 ans: {}", get_gear_ratio(&nums, &symbols));
}
