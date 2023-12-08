// mod day1;
// mod day2;
// mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;

// use std::fmt::Display;

// // Inside year2022/mod.rs

// #[macro_export]
// macro_rules! create_day {
//     ($day:literal, $problem_one:ident, $problem_two:ident) => {
//         mod $day {
//             pub fn problem_one(input: &str) -> String {
//                 // Default implementation or call a specific function
//                 super::$problem_one(input)
//             }

//             pub fn problem_two(input: &str) -> String {
//                 // Default implementation or call a specific function
//                 super::$problem_two(input)
//             }
//         }

//         impl AdventSolution for Day<$day> {
//             type OutputOne = String;
//             type OutputTwo = String;

//             fn problem_one(input: &str) -> Self::OutputOne {
//                 $day::problem_one(input)
//             }

//             fn problem_two(input: &str) -> Self::OutputTwo {
//                 $day::problem_two(input)
//             }
//         }
//     };
// }

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
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

pub use day1::Day1;
pub use day2::Day2;
pub use day3::Day3;
pub use day4::Day4;
pub use day5::Day5;
pub use day6::Day6;
pub use day7::Day7;
