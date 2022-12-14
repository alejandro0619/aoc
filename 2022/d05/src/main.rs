use std::fs;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    sequence::delimited,
    character::complete::{char, newline},
    multi::separated_list1,
    *,   
};

// Here we create the parsing functions:
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
fn parse_lines(input: &str) -> IResult<&str, Vec<Option<&str>>>{
    let (input, result) = separated_list1(tag(" "), parse_crates)(input)?;

    Ok((input, result))
}
fn parse_stack(input: &str) -> IResult<&str, Vec<Vec<Option<&str>>>>{
    let (input, ranges) = separated_list1(newline, parse_lines)(input)?;

    Ok((input, ranges))
}
fn part_one(input: &String) -> IResult<&str, Vec<Vec<Option<&str>>>>{
    parse_stack(input)
}

fn part_two(input: &String){

}
fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not find the file");

    println!("{:#?}", part_one(&input))
}
