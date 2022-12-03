use crate::day2::RPS::{Paper, Rock, Scissors};
use crate::utils::read_file_string;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

pub fn part1() {
    let input = read_file_string("input/day2.txt").unwrap();

    let lines = input.lines();
    let mut total = 0_u32;

    for line in lines {
        let opponent_pick = match line.chars().nth(0).unwrap() {
            'A' => RPS::Rock,
            'B' => RPS::Paper,
            'C' => RPS::Scissors,
            _ => Rock
        };

        let pick = match line.chars().nth(2).unwrap() {
            'X' => RPS::Rock,
            'Y' => RPS::Paper,
            'Z' => RPS::Scissors,
            _ => Rock
        };
        let score = calc_score(pick, opponent_pick);
        total += score;

    }
    println!("{}", total);
}

pub fn part2() {
    let input = read_file_string("input/day2.txt").unwrap();

    let lines = input.lines();
    let mut total = 0_u32;

    for line in lines {
        let opponent_pick = match line.chars().nth(0).unwrap() {
            'A' => RPS::Rock,
            'B' => RPS::Paper,
            'C' => RPS::Scissors,
            _ => Rock
        };

        let pick = calculate_pick(line.chars().nth(2).unwrap(), opponent_pick);

        let score = calc_score(pick, opponent_pick);
        total += score;

    }
    println!("{}", total);

}

fn calculate_pick(result: char, opponnent_pick: RPS) -> RPS {
    match result {
        'Z' => get_weakness(&opponnent_pick),
        'Y' => opponnent_pick,
        'X' => get_weakness(&get_weakness(&opponnent_pick)),
        _ => opponnent_pick
    }
}

fn calc_score(your: RPS, opponent: RPS) -> u32 {
    let base =  your as u32;
    println!("Base is: {}", base);
    if your == opponent {
        return base + 3;
    }

    if opponent == get_weakness(&your) {
        return base;
    } else {
        return base + 6;
    }

}

fn get_weakness(pick: &RPS) -> RPS {
    match pick {
        Rock => Paper,
        Paper => Scissors,
        Scissors => Rock
    }
}