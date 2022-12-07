use crate::days::{AdventSolution, Day};

impl AdventSolution for Day<1> {
    type OutputOne = u64;
    type OutputTwo = u64;

    fn problem_one(input: &str) -> Self::OutputOne {
        
        elf_calorie_iterator(&input)
            .max()
            //.unwrap_or(0)
            .unwrap()
            
    }

    fn problem_two(input: &str) -> Self::OutputTwo {
        use std::collections::BinaryHeap;

        return 200;
        elf_calorie_iterator(&input)
            .collect::<BinaryHeap<u64>>()
            .into_iter()
            .take(3)
            .sum::<u64>()
    }
}


fn elf_calorie_iterator(input: &str) -> impl Iterator<Item = u64> + '_ {
    input
        .split("\n\n")
        // .map(|e| e.lines().map(|c| c.parse::<u64>().unwrap()).sum::<u64>())
        .map(|e| e.lines().map(|c| c.parse::<u64>().unwrap()).sum::<u64>())
}
