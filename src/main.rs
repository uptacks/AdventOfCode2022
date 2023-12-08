#![allow(incomplete_features)]
// #![feature(iter_array_chunks)]
// #![feature(generic_const_exprs)]
// #![feature(iter_advance_by)]

// use std::io;

// mod year2022;
// mod year2023;

// macro_rules! run_aoc_days {
//     ($year:literal, $day:literal) => {
//         match $year {
//             2022 => year2022::Day::<$day>::run(),
//             2023 => year2023::Day<day>::run(),
//             _ => println!("Year not supported"),
//         }
//     };
//     ($year:literal, $day:literal, $($days:literal),+) => {
//         run_aoc_days!($year, $day);
//         run_aoc_days!($year, $($days),+);
//     }
// }

// fn main() {
//     println!("Enter which year you'd like to run: ");

//     let mut input_text = String::new();
//     io::stdin()
//         .read_line(&mut input_text)
//         .expect("failed to read from stdin");

//     let year = input_text.trim().parse::<u32>().unwrap_or(0);

//     match year {
//         2023 => run_aoc_days!(2023, 1), // Modify the days as needed
//         2022 => run_aoc_days!(2022, 1, 2, 3, 4, 5, 6, 7), // Modify the days as needed
//         _ => println!("Year not supported"),
//     }
// }

use std::io;
use AdventOfCode::{year2022, year2023, AdventSolution};

fn main() {
    println!("Enter the year you'd like to run (e.g., 2022):");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read from stdin");
    let year = input_text.trim();

    match year {
        "2022" => run_year_2022::<
            year2022::Day1,
            year2022::Day2,
            year2022::Day3,
            year2022::Day4,
            year2022::Day5,
            year2022::Day6,
        >(), // Add other days as needed
        "2023" => run_year_2023::<year2023::Day1, year2023::Day2>(), // And so on for other years
        _ => println!("Year not supported"),
    }
}

fn run_year_2022<
    D1: AdventSolution,
    D2: AdventSolution,
    D3: AdventSolution,
    D4: AdventSolution,
    D5: AdventSolution,
    D6: AdventSolution,
>() {
    println!("Solutions for Day 1: {:?}", D1::solve());
    println!("Solutions for Day 2: {:?}", D2::solve());
    println!("Solutions for Day 3: {:?}", D3::solve());
    println!("Solutions for Day 4: {:?}", D4::solve());
    println!("Solutions for Day 5: {:?}", D5::solve());
    println!("Solutions for Day 6: {:?}", D6::solve());

    // Add more days as needed
}

fn run_year_2023<D1: AdventSolution, D2: AdventSolution>() {
    println!("Solutions for Day 1: {:?}", D1::solve());
    println!("Solutions for Day 2: {:?}", D2::solve());
}
