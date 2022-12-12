use std::fs;

fn part_one(input: &String) -> u32 {
    input
        .lines()
        .map(|line: &str| {
            line.split(",")
                .map(|pairs| {
                    pairs
                        .split("-")
                        .map(|sections| sections.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .map(|item | {
                if (item[0][0] <= item[1][0] && item[0][1] >= item[1][1]) || item[0][0] >= item[1][0] && item[0][1] <= item[1][1] {
                    1
                } else {
                  0
                }
            },
        )
      .sum::<u32>()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not find the file");
    println!("{}", part_one(&input))
}

