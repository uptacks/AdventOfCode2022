// mod day1;

// use std::fmt::Display;

// pub struct Day<const NUM: u64>;

// pub trait AdventDay {
//     const NUM: u64;
//     fn get_day_name() -> String;
// }

// impl<const NUM: u64> AdventDay for Day<NUM> {
//     const NUM: u64 = NUM;

//     fn get_day_name() -> String {
//         format!("Day {}", NUM)
//     }
// }

// pub trait AdventSolution: AdventDay {
//     type OutputOne: Display;
//     type OutputTwo: Display;

//     fn problem_one(input: &str) -> Self::OutputOne;
//     fn problem_two(input: &str) -> Self::OutputTwo;

//     fn run() {
//         let path = format!("input/day{}.txt", Self::NUM);

//         let input = std::fs::read_to_string(path).expect("unable to open file");

//         println!("Solution for {}", Self::get_day_name());
//         println!("\tProblem one: {}", Self::problem_one(&input));
//         println!("\tProblem two: {}\n", Self::problem_two(&input));
//     }
// }

mod day1;
mod day2;

pub use day1::Day1;
pub use day2::Day2;
