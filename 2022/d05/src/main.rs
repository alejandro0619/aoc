use std::fs;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    sequence::{delimited, preceded},
    character::complete::{
        char,
        newline, 
        multispace1,
        digit1,
        self,
    },
    multi::{separated_list1, many1},
    *,   
};

// Parse each crate: [Letter]:
fn parse_crates(input: &str) -> IResult<&str, Option<&str>> {
    let (input, crates) = alt((
        tag("   "),
        delimited(
            char('['),
            take(1u16),
            char(']'),
        )
    ))(input)?;

    let result = match crates {
        "   " => None,
        value => Some(value),
    };
    Ok((input, result))
}
// Parses the whole line
fn parse_lines(input: &str) -> IResult<&str, Vec<Option<&str>>>{
    let (input, result) = separated_list1(tag(" "), parse_crates)(input)?;

    Ok((input, result))
}
#[derive(Debug)]
struct Moves {
    number_crates: u32,
    from: u32,
    to: u32
}

fn parse_moves(input: &str) -> IResult<&str, Moves> {
    let (input, _) = tag("move ")(input)?;
    let (input, number_crates) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;

    Ok((input, Moves {
        number_crates,
        from,
        to,
    }))
}
// Parses the stack: Horizontally
fn parse_stack(input: &str) -> IResult<&str, Vec<Vec<Option<&str>>>>{
    let (input, hranges) = separated_list1(newline, parse_lines)(input)?;
    let (input, _) = many1(preceded(multispace1, digit1))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _moves) = separated_list1(newline, parse_moves)(input)?;
    let mut vranges: Vec<Vec<_>> = Vec::new(); // Vertical crates

    hranges.iter().for_each(|_| vranges.push(vec![])); // Push empty vectors
    // Black magic I don't even understand
    hranges.iter().rev().for_each(|v| {
        v.iter().enumerate().for_each(|(i, c)| {
            vranges[i].push(*c)
        })
    });

    Ok((input, vranges))
}
fn part_one(input: &String) -> IResult<&str, Vec<Vec<Option<&str>>>>{
    parse_stack(input)
}

fn part_two(input: &String){

}
fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not find the file");
    let (_, result) = part_one(&input).unwrap();

    println!("{:?}", result);
}
