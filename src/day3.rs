use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn get_priority(c: &char) -> u32 {
    let adjustment: u32 = if c.is_ascii_lowercase() { 96 } else { 38 };

    *c as u32 - adjustment
}

pub fn split_backpack(text: &str) -> (&str, &str) {
    let half = text.len() / 2;
    text.split_at(half)
}

pub fn find_duplicates(left: &str, right: &str) -> Vec<char> {
    let left_chars: HashSet<char> = left.chars().collect();
    let right_chars: HashSet<char> = right.chars().collect();

    intersect(left_chars, &right_chars)
}

fn intersect(left_chars: HashSet<char>, right_chars: &HashSet<char>) -> Vec<char> {
    left_chars.intersection(&right_chars).map(|i| *i).collect()
}

pub fn part1() {
    let input = include_str!("../input/day3.txt");

    let lines = input.lines();
    let mut sum: u32 = 0;

    for line in lines {
        let (left, right) = split_backpack(line);
        let duplicate_chars = find_duplicates(left, right);
        sum += get_priority(duplicate_chars.get(0).unwrap());
    }

    println!("sum is: {}", sum);
}

pub fn part2() {
    let input = include_str!("../input/day3.txt");

    let lines = input.lines();

    let sum: u32 = lines
        .map(|line| line.chars().collect::<HashSet<char>>())
        .tuples()
        .map(|(a, b, c)| {
            let res: HashSet<char> = intersect(a, &b).into_iter().collect();
            let final_res = intersect(res, &c);
            return get_priority(&final_res.get(0).unwrap());
        })
        .sum();

    println!("sum is: {}", sum);
}
