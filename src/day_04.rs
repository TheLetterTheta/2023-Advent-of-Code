use aoc_runner_derive::{aoc, aoc_generator};

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Game {
    winning_nums: HashSet<u32>,
    actual_nums: HashSet<u32>,
    copies: usize,
}

type Line = Game;
type Input = Vec<Line>;

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, _id) = delimited(
        tuple((tag("Card"), space1)),
        complete::u32,
        tuple((tag(":"), space1)),
    )(input)?;
    let (input, (wins, actual)) = separated_pair(
        separated_list1(space1, complete::u32),
        tuple((space1, tag("|"), space1)),
        separated_list1(space1, complete::u32),
    )(input)?;

    Ok((
        input,
        Game {
            winning_nums: wins.into_iter().collect(),
            actual_nums: actual.into_iter().collect(),
            copies: 1,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, list) = separated_list1(line_ending, parse_line)(input)?;

    Ok((input, list.into_iter().collect()))
}

#[aoc_generator(day4)]
fn day4_generator(input: &str) -> Input {
    let _input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let (input, output) = parse_input(input).expect("Could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day4, part1)]
fn solve_part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|card| card.actual_nums.intersection(&card.winning_nums).count())
        .map(|n| 2_u32.pow(n as u32 - 1))
        .sum()
}

#[aoc(day4, part2)]
fn solve_part2(input: &Input) -> usize {
    let mut input = input.clone();

    for i in 0..input.len() {
        let next_few = input[i]
            .actual_nums
            .intersection(&input[i].winning_nums)
            .count();
        for j in i + 1..(i + 1 + next_few) {
            input[j].copies += input[i].copies;
        }
    }

    input.iter().map(|i| i.copies).sum::<usize>()
}
