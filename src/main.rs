use std::fs::File;
use std::io::{BufRead, BufReader};

fn calories_per_elf(filename: &str) -> Vec<i32> {
    let mut elves = vec![];
    elves.push(0);
    let mut i = 0;

    let file = File::open(filename).unwrap();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        let line = line.trim();

        if !line.is_empty() {
            let line: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            elves[i] = elves[i] + line[0];
        } else {
            i = i+1;
            elves.push(0);
        }
    }

    elves}

fn main() {
    let filename = "../input/day1.txt";

    let calories_per_elf = calories_per_elf(filename);


    // Part 1
    println!("The total number of calories carried by each Elf is:");
    let mut max_calories = 0;
    let mut best_elf = 0;
    for (i, calories) in calories_per_elf.iter().enumerate() {
        println!("Elf {}: {} calories", i + 1, calories);
        if(max_calories < *calories){
            max_calories = *calories;
            best_elf = i;
        }
    }

    print!("The best elf is: {}", best_elf);
    print!(" with {} calories", max_calories);

    // Part 2

    let mut elves = calories_per_elf.clone();
    elves.sort_by(|a, b| b.cmp(a));

    let mut sum = 0;
    for i in 0..3 {
        sum = sum + elves[i];
    }

    println!("The sum of the three best elves is: {}", sum);

}