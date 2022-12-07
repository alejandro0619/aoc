use std::fs;

fn part_one() -> u32 {
    let input = fs::read_to_string("input.txt").expect("Cannot find the file");

    //println!("text: \n {}", input);

    input
        .split("\n\n")
        .map(|line| {
            line.lines()
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

fn part_two() {
    let input = fs::read_to_string("input.txt").expect("Cannot find the file");

    //println!("text: \n {}", input);

    let mut input = input
        .split("\n\n")
        .map(|line| {
            line.lines()
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    input.sort_by(|a, b| b.cmp(a));
    let sum = input.iter().take(3).sum::<u32>();
    println!("{}", sum);
}
fn main() {
    part_one();
    part_two();
}

