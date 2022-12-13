use std::cmp;
use std::fs;
use std::str::FromStr;

fn _part_one() -> i32 {
    let input = fs::read_to_string("input.txt").expect("Cannot find the file");
    let lines = input.lines();

    // we set their discriminant
    #[allow(dead_code)]
    #[derive(PartialEq, Clone, Copy)]
    enum Shapes {
        Rock = 1,
        Paper = 2,
        Scissors = 3,
    }

    impl PartialOrd for Shapes {
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
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
                Some(cmp::Ordering::Greater)
            } else if *self == Shapes::Scissors && *other == Shapes::Rock {
                Some(cmp::Ordering::Less)
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
            if mov[0] > mov[1] {
                acc + mov[1] as i32 + 0
            } else if mov[0] < mov[1] {
                acc + mov[1] as i32 + 6
            } else {
                acc + mov[1] as i32 + 3
            }
        })
}

fn part_two() {
    let input = fs::read_to_string("input.txt").expect("Cannot find the file");
    let lines = input.lines();

    #[derive(Debug)]
    enum Shapes {
        Rock = 1,
        Paper = 2,
        Scissors = 3,
        Win,
        Draw,
        Lose,
    }

    impl Shapes {
        fn win(&self) -> Self {
            match *self {
                Shapes::Rock => Shapes::Paper,
                Shapes::Paper => Shapes::Scissors,
                Shapes::Scissors => Shapes::Rock,
                _ => unreachable!(),
            }
        }

        fn lose(&self) -> Self {
            match *self {
                Shapes::Rock => Shapes::Scissors,
                Shapes::Paper => Shapes::Rock,
                Shapes::Scissors => Shapes::Paper,
                _ => unreachable!(),
            }
        }
        fn draw(&self) -> Self {
            match *self {
                Shapes::Rock => Shapes::Rock,
                Shapes::Paper => Shapes::Paper,
                Shapes::Scissors => Shapes::Scissors,
                _ => unreachable!(),
            }
        }
    }
    impl FromStr for Shapes {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" => Ok(Self::Rock),
                "B" => Ok(Self::Paper),
                "C" => Ok(Self::Scissors),
                "X" => Ok(Self::Lose),
                "Y" => Ok(Self::Draw),
                "Z" => Ok(Self::Win),
                _ => Err("Not a valid move"),
            }
        }
    }

    let result: i32 = lines
        .map(|item| {
            item.split_whitespace()
                .map(|movement| {
                        match movement {
                        "A" | "B" | "C" => movement.parse::<Shapes>().unwrap(),
                        "X" | "Y" | "Z" => movement.parse::<Shapes>().unwrap(),
                        _ => unreachable!(),
                    }
                })
                .collect::<Vec<Shapes>>()
        }).map(|item| {
            match item[1] {
                Shapes::Lose => item[0].lose() as i32 + 0,
                Shapes::Draw => item[0].draw() as i32 + 3,
                Shapes::Win =>  item[0].win() as i32 + 6,
                _ => unreachable!(),
            }
        }).sum();

        println!("{}", result);
}
fn main() {
    //println!("{:?}", part_one());
    println!("{:?}", part_two());
}
