use crate::AdventSolution;
use itertools::Itertools;

pub struct DayN;
use crate::read_input_for_day;

impl AdventSolution for DayN {
    const DAY: u32 = 0;
    const YEAR: u32 = 0;
    fn solve() -> (String, String) {
        // Logic for day N solutions
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
