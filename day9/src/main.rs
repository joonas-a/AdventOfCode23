use std::fs;

fn main() {
    let file_path = "./src/data.txt";
    let s = fs::read_to_string(file_path).expect("Could not read file");

    helper(&s);
}

fn helper(input: &str) {
    let all_vecs: Vec<Vec<Vec<i32>>> = input
        .lines()
        .map(|line| {
            let mut cur_vec: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            let mut vecs = std::iter::once(cur_vec.clone()).collect::<Vec<Vec<i32>>>();

            while !cur_vec.iter().all(|&x| x == 0) {
                let next_vec: Vec<i32> = cur_vec.windows(2).map(|nums| nums[1] - nums[0]).collect();
                cur_vec = next_vec.clone();
                vecs.push(next_vec);
            }

            vecs
        })
        .collect();

    part1(&all_vecs);
    part2(&all_vecs);
}

fn part1(vecs: &[Vec<Vec<i32>>]) {
    let res = vecs.iter().fold(0i32, |acc, cur_vec| {
        acc + cur_vec
            .iter()
            .rev()
            .fold(0i32, |acc, x| acc + x.last().unwrap_or(&0))
    });
    println!("Part 1 ans: {res}")
}

fn part2(vecs: &[Vec<Vec<i32>>]) {
    let res = vecs.iter().fold(0i32, |acc, cur_vec| {
        acc + cur_vec
            .iter()
            .rev()
            .fold(0i32, |acc, x| -acc + x.first().unwrap())
    });
    println!("Part 2 ans: {res}")
}
