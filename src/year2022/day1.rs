use crate::AdventSolution;
use itertools::Itertools;

pub struct Day1;
use crate::read_input_for_day;

impl AdventSolution for Day1 {
    const DAY: u32 = 1;
    const YEAR: u32 = 2022;
    fn solve() -> (String, String) {
        // Logic for day 1 solutions
        let input = read_input_for_day(Self::DAY, Self::YEAR);
        let part_1_solution = problem_one(&input).to_string();
        let part_2_solution = problem_two(&input).to_string();
        (part_1_solution, part_2_solution)
    }
}

fn problem_one(input: &str) -> u64 {
    return elf_calorie_vec(input)
        .iter()
        .max()
        .unwrap_or(&0u64)
        .to_owned();
}

fn problem_two(input: &str) -> u64 {
    return elf_calorie_vec(input)
        .iter()
        .sorted()
        .rev()
        .take(3)
        .sum::<u64>();
}

fn elf_calorie_vec(input: &str) -> Vec<u64> {
    let elf: Vec<u64> = input
        .split("\r\n\r\n")
        .map(|elf_collection| {
            elf_collection
                .lines()
                .map(|c| c.parse::<u64>().unwrap_or(0))
                .sum::<u64>()
        })
        .collect::<Vec<u64>>();

    return elf;
}
