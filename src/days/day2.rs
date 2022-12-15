use itertools::Itertools;

use crate::days::{AdventSolution, Day};

impl AdventSolution for Day<2> {
    type OutputOne = u64;
    type OutputTwo = u64;

    fn problem_one(input: &str) -> Self::OutputOne {
        let mut score = 0;

        for i in game_vec(&input) {
            match (i[1]) {
                88 => score += 1,
                89 => score += 2,
                90 => score += 3,
                _ => score += 0,
            }

            match(winning_son(i[0], i[1])) {
                1 => score += 6,
                2 => score += 3,
                3 => score += 0,
                _ => score += 0,
            }
        }
        
        return score;
    }

    fn problem_two(input: &str) -> Self::OutputTwo {

        let mut score = 0;
        for i in game_vec(&input) {
            match (i[1]) {
                88 => match(i[0]) {
                    65 => score = score + 0 + 3,
                    66 => score = score + 0 + 1,
                    67 => score = score + 0 + 2,
                    _ => score = score + 0 + 0,
                },
                89 => match(i[0]) {
                    65 => score = score + 3 + 1,
                    66 => score = score + 3 + 2,
                    67 => score = score + 3 + 3,
                    _ => score = score + 0 + 0,
                },
                90 => match(i[0]) {
                    65 => score = score + 6 + 2,
                    66 => score = score + 6 + 3,
                    67 => score = score + 6 + 1,
                    _ => score = score + 0 + 0,
                },

                _ => score = score + 0 + 0,
            }

        }

        return score;

    }


}

pub fn winning_son(a: u64, b: u64) -> u64 {
    match(a) {
        65 => match(b) {
            88 => 2,
            89 => 1,
            90 => 3,
            _ => 0,
        },
        66 => match(b) {
            88 => 3,
            89 => 2,
            90 => 1,
            _ => 0,
        },
        67 => match(b) {
            88 => 1,
            89 => 3,
            90 => 2,
            _ => 0,
        },
        _ => 0,
    }
}

pub fn game_vec(input: &str) -> Vec<Vec<u64>> {

    let game : Vec<Vec<u64>> = input
        //.split("\r\n")
        //.collect_vec();
        .split("\r\n")
        .map(|game_collection| {
            game_collection
                .split(' ')
                .map(|c| c.chars().nth(0).unwrap() as u64)
                .collect_vec()
                
        })
        .collect::<Vec<Vec<u64>>>();
    

    return game;

}
