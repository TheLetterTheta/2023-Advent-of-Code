

use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, space1},
    multi::{separated_list1},
    IResult, Parser,
};

type Input = Vec<Vec<i32>>;

fn parse_input(input: &str) -> IResult<&str, Input> {
    separated_list1(line_ending, separated_list1(space1, complete::i32))(input)
}

#[aoc_generator(day9)]
fn day9_generator(input: &str) -> Input {
    let _input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    let (input, output) = parse_input(input).expect("Could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day9, part1)]
fn solve_part1(input: &Input) -> i32 {
    input
        .iter()
        .map(|n| {
            let mut find_derive = vec![*n.last().expect("empty input?")];
            let mut curr = n.clone();
            loop {
                curr = curr
                    .iter()
                    .tuple_windows()
                    .map(|(l, r)| r - l)
                    .collect_vec();
                let last = curr.last().expect("Ran out before derivative");

                find_derive.push(*last);

                if curr.iter().tuple_windows().all(|(l, r)| l == r) {
                    break;
                }
            }
            find_derive.iter().sum::<i32>()
        })
        .sum()
}

#[aoc(day9, part2)]
fn solve_part2(input: &Input) -> i32 {
    input
        .iter()
        .map(|n| {
            let mut find_derive = vec![*n.first().expect("empty input?")];
            let mut curr = n.clone();
            loop {
                curr = curr
                    .iter()
                    .tuple_windows()
                    .map(|(l, r)| r - l)
                    .collect_vec();
                let first = curr.first().expect("Ran out before derivative");

                find_derive.push(*first);

                if curr.iter().tuple_windows().all(|(l, r)| l == r) {
                    break;
                }
            }
            find_derive.iter().rev().fold(0, |acc, v| v - acc)
        })
        .sum()
}
