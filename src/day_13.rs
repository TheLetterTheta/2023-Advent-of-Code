use rayon::iter::ParallelIterator;
use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use core::slice::Iter;
use rayon::prelude::ParallelBridge;

use itertools::{repeat_n, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult, Parser,
};

#[derive(Debug)]
struct Grid {
    rows: Vec<String>,
    columns: Vec<String>,
}

impl Grid {
    fn find_on_row(&self) -> usize {
        let potential_middles = self
            .rows
            .iter()
            .enumerate()
            .tuple_windows()
            .filter(|((_, left), (_, right))| left == right);
        dbg!(potential_middles.collect_vec());

        todo!()
    }
}

type Input = Vec<Grid>;

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    let (input, rows) = separated_list1(
        line_ending,
        many1(alt((complete::char('#'), complete::char('.')))),
    )(input)?;

    let columns = (0..rows[0].len())
        .map(|i| rows.iter().map(|row| row[i]).collect())
        .collect();

    Ok((
        input,
        Grid {
            rows: rows.iter().map(|v| v.iter().collect()).collect(),
            columns,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    separated_list1(tuple((line_ending, line_ending)), parse_grid)(input)
}

#[aoc_generator(day13)]
fn day13_generator(input: &str) -> Input {
    let _input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    let (input, output) = parse_input.parse(_input).expect("could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day13, part1)]
fn solve_part1(input: &Input) -> usize {
    let _ = input.iter().map(|v| v.find_on_row()).collect_vec();
    todo!()
}

#[aoc(day13, part2)]
fn solve_part2(input: &Input) -> usize {
    todo!()
}
