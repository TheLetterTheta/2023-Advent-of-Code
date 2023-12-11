use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Copy, Clone)]
enum Sky {
    Empty,
    Galaxy,
}

type Input = Vec<Vec<Sky>>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Sky::Empty,
                    '#' => Sky::Galaxy,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec()
}

#[aoc_generator(day11)]
fn day11_generator(input: &str) -> Input {
    let _input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    parse_input(input)
}

#[aoc(day11, part1)]
fn solve_part1(input: &Input) -> usize {
    let empty_lines = input
        .iter()
        .enumerate()
        .filter(|(_, line)| line.iter().all(|l| matches!(l, Sky::Empty)))
        .map(|(y, _)| y)
        .collect_vec();
    let empty_columns = input
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| matches!(c, Sky::Empty))
                .map(|(x, _)| x)
                .collect::<HashSet<usize>>()
        })
        .reduce(|acc, next| acc.intersection(&next).copied().collect())
        .expect("Good input?");

    input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| matches!(c, Sky::Galaxy))
                .map(move |(x, _)| (y, x))
        })
        .combinations(2)
        .map(|distance| {
            let first = distance[0];
            let second = distance[1];
            let distance = (first.0.max(second.0) - first.0.min(second.0))
                + (first.1.max(second.1) - first.1.min(second.1));

            let gap_columns = empty_lines
                .iter()
                .filter(|&&c| c > first.0.min(second.0) && c < first.0.max(second.0))
                .count();
            let gap_rows = empty_columns
                .iter()
                .filter(|&&c| c > first.1.min(second.1) && c < first.1.max(second.1))
                .count();

            distance + gap_columns + gap_rows
        })
        .sum()
}

#[aoc(day11, part2)]
fn solve_part2(input: &Input) -> usize {
    let empty_lines = input
        .iter()
        .enumerate()
        .filter(|(_, line)| line.iter().all(|l| matches!(l, Sky::Empty)))
        .map(|(y, _)| y)
        .collect_vec();
    let empty_columns = input
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| matches!(c, Sky::Empty))
                .map(|(x, _)| x)
                .collect::<HashSet<usize>>()
        })
        .reduce(|acc, next| acc.intersection(&next).copied().collect())
        .expect("Good input?");

    input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| matches!(c, Sky::Galaxy))
                .map(move |(x, _)| (y, x))
        })
        .combinations(2)
        .map(|distance| {
            let first = distance[0];
            let second = distance[1];
            let distance = (first.0.max(second.0) - first.0.min(second.0))
                + (first.1.max(second.1) - first.1.min(second.1));

            let gap_columns = empty_lines
                .iter()
                .filter(|&&c| c > first.0.min(second.0) && c < first.0.max(second.0))
                .count();
            let gap_rows = empty_columns
                .iter()
                .filter(|&&c| c > first.1.min(second.1) && c < first.1.max(second.1))
                .count();

            distance + (999_999 * gap_columns) + (999_999 * gap_rows)
        })
        .sum()
}
