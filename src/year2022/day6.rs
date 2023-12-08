use itertools::Itertools;

use crate::read_input_for_day;
use crate::AdventSolution;

pub struct Day6;

impl AdventSolution for Day6 {
    const DAY: u32 = 6;
    const YEAR: u32 = 2022;
    fn solve() -> (String, String) {
        let input = read_input_for_day(Self::DAY, Self::YEAR);
        let part_1_solution = problem_one(&input).to_string();
        let part_2_solution = problem_two(&input).to_string();
        (part_1_solution, part_2_solution)
    }
}

fn problem_one(input: &str) -> u64 {
    let binding = input.chars().collect_vec();
    let mut window = binding.windows(4);
    let mut count = 4;

    // Loop through windows
    while let Some(w) = window.next() {
        // Check if all characters in the window are unique
        if w.iter().unique().count() == 4 {
            // If so, return the first character
            return count;
        }
        count += 1;
    }
    return 0;
}

fn problem_two(input: &str) -> u64 {
    let binding = input.chars().collect_vec();
    let mut window = binding.windows(14);
    let mut count = 14;

    // Loop through windows
    while let Some(w) = window.next() {
        // Check if all characters in the window are unique
        if w.iter().unique().count() == 14 {
            // If so, return the first character
            return count;
        }
        count += 1;
    }
    return 0;

    return 0;
}
