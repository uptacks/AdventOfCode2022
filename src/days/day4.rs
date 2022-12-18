use itertools::Itertools;

use crate::days::{AdventSolution, Day};

impl AdventSolution for Day<4> {
    type OutputOne = u64;
    type OutputTwo = u64;

    fn problem_one(input: &str) -> Self::OutputOne {
        let pairs = get_assignment_pairs(input);
        let mut count = 0;
        for pair in pairs {
            if(fully_contained(pair[0], pair[1])) {
                count = count + 1;
            }
        }
        return count;
        }
    

    fn problem_two(input: &str) -> Self::OutputTwo {
        let pairs = get_assignment_pairs(input);
        let mut count = 0;
        for pair in pairs {
            if(part_contained(pair[0], pair[1])) {
                count = count + 1;
            }
        }
        return count;
        }

    }


fn part_contained(first: &str, second: &str) -> bool {    
    let mut first_iter = first.split('-');
    let mut second_iter = second.split('-');

    let first_start = first_iter.nth(0).unwrap().parse::<u64>().unwrap();
    let first_end = first_iter.nth(0).unwrap().parse::<u64>().unwrap();
    let second_start = second_iter.nth(0).unwrap().parse::<u64>().unwrap();
    let second_end = second_iter.nth(0).unwrap().parse::<u64>().unwrap();

    if (first_start <= second_start) && (first_end >= second_start) {
        return true;
    } else if (first_start <= second_end) && (first_end >= second_end) {
        return true;
    } else if (first_start >= second_start) && (first_end <= second_end) {
        return true;
    }
    return false;
}

fn fully_contained(first: &str, second: &str) -> bool {    
    let mut first_iter = first.split('-');
    let mut second_iter = second.split('-');

    let first_start = first_iter.nth(0).unwrap().parse::<u64>().unwrap();
    let first_end = first_iter.nth(0).unwrap().parse::<u64>().unwrap();
    let second_start = second_iter.nth(0).unwrap().parse::<u64>().unwrap();
    let second_end = second_iter.nth(0).unwrap().parse::<u64>().unwrap();

    if (first_start <= second_start) && (first_end >= second_end) {
        return true;
    } else if (first_start >= second_start) && (first_end <= second_end) {
        return true;
    }
    return false;
}

fn get_assignment_pairs(input: &str) -> Vec<Vec<&str>> {
    let string: Vec<Vec<&str>> = input
    .split("\r\n")
    .map(|line| line.split(',').collect())
    .collect()
    ;

    return string;

    // input
    //     .lines()
    //     .map(|pair| {
    //         let mut split = pair.split(',')
    //             .map(|s| s.split('-').collect::<Vec<_>>());

    //         (split.next().collect(), split.next().collect())
    //     })
    //     .collect()
}
