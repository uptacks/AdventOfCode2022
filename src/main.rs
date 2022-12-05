use std::fs::File;
use std::io::{BufRead, BufReader};
use std::borrow::ToOwned;

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

    #[derive(Debug)]
    enum Shape {
        Rock,
        Paper,
        Scissors,
    }
    
    // Define a function that maps a character to a Shape
    fn char_to_shape(c : char) -> Shape {
        match c {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => panic!("Invalid shape"),
        }
    }

    fn round_score(your_shape: &Shape, opponent_shape: &Shape) -> i32 {
        // Convert the shapes to integers for easier comparison
        let your_shape = match your_shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };
    
        let opponent_shape = match opponent_shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };
    
        // Calculate the score for the round
        if your_shape == opponent_shape {
            // Draw
            return 3 + your_shape;
        } else if your_shape == opponent_shape + 1 || (your_shape == 1 && opponent_shape == 3) {
            // You win
            return 6 + your_shape;
        } else {
            // You lose
            return 0 + your_shape;
        }
    }

fn game_score(filename: &str) -> i32 {
    // Create a map of shapes to responses


    let mut rounds = vec![];

    let file = File::open(filename).unwrap();
    for line in BufReader::new(file).lines() {
        let mut liney = line.unwrap();

        

        liney = liney.trim().to_owned();

        
        if !liney.is_empty() {
            liney = liney
                .split_whitespace()
                .collect();
            rounds.push(liney);
        }
    }



    // Calculate the score for each round
    let mut score = 0;
    for round in rounds {
        let opponent_shape = &char_to_shape(round.chars().nth(0).unwrap());
        let your_shape = choose_shape(opponent_shape,round.chars().nth(1).unwrap());
        score += round_score(&your_shape, opponent_shape);
    }
    
        // Return the total score
        score
    }
    
fn choose_shape(opponent_shape: &Shape, outcome: char) -> Shape {
    match outcome {
        'X' => match opponent_shape {
            Shape::Rock => char_to_shape('C'),
            Shape::Paper => char_to_shape('A'),
            Shape::Scissors => char_to_shape('B'),
            },
            'Y' => match opponent_shape {
                Shape::Rock => char_to_shape('A'),
                Shape::Paper => char_to_shape('B'),
                Shape::Scissors => char_to_shape('C'),
            },
            'Z' => match opponent_shape {
                Shape::Rock => char_to_shape('B'),
                Shape::Paper => char_to_shape('C'),
                Shape::Scissors => char_to_shape('A'),
            },
            _ => panic!("Invalid outcome"),
        }

    }
    
    
fn main() {
    let mut filename = "../input/day1.txt";

    let calories_per_elf = calories_per_elf(filename);


    // Day 1 Part 1
    let mut max_calories = 0;
    let mut best_elf = 0;
    for (i, calories) in calories_per_elf.iter().enumerate() {
        if(max_calories < *calories){
            max_calories = *calories;
            best_elf = i;
        }
    }

    print!("The best elf is: {}", best_elf);
    print!(" with {} calories \n", max_calories);

    // Day 2 Part 2

    let mut elves = calories_per_elf.clone();
    elves.sort_by(|a, b| b.cmp(a));

    let mut sum = 0;
    for i in 0..3 {
        sum = sum + elves[i];
    }

    println!("The sum of the three best elves is: {}", sum);

    // Day 2 Part 1 
    // Calculate the total score for the game
    filename = "../input/day2.txt";
    let score = game_score(filename);
    println!("The total score for the game is: {}", score);

    // Day 2 Part 2


}