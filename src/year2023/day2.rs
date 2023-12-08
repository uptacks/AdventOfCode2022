use crate::AdventSolution;
use itertools::Itertools;

pub struct Day2;
use crate::read_input_for_day;

impl AdventSolution for Day2 {
    const DAY: u32 = 2;
    const YEAR: u32 = 2023;
    fn solve() -> (String, String) {
        let input = read_input_for_day(Self::DAY, Self::YEAR);
        let part_1_solution = part_1_solution(&input).to_string();
        let part_2_solution = part_2_solution(&input).to_string();
        (part_1_solution, part_2_solution)
    }
}

fn part_1_solution(input: &str) -> u64 {
    return 0;
}

fn part_2_solution(input: &str) -> u64 {
    return 0;
}
