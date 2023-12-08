#![allow(dead_code)]

use itertools::Itertools;

use crate::read_input_for_day;
use crate::AdventSolution;

pub struct Day7;

impl AdventSolution for Day7 {
    const DAY: u32 = 7;
    const YEAR: u32 = 2022;
    fn solve() -> (String, String) {
        let input = read_input_for_day(Self::DAY, Self::YEAR);
        let part_1_solution = problem_one(&input).to_string();
        let part_2_solution = problem_two(&input).to_string();
        (part_1_solution, part_2_solution)
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    size: u64,
    parent: String,
    Files: Option<Vec<File>>,
    subdirectories: Option<Vec<Directory>>,
}

impl Directory {
    fn new_root() -> Directory {
        return Directory {
            name: '/'.to_string(),
            size: 0,
            parent: "/".to_string(),
            Files: Some(Vec::new()),
            subdirectories: Some(Vec::new()),
        };
    }

    fn new(
        input_name: String,
        input_size: u64,
        parent: String,
        input_Files: Option<Vec<File>>,
        input_subDirectory: Option<Vec<Directory>>,
    ) -> Directory {
        return Directory {
            name: input_name,
            size: input_size,
            parent: parent,
            Files: input_Files,
            subdirectories: input_subDirectory,
        };
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

impl File {
    fn new(input_name: String, input_size: u64) -> File {
        return File {
            name: input_name,
            size: input_size,
        };
    }
}

fn problem_one(input: &str) -> u64 {
    let filesystem = parse_filesystem(input);
    return 0;
}

fn problem_two(input: &str) -> u64 {
    return 0;
}

fn parse_filesystem(input: &str) -> Directory {
    let mut filesystem = Directory::new_root();

    let mut depth = 0;
    let mut cur_dir = "/".to_owned();

    for line in input.lines() {
        if line.chars().nth(0).unwrap() == '$' {
            let mut command = line.split(" ").collect_vec();
            match &command[0] {
                cd => {
                    match &command[1] {
                        &".." => {
                            depth -= 1;
                            cur_dir = "".to_string()
                        } // Wrong but just to complete
                        &"/" => {
                            depth = 0;
                            cur_dir = "/".to_owned()
                        }
                        _ => {}
                    }
                }
                ls => {
                    continue;
                }
                _ => {
                    continue;
                }
            }
        }
    }

    println!("{:?}", filesystem);

    return Directory::new_root();
}
