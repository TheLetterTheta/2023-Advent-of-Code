use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, none_of, not_line_ending, one_of},
    multi::{many0, many1, separated_list1},
    sequence::{preceded, tuple},
    IResult, Parser,
};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum Chars {
    Blank,
    Symbol,
    Number(u32),
    Gear,
}

type Input = Vec<Vec<Chars>>;

fn parse_line(input: &str) -> IResult<&str, Vec<Chars>> {
    many0(alt((
        tag(".").map(|_| Chars::Blank),
        tag("*").map(|_| Chars::Gear),
        one_of("0123456789").map(|n| Chars::Number(n.to_digit(10).expect("Digit"))),
        none_of(".0123456789*\n").map(|_| Chars::Symbol),
    )))(input)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, list) = separated_list1(line_ending, parse_line)(input)?;

    Ok((input, list.into_iter().collect()))
}

#[aoc_generator(day3)]
pub fn day2_generator(input: &str) -> Input {
    let _input = "467..114..
...*......
..35..633.
......#...
617*1.....
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let (input, output) = parse_input(input).expect("Could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Input) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(level, row)| {
            let (num, sum) = row
                .iter()
                .enumerate()
                .filter(|(column, n)| {
                    // check if n should be included still
                    match n {
                        Chars::Number(_) => {
                            let mut left_boundary = (0..=*column)
                                .rev()
                                .take_while(|i| matches!(input[level][*i], Chars::Number(_)))
                                .min()
                                .expect("Left boundary");
                            let mut right_boundary = (*column..input[level].len())
                                .take_while(|i| matches!(input[level][*i], Chars::Number(_)))
                                .max()
                                .expect("Right boundary");

                            if left_boundary > 0 {
                                left_boundary -= 1;
                            }
                            if right_boundary < input[level].len() - 1 {
                                right_boundary += 1;
                            }

                            if level > 0
                                && (left_boundary..=right_boundary).any(|index| {
                                    matches!(input[level - 1][index], Chars::Gear | Chars::Symbol)
                                })
                            {
                                return true;
                            }
                            if level < input.len() - 1
                                && (left_boundary..=right_boundary).any(|index| {
                                    matches!(input[level + 1][index], Chars::Gear | Chars::Symbol)
                                })
                            {
                                return true;
                            }
                            if matches!(input[level][left_boundary], Chars::Gear | Chars::Symbol) {
                                return true;
                            }
                            if matches!(input[level][right_boundary], Chars::Gear | Chars::Symbol) {
                                return true;
                            }

                            false
                        }
                        _ => true,
                    }
                })
                .fold((0, 0), |acc, (_, c)| match c {
                    Chars::Number(n) => (acc.0 * 10 + n, acc.1),
                    _ => (0, acc.1 + acc.0),
                });
            num + sum
        })
        .sum::<u32>()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Input) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(level, row)| {
            row.iter()
                .enumerate()
                .map(|(column, n)| {
                    // check if n should be included still
                    match n {
                        Chars::Gear => {
                            let mut found_up = false;
                            let mut up_num = 1;
                            let mut found_down = false;
                            let mut down_num = 1;
                            let mut found_left = false;
                            let mut left_num = 1;
                            let mut found_right = false;
                            let mut right_num = 1;

                            let mut found_upleft = false;
                            let mut upleft_num = 1;
                            let mut found_upright = false;
                            let mut upright_num = 1;

                            let mut found_downleft = false;
                            let mut downleft_num = 1;
                            let mut found_downright = false;
                            let mut downright_num = 1;

                            if level > 0 {
                                found_up = matches!(input[level - 1][column], Chars::Number(_));
                                if found_up {
                                    let left_boundary = (0..=column)
                                        .rev()
                                        .take_while(|i| {
                                            matches!(input[level - 1][*i], Chars::Number(_))
                                        })
                                        .min()
                                        .expect("Left boundary");
                                    let right_boundary = (column..input[level].len())
                                        .take_while(|i| {
                                            matches!(input[level - 1][*i], Chars::Number(_))
                                        })
                                        .max()
                                        .expect("Right boundary");

                                    up_num = (left_boundary..=right_boundary)
                                        .map(|i| match input[level - 1][i] {
                                            Chars::Number(n) => n,
                                            _ => unreachable!(),
                                        })
                                        .fold(0, |acc, n| acc * 10 + n);
                                }
                                if !found_up
                                    && column > 0
                                    && matches!(input[level - 1][column - 1], Chars::Number(_))
                                {
                                    found_upleft = true;
                                    let left_boundary = (0..column)
                                        .rev()
                                        .take_while(|i| {
                                            matches!(input[level - 1][*i], Chars::Number(_))
                                        })
                                        .min()
                                        .expect("Left boundary");
                                    let right_boundary = column - 1;

                                    upleft_num = (left_boundary..=right_boundary)
                                        .map(|i| match input[level - 1][i] {
                                            Chars::Number(n) => n,
                                            _ => unreachable!(),
                                        })
                                        .fold(0, |acc, n| acc * 10 + n);
                                }
                                if !found_up
                                    && column < input[level].len() - 1
                                    && matches!(input[level - 1][column + 1], Chars::Number(_))
                                {
                                    found_upright = true;
                                    let left_boundary = column + 1;
                                    let right_boundary = (column + 1..input[level].len())
                                        .take_while(|i| {
                                            matches!(input[level - 1][*i], Chars::Number(_))
                                        })
                                        .max()
                                        .expect("Right boundary");

                                    upright_num = (left_boundary..=right_boundary)
                                        .map(|i| match input[level - 1][i] {
                                            Chars::Number(n) => n,
                                            _ => unreachable!(),
                                        })
                                        .fold(0, |acc, n| acc * 10 + n);
                                }
                            }

                            if level < input.len() - 1 {
                                found_down = matches!(input[level + 1][column], Chars::Number(_));
                                if found_down {
                                    let left_boundary = (0..=column)
                                        .rev()
                                        .take_while(|i| {
                                            matches!(input[level + 1][*i], Chars::Number(_))
                                        })
                                        .min()
                                        .expect("Left boundary");
                                    let right_boundary = (column..input[level].len())
                                        .take_while(|i| {
                                            matches!(input[level + 1][*i], Chars::Number(_))
                                        })
                                        .max()
                                        .expect("Right boundary");

                                    down_num = (left_boundary..=right_boundary)
                                        .map(|i| match input[level + 1][i] {
                                            Chars::Number(n) => n,
                                            _ => unreachable!(),
                                        })
                                        .fold(0, |acc, n| acc * 10 + n);
                                }
                                if !found_down
                                    && column > 0
                                    && matches!(input[level + 1][column - 1], Chars::Number(_))
                                {
                                    found_downleft = true;
                                    let left_boundary = (0..column)
                                        .rev()
                                        .take_while(|i| {
                                            matches!(input[level + 1][*i], Chars::Number(_))
                                        })
                                        .min()
                                        .expect("Left boundary");
                                    let right_boundary = column - 1;

                                    downleft_num = (left_boundary..=right_boundary)
                                        .map(|i| match input[level + 1][i] {
                                            Chars::Number(n) => n,
                                            _ => unreachable!(),
                                        })
                                        .fold(0, |acc, n| acc * 10 + n);
                                }
                                if !found_down
                                    && column < input[level].len() - 1
                                    && matches!(input[level + 1][column + 1], Chars::Number(_))
                                {
                                    found_downright = true;
                                    let left_boundary = column + 1;
                                    let right_boundary = (column + 1..input[level].len())
                                        .take_while(|i| {
                                            matches!(input[level + 1][*i], Chars::Number(_))
                                        })
                                        .max()
                                        .expect("Right boundary");

                                    downright_num = (left_boundary..=right_boundary)
                                        .map(|i| match input[level + 1][i] {
                                            Chars::Number(n) => n,
                                            _ => unreachable!(),
                                        })
                                        .fold(0, |acc, n| acc * 10 + n);
                                }
                            }

                            if column > 0 && matches!(input[level][column - 1], Chars::Number(_)) {
                                found_left = true;
                                let left_boundary = (0..column)
                                    .rev()
                                    .take_while(|i| matches!(input[level][*i], Chars::Number(_)))
                                    .min()
                                    .expect("Left boundary");
                                let right_boundary = column - 1;

                                left_num = (left_boundary..=right_boundary)
                                    .map(|i| match input[level][i] {
                                        Chars::Number(n) => n,
                                        _ => unreachable!(),
                                    })
                                    .fold(0, |acc, n| acc * 10 + n);
                            }

                            if column < input[level].len() - 1
                                && matches!(input[level][column + 1], Chars::Number(_))
                            {
                                found_right = true;
                                let left_boundary = column + 1;
                                let right_boundary = (column + 1..input[level].len())
                                    .take_while(|i| matches!(input[level][*i], Chars::Number(_)))
                                    .max()
                                    .expect("Right boundary");

                                right_num = (left_boundary..=right_boundary)
                                    .map(|i| match input[level][i] {
                                        Chars::Number(n) => n,
                                        _ => unreachable!(),
                                    })
                                    .fold(0, |acc, n| acc * 10 + n);
                            }

                            if vec![
                                found_up,
                                found_down,
                                found_left,
                                found_right,
                                found_upleft,
                                found_upright,
                                found_downleft,
                                found_downright,
                            ]
                            .iter()
                            .filter(|i| **i)
                            .count()
                                == 2
                            {
                                    up_num
                                        * down_num
                                        * left_num
                                        * right_num
                                        * upleft_num
                                        * upright_num
                                        * downleft_num
                                        * downright_num
                            } else {
                                0
                            }
                        }
                        Chars::Symbol => 0,
                        _ => 0,
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}
