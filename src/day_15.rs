use rayon::iter::ParallelIterator;
use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use core::slice::Iter;
use rayon::prelude::ParallelBridge;

use itertools::{repeat_n, Itertools};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{self, alpha1, line_ending, space1},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated, tuple},
    IResult, Parser,
};

type Input = Vec<String>;

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, output) =
        separated_list1(tag(","), is_not(",").map(|v: &str| v.to_string()))(input)?;

    Ok((input, output))
}

#[aoc_generator(day15)]
fn day15_generator(input: &str) -> Input {
    let _input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    let (input, output) = parse_input.parse(input).expect("could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day15, part1)]
fn solve_part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|line| {
            line.chars()
                .fold(0, |acc, c| (17 * (acc + (c as u32))) % 256)
        })
        .sum()
}

#[aoc(day15, part2)]
fn solve_part2(input: &Input) -> usize {
    #[derive(Debug)]
    enum Operation {
        Set(String, u8),
        Remove(String),
    }

    let mut boxes: Vec<Vec<(String, u8)>> = vec![vec![]; 256];
    input.iter().for_each(|line| {
        let (_, operation) = alt((
            separated_pair(alpha1::<&str, ()>, tag("="), complete::u8)
                .map(|(string, val): (&str, u8)| Operation::Set(string.to_string(), val)),
            terminated(alpha1, tag("-")).map(|string: &str| Operation::Remove(string.to_string())),
        ))
        .parse(line.as_str())
        .expect("invalid input format");

        match operation {
            Operation::Set(ref k, v) => {
                let box_index =
                    k.chars().fold(0, |acc, c| (17 * (acc + (c as u32))) % 256) as usize;
                if let Some(exists) = boxes[box_index].iter_mut().find(|v| v.0 == *k){
                    exists.1 = v;
                } else {
                boxes[box_index].push((k.to_string(), v));
                }
            }
            Operation::Remove(ref k) => {
                let box_index =
                    k.chars().fold(0, |acc, c| (17 * (acc + (c as u32))) % 256) as usize;
                boxes[box_index].retain(|v| v.0 != *k);
            }
        }
    });
    boxes
        .iter()
        .enumerate()
        .filter(|(_, v)| !v.is_empty())
        .flat_map(|(b, values)| {
            values
                .iter()
                .enumerate()
                .map(move |(index, (_key, value))| (1+b) * (1+index) * (*value as usize))
        })
        .sum()
}
