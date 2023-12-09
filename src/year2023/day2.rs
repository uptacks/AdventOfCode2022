use std::ops::Mul;

use crate::AdventSolution;
use itertools::{rev, Itertools};

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
    let mut total: u64 = 0;
    let games = parse_input(&input);
    for (id, game) in games.iter().enumerate() {
        if check_valid(game) {
            total += (id as u64 + 1);
        }
    }

    return total;
}

fn part_2_solution(input: &str) -> u64 {
    let mut sum_power = 0;
    let mut min_game_collection = (0, 0, 0);

    let games = parse_input(&input);

    for (id, game) in games.iter().enumerate() {
        min_game_collection = get_min_game_collection(game);
        sum_power += min_game_collection.0 * min_game_collection.1 * min_game_collection.2
    }

    return sum_power as u64;
}

fn parse_input(input: &str) -> Vec<Vec<(i32, i32, i32)>> {
    let mut games: Vec<Vec<(i32, i32, i32)>> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        if parts.len() < 2 {
            continue;
        }
        let sets = parts[1]
            .trim()
            .split(";")
            .map(|set| {
                let mut red = 0;
                let mut blue = 0;
                let mut green = 0;
                for colour_count in set.split(',') {
                    let parts: Vec<&str> = colour_count.trim().split_whitespace().collect();
                    let count = parts[0].parse().unwrap();
                    match parts[1] {
                        "red" => red = count,
                        "blue" => blue = count,
                        "green" => green = count,
                        _ => {}
                    }
                }
                (red, green, blue)
            })
            .collect();
        games.push(sets)
    }
    return games;
}

fn check_valid(game: &Vec<(i32, i32, i32)>) -> bool {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    for (red, green, blue) in game {
        if red > &max_red || blue > &max_blue || green > &max_green {
            return false;
        }
    }
    return true;
}

fn get_min_game_collection(game: &Vec<(i32, i32, i32)>) -> (i32, i32, i32) {
    let mut largest_red = 0;
    let mut largest_green = 0;
    let mut largest_blue = 0;

    for (red, green, blue) in game {
        if red > &largest_red {
            largest_red = *red
        };
        if blue > &largest_blue {
            largest_blue = *blue
        };
        if green > &largest_green {
            largest_green = *green
        };
    }

    return (largest_red, largest_green, largest_blue);
}
