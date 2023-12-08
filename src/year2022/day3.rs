use crate::read_input_for_day;
use crate::AdventSolution;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day3;

impl AdventSolution for Day3 {
    const DAY: u32 = 3;
    const YEAR: u32 = 2022;
    fn solve() -> (String, String) {
        let input = read_input_for_day(Self::DAY, Self::YEAR);
        let part_1_solution = problem_one(&input).to_string();
        let part_2_solution = problem_two(&input).to_string();
        (part_1_solution, part_2_solution)
    }
}

fn problem_one(input: &str) -> u64 {
    let mut priority: u64 = 0;
    let common: Vec<char> = common_items(input);
    for i in common {
        if i.is_ascii_lowercase() {
            priority = priority + (i as u64 - 96);
        } else {
            priority = priority + (i as u64 - 38);
        }
    }

    return priority;
}

fn problem_two(input: &str) -> u64 {
    let mut priority: u64 = 0;
    let mut common = common_badge(input);
    for i in common {
        if i.is_ascii_lowercase() {
            priority = priority + (i as u64 - 96);
        } else {
            priority = priority + (i as u64 - 38);
        }
    }
    return priority;
}

pub fn common_badge(input: &str) -> Vec<char> {
    let dup_type = input.split("\r\n").collect_vec();

    let mut common: Vec<char> = Vec::new();
    let num_groups = dup_type.len() / 3;

    for i in 0..num_groups {
        let mut first: HashSet<char> = dup_type[i * 3].chars().collect();
        let mut second: HashSet<char> = dup_type[i * 3 + 1].chars().collect();
        let mut third: HashSet<char> = dup_type[i * 3 + 2].chars().collect();
        let first_comp: HashSet<char> = first.intersection(&second).cloned().collect();
        let sec_comp = first_comp.intersection(&third).cloned().next().unwrap();
        common.push(sec_comp);
    }

    return common;
}

pub fn common_items(input: &str) -> Vec<char> {
    let dup_type = input.split("\r\n").collect_vec();

    let mut common: Vec<char> = Vec::new();

    for i in dup_type {
        let half = i.len() / 2;

        let mut first: HashSet<char> = i.chars().take(half).collect();
        let mut second: HashSet<char> = i.chars().skip(half).collect();
        common.push(first.intersection(&second).cloned().next().unwrap());
    }

    return common;
}
