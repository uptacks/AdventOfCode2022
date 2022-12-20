use itertools::Itertools;
use crate::days::{AdventSolution, Day};

// Create instructions structure
#[derive(Debug)]
struct Instructions {
    numberToMove: u64,
    start: u64,
    end: u64,
}

impl AdventSolution for Day<5> {
    type OutputOne = String;
    type OutputTwo = String;

    fn problem_one(input: &str) -> Self::OutputOne {
        let (mut crates, mut instructionList) = assign_to_stack(input);
        let mut crates = move_crates_9000(crates, instructionList);
        return crates.iter_mut().map(|x| x.pop().unwrap()).collect::<String>(); 
    }

    fn problem_two(input: &str) -> Self::OutputTwo {
        let (mut crates, mut instructionList) = assign_to_stack(input);
        let mut crates = move_crates_9001(crates, instructionList);
        return crates.iter_mut().map(|x| x.pop().unwrap()).collect::<String>(); 
    }

}

fn move_crates_9000(mut crates: Vec<Vec<char>>, mut instructionList: Vec<Instructions>) -> Vec<Vec<char>> {
    for instruction in instructionList {
        for i in 0..instruction.numberToMove {
            let mut temp = crates[(instruction.start - 1) as usize].pop().unwrap().to_owned();
            crates[(instruction.end - 1) as usize].push(temp);
        }
    }
    return crates;
}

fn move_crates_9001(mut crates: Vec<Vec<char>>, mut instructionList: Vec<Instructions>) -> Vec<Vec<char>> {
    for instruction in instructionList {
        let mut to_collect = crates[(instruction.start - 1) as usize].len() - instruction.numberToMove as usize;
        let mut temp: Vec<char> = crates[(instruction.start - 1) as usize].drain((to_collect..)).collect();
        crates[(instruction.end - 1) as usize].extend(temp);
    }
    return crates;
}

fn assign_to_stack(input: &str) -> (Vec<Vec<char>>, Vec<Instructions>) {
    let mut crates: Vec<Vec<char>> = Vec::new();

    let mut copy: Vec<Vec<String>> = Vec::new();

    let mut num_crates = 0;

    let mut intruction_index = 0;

    for line in input.lines() {
        let t = line.replace(|c:char| (c=='[' || c==']'), "");

        let mut u = t.split(" ").map(|s| s.to_string()).collect_vec();

        let mut index_to_remove = Vec::<u64>::new();
        let mut count = 0;
        for (index,i) in &mut u.iter_mut().enumerate() {
            if i == "" && count!= 3{
                count += 1;
                index_to_remove.push(index as u64);
                continue;
            }
            count = 0;
        }

        for i in index_to_remove.iter().rev() {
            u.remove(*i as usize);
        }


        copy.push(u);
    }

    for i in &copy {
        if i[0].parse::<u64>().unwrap_or(0) == 1u64 {
            num_crates = i[i.len()-1].parse().unwrap();
            break;
        }
    }

    for i in 0..num_crates {
        crates.push(Vec::new());
    }

    for (n,i) in copy.iter().enumerate() {
        if i[0].parse::<u64>().unwrap_or(0) == 1u64 {
            intruction_index = n+2;
            break;
        }
        for (crate_num, crate_item) in i.iter().enumerate() {
            if (crate_item.chars().nth(0).is_none()) {
                continue;
            }
            crates[crate_num].push(crate_item.chars().nth(0).unwrap());
        }
    }

    let mut instructionList: Vec<Instructions> = Vec::new();

    for i in &copy[intruction_index..] {
        let mut instruction = Instructions {
            numberToMove: 0,
            start: 0,
            end: 0,
        };
        instruction.numberToMove = i[1].parse().unwrap();
        instruction.start = i[3].parse().unwrap();
        instruction.end = i[5].parse().unwrap();
        instructionList.push(instruction);
    }

    // Reverse order to vectors in crates
    for i in &mut crates {
        i.reverse();
    }
    
    return (crates, instructionList);
    
}
