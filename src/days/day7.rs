#![allow(dead_code)]

use itertools::Itertools;
use crate::days::{AdventSolution, Day};

#[derive(Debug)]
struct directory {
    name: String,
    size: u64,
    parent : String,
    files: Option<Vec<file>>,
    subdirectories: Option<Vec<directory>>
}

impl directory {
    fn new_root() -> directory {
        return directory { name:'/'.to_string(), size: 0, parent: "/".to_string(), files: Some(Vec::new()), subdirectories: Some(Vec::new()) };
    }

    fn new(input_name : String, input_size : u64, parent: String, input_files : Option<Vec<file>>,input_subdirectory : Option<Vec<directory>>) -> directory {
        return directory { name: input_name, size: input_size, parent: parent, files: input_files, subdirectories: input_subdirectory }
    }
}

#[derive(Debug)]
struct file {
    name: String,
    size: u64,
}

impl file {
    fn new(input_name: String, input_size: u64) -> file {
        return file { name: input_name, size: input_size }
    }
    
}

impl AdventSolution for Day<7> {
    type OutputOne = u64;
    type OutputTwo = u64;

    fn problem_one(input: &str) -> Self::OutputOne {
        let filesystem = parse_filesystem(input);
    return 0;
    }

    fn problem_two(input: &str) -> Self::OutputTwo {
    return 0;

    }

}

fn parse_filesystem(input : &str) -> directory {
    let mut filesystem = directory::new_root();

    let mut depth = 0;
    let mut cur_dir = "/".to_owned();

    for line in input.lines() {
        if line.chars().nth(0).unwrap() == '$' {
            let mut command = line.split(" ").collect_vec();
            match (&command[1]) {
                cd => {
                    match (&command[2]) {
                        &".." => { depth -= 1; cur_dir = },
                        &"/" => {depth = 0; cur_dir = "/".to_owned()},
                        _ => {}
                    }
                },
                ls => {
                    continue;
                },
                _ => {
                    continue;
                }
            }
        }

    }

    println!("{:?}", filesystem);

    return directory::new_root();
}
