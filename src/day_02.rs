use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, line_ending, one_of},
    combinator::opt,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult, Parser,
};
use std::collections::{HashMap, HashSet};

type Input = HashMap<u32, Vec<Vec<(u32, String)>>>;

fn parse_line(input: &str) -> IResult<&str, (u32, Vec<Vec<(u32, String)>>)> {
    tuple((
        preceded(tag("Game "), complete::u32),
        preceded(
            tag(": "),
            separated_list1(
                tag("; "),
                separated_list1(
                    tag(", "),
                    tuple((
                        complete::u32,
                        preceded(
                            tag(" "),
                            alt((tag("blue"), tag("red"), tag("green")))
                                .map(|f: &str| f.to_string()),
                        ),
                    )),
                ),
            ),
        ),
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, list) = separated_list1(line_ending, parse_line)(input)?;

    Ok((input, list.into_iter().collect()))
}

#[aoc_generator(day2)]
pub fn day2_generator(input: &str) -> Input {
    let _input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let (input, output) = parse_input(input).unwrap();
    assert!(input.is_empty());
    output
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Input) -> u32 {
    input
        .iter()
        .filter_map(|(k, v)| {
            if v.iter().all(|pull| {
                pull.into_iter()
                    .all(|(number, color)| match color.as_str() {
                        "green" => number <= &13,
                        "red" => number <= &12,
                        "blue" => number <= &14,
                        _ => unreachable!(),
                    })
            }) {
                Some(k)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Input) -> u32 {
    input
        .values()
        .map(|v| {
            let (max_green, max_red, max_blue) = v.iter().fold((0, 0, 0), |mut acc, pull| {
                pull.iter()
                    .for_each(|(number, color)| match color.as_str() {
                        "green" => {
                            if number > &acc.0 {
                                acc.0 = *number;
                            }
                        }
                        "red" => {
                            if number > &acc.1 {
                                acc.1 = *number
                            }
                        }
                        "blue" => {
                            if number > &acc.2 {
                                acc.2 = *number
                            }
                        }
                        _ => unreachable!(),
                    });
                acc
            });
            max_green * max_red * max_blue
        })
        .sum()
}
