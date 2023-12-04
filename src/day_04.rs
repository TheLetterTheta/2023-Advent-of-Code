use aoc_runner_derive::{aoc, aoc_generator};

use nom::{character::complete::line_ending, multi::separated_list1, IResult};

type Line = Vec<usize>;
type Input = Vec<Line>;

fn parse_line(_input: &str) -> IResult<&str, Line> {
    todo!();
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, list) = separated_list1(line_ending, parse_line)(input)?;

    Ok((input, list.into_iter().collect()))
}

#[aoc_generator(day4)]
pub fn day4_generator(input: &str) -> Input {
    let _input = "";

    let (input, output) = parse_input(input).expect("Could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day4, part1)]
pub fn solve_part1(_input: &Input) -> u32 {
    todo!();
}

#[aoc(day4, part2)]
pub fn solve_part2(_input: &Input) -> u32 {
    todo!();
}
