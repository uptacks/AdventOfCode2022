use std::fs;
use std::path::PathBuf;

pub mod year2022;
pub mod year2023;

pub trait AdventSolution {
    const DAY: u32;
    const YEAR: u32;
    fn solve() -> (String, String); // Returns solutions for part 1 and part 2
}

pub fn read_input_for_day(day: u32, year: u32) -> String {
    let path = PathBuf::from(format!("src/year{}/input/day{}.txt", year, day));
    fs::read_to_string(path).expect("Failed to read input file")
}
