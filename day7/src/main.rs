use itertools::sorted;
use std::{cmp::Ordering, collections::HashMap, fs};

fn main() {
    let file_path = "./src/data.txt";
    let s = fs::read_to_string(file_path).expect("Could not read file");

    part1(&s);
    part2(&s);
}

#[derive(Debug, Clone)]
struct Hand {
    can_use: bool,
    cards: Vec<i32>,
    sorted: Vec<i32>,
    bid: i32,
}

impl Hand {
    fn new(str_cards: &str, bid: &str) -> Self {
        Self {
            can_use: true,
            cards: str_cards.chars().map(|c| char_to_int(c, false)).collect(),
            sorted: sorted(str_cards.chars().map(|c| char_to_int(c, false)))
                .rev()
                .collect(),
            bid: bid.parse().unwrap(),
        }
    }
    fn with_jokers(str_cards: &str, bid: &str) -> Self {
        let common_char = most_common_card(str_cards);
        Self {
            can_use: true,
            cards: str_cards.chars().map(|c| char_to_int(c, true)).collect(),
            sorted: sorted(str_cards.chars().map(|c| {
                if c == 'J' {
                    char_to_int(common_char, true)
                } else {
                    char_to_int(c, true)
                }
            }))
            .rev()
            .collect(),
            bid: bid.parse().unwrap(),
        }
    }
    fn consume(&mut self) {
        self.can_use = false;
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        for (self_card, other_card) in self.cards.iter().zip(&other.cards) {
            match self_card.cmp(other_card) {
                Ordering::Equal => continue,
                non_equal => return non_equal,
            }
        }
        // line below should never run but is needed
        self.bid.cmp(&other.bid)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hand {}

fn most_common_card(cards: &str) -> char {
    let mut char_count = HashMap::new();

    for ch in cards.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }
    char_count.remove(&'J');
    let max_count = char_count.values().cloned().max().unwrap_or(0);

    char_count
        .into_iter()
        .filter(|&(_, count)| count == max_count)
        .map(|(ch, _)| ch)
        .max_by_key(|&ch| ch as u8)
        .unwrap_or('J')
}

fn char_to_int(card: char, jokers: bool) -> i32 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => {
            if jokers {
                1
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid char!"),
    }
}

fn sort_hands(hands: &mut [Hand]) -> Vec<Hand> {
    let mut res: Vec<Hand> = Vec::new();
    // in each step eliminate one type of hand, starting from five of a kind
    for step in 1..=7 {
        let mut sorting_vec: Vec<Hand> = Vec::new();
        match step {
            1 => {
                for hand in hands.iter_mut().filter(|x| x.can_use) {
                    if hand.sorted[0] == hand.sorted[4] {
                        sorting_vec.push(hand.clone());
                        hand.consume();
                    }
                }
            }
            2 => {
                for hand in hands.iter_mut().filter(|x| x.can_use) {
                    if hand.sorted[0] == hand.sorted[3] || hand.sorted[1] == hand.sorted[4] {
                        sorting_vec.push(hand.clone());
                        hand.consume();
                    }
                }
            }
            3 => {
                for hand in hands.iter_mut().filter(|x| x.can_use) {
                    if hand.sorted[0] == hand.sorted[2] && hand.sorted[3] == hand.sorted[4]
                        || hand.sorted[0] == hand.sorted[1] && hand.sorted[2] == hand.sorted[4]
                    {
                        sorting_vec.push(hand.clone());
                        hand.consume();
                    }
                }
            }
            4 => {
                for hand in hands.iter_mut().filter(|x| x.can_use) {
                    if hand.sorted[0] == hand.sorted[2]
                        || hand.sorted[1] == hand.sorted[3]
                        || hand.sorted[2] == hand.sorted[4]
                    {
                        sorting_vec.push(hand.clone());
                        hand.consume();
                    }
                }
            }
            5 => {
                for hand in hands.iter_mut().filter(|x| x.can_use) {
                    if hand.sorted[0] == hand.sorted[1] && hand.sorted[2] == hand.sorted[3]
                        || hand.sorted[1] == hand.sorted[2] && hand.sorted[3] == hand.sorted[4]
                        || hand.sorted[0] == hand.sorted[1] && hand.sorted[3] == hand.sorted[4]
                    {
                        sorting_vec.push(hand.clone());
                        hand.consume();
                    }
                }
            }
            6 => {
                for hand in hands.iter_mut().filter(|x| x.can_use) {
                    if hand.sorted[0] == hand.sorted[1]
                        || hand.sorted[1] == hand.sorted[2]
                        || hand.sorted[2] == hand.sorted[3]
                        || hand.sorted[3] == hand.sorted[4]
                    {
                        sorting_vec.push(hand.clone());
                        hand.consume();
                    }
                }
            }
            _ => {
                for hand in hands.iter_mut().filter(|x| x.can_use) {
                    sorting_vec.push(hand.clone());
                }
            }
        }
        sorting_vec.sort();
        res.extend(sorting_vec.clone().into_iter().rev());
        sorting_vec.clear();
    }
    res
}

fn part1(input: &str) {
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let (cards, bid) = line.split_once(' ').expect("cant split row in half");
        hands.push(Hand::new(cards, bid))
    }
    let res = sort_hands(&mut hands)
        .iter()
        .rev()
        .enumerate()
        .fold(0usize, |acc, (index, hand)| {
            acc + (index + 1) * hand.bid as usize
        });
    println!("Part 1 ans: {res}");
}

fn part2(input: &str) {
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let (cards, bid) = line.split_once(' ').expect("cant split row in half");
        hands.push(Hand::with_jokers(cards, bid))
    }

    let res = sort_hands(&mut hands)
        .iter()
        .rev()
        .enumerate()
        .fold(0usize, |acc, (index, hand)| {
            acc + (index + 1) * hand.bid as usize
        });
    println!("Part 2 ans: {res}");
}
