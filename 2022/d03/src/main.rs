#![feature(iter_array_chunks)]

use std::{collections::HashMap, fs};

fn part_one(input: &String) -> () {
    let lines = input.lines();

    let letters_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, l)| (l, i + 1))
        .collect::<HashMap<char, usize>>();

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
                .find(|c| item.second.contains(*c))
                .unwrap();

            *letters_scores.get(&common).unwrap() as i32
        })
        .sum::<i32>();
    println!("{}", score);
}
fn part_two(input: &String) {
    let letters_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, l)| (l, i + 1))
        .collect::<HashMap<char, usize>>();
    #[allow(dead_code)]
    #[derive(Debug)]
    struct RucksackGroup {
        first: String,
        second: String,
        third: String,
    }
    let total_score = input
        .lines()
        .array_chunks::<3>()
        .map(|[f, s, t]| RucksackGroup {
            first: f.to_string(),
            second: s.to_string(),
            third: t.to_string(),
        })
        .map(|item| {
            let common = item
                .first
                .chars()
                .find(|c| item.second.contains(*c) && item.third.contains(*c))
                .unwrap();

            *letters_scores.get(&common).unwrap() as i32
        })
        .sum::<i32>();

    println!("{}", total_score)
}
fn main() {
    let input = fs::read_to_string("input.txt").expect("Can't find the file");
    part_one(&input);

    part_two(&input);
}
