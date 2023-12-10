use crate::AdventSolution;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day3;
use crate::read_input_for_day;

impl AdventSolution for Day3 {
    const DAY: u32 = 3;
    const YEAR: u32 = 2023;
    fn solve() -> (String, String) {
        // Logic for day N solutions
        let input = read_input_for_day(Self::DAY, Self::YEAR);
        let part_1_solution = part_1_solution(&input).to_string();
        let part_2_solution = part_2_solution(&input).to_string();
        (part_1_solution, part_2_solution)
    }
}

fn get_number_array(input: &str) -> Vec<Vec<(usize, i64)>> {
    let mut arr: Vec<Vec<(usize, i64)>> = Vec::new();

    for line in input.lines() {
        let mut line_result: Vec<(usize, i64)> = Vec::new();
        let mut current_number = String::new();
        let mut start_index = None;

        for (id, char) in line.char_indices() {
            if char.is_digit(10) {
                if start_index.is_none() {
                    start_index = Some(id);
                }

                current_number.push(char);
            } else if !current_number.is_empty() {
                if let Some(start) = start_index {
                    if let Ok(number) = current_number.parse::<i64>() {
                        line_result.push((start, number));
                    }
                }
                current_number.clear();
                start_index = None;
            }

            if !current_number.is_empty() {
                if let Some(start) = start_index {
                    if let Ok(number) = current_number.parse::<i64>() {
                        line_result.push((start, number));
                    }
                }
            }
        }
        arr.push(line_result);
    }
    return arr;
}

fn get_symbol_array(input: &str) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let special_chars = line
            .char_indices()
            .filter(|&(_, c)| c.is_ascii_punctuation() && c != '.')
            .map(|(x, _)| (x, i));
        result.extend(special_chars);
    }
    return result;
}

fn part_1_solution(input: &str) -> u64 {
    let mut sum = 0;
    let number_array: Vec<Vec<(usize, i64)>> = get_number_array(&input);
    let symbol_array: Vec<(usize, usize)> = get_symbol_array(&input);
    // let symbol_set: HashSet<(usize, usize)> = symbol_array.into_iter().collect();

    for (y, line) in number_array.iter().enumerate() {
        for &(x, number) in line {
            let number_length = number.to_string().len();
            let mut is_added = false;
            for nx in x..x + number_length {
                for &(sx, sy) in &symbol_array {
                    if (sx as isize - nx as isize).abs() <= 1
                        && (sy as isize - y as isize).abs() <= 1
                    {
                        if !is_added {
                            sum += number;
                            is_added = true;
                        }
                        break;
                    }
                }
                if is_added {
                    break; // Break out of the nx loop once the number is counted
                }
            }
        }
    }

    return sum as u64;
}

fn part_2_solution(input: &str) -> u64 {
    return 0;
}
