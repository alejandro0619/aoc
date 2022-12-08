use std::fs;
use std::cmp::Ordering;

fn part_one() -> i32 {
    let input = fs::read_to_string("input.txt").expect("Cannot find the file");
    let lines = input.lines();

    // we set their discriminant
    #[derive(Debug, PartialEq)]
    enum Shapes {
        Rock = 1,
        Paper = 2,
        Scissors = 3,
    }

    impl PartialOrd for Shapes {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            // We only compare Rock an Scissors manually
            // Because Rock > Scissors and viceversa Scissors < Rock
            // So if we compare it within its discriminant
            // Rock will always be less than Scissors
            // Because 1 < 3
            // So we need to write that comparition manually
            // For the rest of possible option
            // We can compare within the discriminant:
            // 1 < 2 Paper beats Rock
            // 2 < 3 Scissors beats Paper

            if *self == Shapes::Rock && *other == Shapes::Scissors {
                Some(Ordering::Greater)
            } else if *self == Shapes::Scissors && *other == Shapes::Rock {
                Some(Ordering::Less)
            } else {
                Some((*self as i32).cmp(&(*other as i32)))
            }
        }
    }

    lines
        .map(|item| {
            item.split_whitespace()
                .map(|shape| -> Shapes {
                    match shape {
                        "A" | "X" => Shapes::Rock,
                        "B" | "Y" => Shapes::Paper,
                        "C" | "Z" => Shapes::Scissors,
                        _ => unreachable!("Invalid shape"),
                    }
                })
                .collect::<Vec<Shapes>>()
        })
        .fold(0, |acc, mov| {
            if mov[0] > mov [1] {
                acc + mov [1] as i32 + 0
            } else if mov [0] < mov [1] {
                acc + mov [1] as i32 + 6
            } else {
                acc + mov [1] as i32 + 3
            }
        })

}
fn main() {
    println!("{:?}", part_one());
}
