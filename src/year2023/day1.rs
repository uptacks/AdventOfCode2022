use std::collections::HashMap;
use std::str::MatchIndices;

use itertools::Itertools;

pub struct Day1;
use crate::AdventSolution;

use crate::read_input_for_day;

impl AdventSolution for Day1 {
    const DAY: u32 = 1;
    const YEAR: u32 = 2023;
    fn solve() -> (String, String) {
        // Logic for day N solutions
        let input = read_input_for_day(Self::DAY, Self::YEAR);
        let part_1_solution = part_1_solution(&input).to_string();
        let part_2_solution = part_2_solution(&input).to_string();
        (part_1_solution, part_2_solution)
    }
}

fn part_1_solution(input: &str) -> u64 {
    let mut sum: u32 = 0;
    const RADIX: u32 = 10;

    for line in input.lines() {
        let forward_val = line.chars().find(|x| x.is_numeric()).unwrap();
        let back_val = line.chars().rev().find(|x| x.is_numeric()).unwrap();

        sum = sum + (forward_val.to_digit(RADIX).unwrap() * 10) + back_val.to_digit(RADIX).unwrap();
    }
    return sum as u64;
}

fn part_2_solution(input: &str) -> u64 {
    let digits: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    return 0;
}
