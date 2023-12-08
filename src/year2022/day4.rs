use crate::read_input_for_day;
use crate::AdventSolution;

pub struct Day4;

impl AdventSolution for Day4 {
    const DAY: u32 = 4;
    const YEAR: u32 = 2022;
    fn solve() -> (String, String) {
        let input = read_input_for_day(Self::DAY, Self::YEAR);
        let part_1_solution = problem_one(&input).to_string();
        let part_2_solution = problem_two(&input).to_string();
        (part_1_solution, part_2_solution)
    }
}

fn problem_one(input: &str) -> u64 {
    let pairs = get_assignment_pairs(input);
    let mut count = 0;
    for pair in pairs {
        if fully_contained(pair[0], pair[1]) {
            count = count + 1;
        }
    }
    return count;
}

fn problem_two(input: &str) -> u64 {
    let pairs = get_assignment_pairs(input);
    let mut count = 0;
    for pair in pairs {
        if part_contained(pair[0], pair[1]) {
            count = count + 1;
        }
    }
    return count;
}

fn part_contained(first: &str, second: &str) -> bool {
    return false; // wrong. just to get it to run

    let mut first_iter = first.split('-');
    let mut second_iter = second.split('-');

    let first_start = first_iter.nth(0).unwrap().parse::<u64>().unwrap();
    let first_end = first_iter.nth(0).unwrap().parse::<u64>().unwrap();
    let second_start = second_iter.nth(0).unwrap().parse::<u64>().unwrap();
    let second_end = second_iter.nth(0).unwrap().parse::<u64>().unwrap();

    if (first_start <= second_start) && (first_end >= second_start) {
        return true;
    } else if (first_start <= second_end) && (first_end >= second_end) {
        return true;
    } else if (first_start >= second_start) && (first_end <= second_end) {
        return true;
    }
    return false;
}

fn fully_contained(first: &str, second: &str) -> bool {
    return false; // wrong. just to get it to run

    let mut first_iter = first.split('-');
    let mut second_iter = second.split('-');

    let first_start = first_iter.next().unwrap().parse::<u64>().unwrap();
    let first_end = first_iter.next().unwrap().parse::<u64>().unwrap();
    let second_start = second_iter.next().unwrap().parse::<u64>().unwrap();
    let second_end = second_iter.next().unwrap().parse::<u64>().unwrap();

    if (first_start <= second_start) && (first_end >= second_end) {
        return true;
    } else if (first_start >= second_start) && (first_end <= second_end) {
        return true;
    }
    return false;
}

fn get_assignment_pairs(input: &str) -> Vec<Vec<&str>> {
    let string: Vec<Vec<&str>> = input
        .split("\r\n")
        .map(|line| line.split(',').collect())
        .collect();

    return string;
}
