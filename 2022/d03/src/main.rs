use std::{fs, collections::HashMap};

fn part_one(input: String) -> () {
    let lines = input.lines();

    let letters_scores = ('a'..='z')
    .chain('A'..='Z')
    .enumerate()
    .map(|(i, l)|{
        (l, i + 1)
    })
    .collect::<HashMap<char, usize>>();

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Compartments {
        first: String,
        second: String,
    }

    let score = lines
        .map(|rucksack| {
            let sack_length = rucksack.len() / 2;

            Compartments {
                first: rucksack[..sack_length].to_string(), // First compartment
                second: rucksack[sack_length..].to_string(), // Second compartment
            }
        })
        .map(|item| -> i32 {
            let common = item
            .first
            .chars()
            .find(|c| item.second.contains(*c)).unwrap();

            *letters_scores.get(&common).unwrap() as i32
        })
        .sum::<i32>();
    println!("{}", score);
    
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Can't find the file");
    part_one(input);
}
