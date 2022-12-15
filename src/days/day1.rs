use itertools::Itertools;

use crate::days::{AdventSolution, Day};

impl AdventSolution for Day<1> {
    type OutputOne = u64;
    type OutputTwo = u64;

    fn problem_one(input: &str) -> Self::OutputOne {
        
        return elf_calorie_vec(&input)
        .iter()
        .max()
        .unwrap_or(&0u64)
        .to_owned();

    }

    fn problem_two(input: &str) -> Self::OutputTwo {
        
        return elf_calorie_vec(&input)
        .iter()
        .sorted()
        .rev()
        .take(3)
        .sum::<u64>();

    }

}


// fn elf_calorie_iterator(input: &str) -> impl Iterator<Item = u64> + '_ {
fn elf_calorie_vec(input: &str) -> Vec<u64> {

    let elf : Vec<u64> = input
        .split("\r\n\r\n").map(|elf_collection| {
            elf_collection
                .lines()
                .map(|c| c.parse::<u64>().unwrap_or(0))
                .sum::<u64>()
        })
        .collect::<Vec<u64>>();
    
        return elf

        

    
        
        // .map(|elf_collection| {
        //     elf_collection
        //         .lines()
        //         .map(|c| c.parse::<u64>().unwrap_or(0))
        //         .sum::<u64>()
        // })
}
