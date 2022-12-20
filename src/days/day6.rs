use itertools::Itertools;

use crate::days::{AdventSolution, Day};

impl AdventSolution for Day<6> {
    type OutputOne = u64;
    type OutputTwo = u64;

    fn problem_one(input: &str) -> Self::OutputOne {
        let binding = input.chars().collect_vec();
        let mut window = binding.windows(4);
        let mut count = 4;

        // Loop through windows
        while let Some(w) = window.next() {
            // Check if all characters in the window are unique
            if w.iter().unique().count() == 4 {
                // If so, return the first character
                return count;
            }
            count += 1;
        }
    return 0;
    }

    fn problem_two(input: &str) -> Self::OutputTwo {
        let binding = input.chars().collect_vec();
        let mut window = binding.windows(14);
        let mut count = 14;

        // Loop through windows
        while let Some(w) = window.next() {
            // Check if all characters in the window are unique
            if w.iter().unique().count() == 14 {
                // If so, return the first character
                return count;
            }
            count += 1;
        }
    return 0;
    
    
        return 0;

    }

}
