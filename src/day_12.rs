use std::collections::HashSet;
use rayon::iter::ParallelIterator;

use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::ParallelBridge;
use core::slice::Iter;

use itertools::{repeat_n, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Condition {
    Unknown,
    Broken,
    Fixed,
}

#[derive(Debug)]
struct Line {
    spring: Vec<Condition>,
    required: Vec<u8>,
}

impl Line {
    fn count_unknown(&self) -> usize {
        self.spring
            .iter()
            .filter(|v| matches!(v, Condition::Unknown))
            .count()
    }

    fn count_broken(&self) -> usize {
        self.spring
            .iter()
            .filter(|v| matches!(v, Condition::Broken))
            .count()
    }

    fn total(&self) -> u8 {
        self.required.iter().sum()
    }

    fn permutations(&self) -> itertools::MultiProduct<core::array::IntoIter<Condition, 2>> {
        let count = self.count_unknown();

        repeat_n([Condition::Fixed, Condition::Broken], count).multi_cartesian_product()
    }

    fn is_valid_permutation(&self, permutation: &[Condition]) -> bool {
        let mut char = permutation.iter();

        self.spring
            .iter()
            .map(|c| match c {
                Condition::Unknown => char.next().expect("enough permutations exist"),
                v => v,
            })
            .dedup_with_count()
            .filter(|&(_, v)| matches!(v, Condition::Broken))
            .map(|(count, _)| count as u8)
            .collect_vec()
            == self.required
    }

    fn five_permutations(&self) -> itertools::MultiProduct<core::array::IntoIter<Condition, 2>> {
        let count = self.count_unknown() * 5;

        repeat_n([Condition::Fixed, Condition::Broken], count).multi_cartesian_product()
    }

    fn is_valid_permutation_five(&self, permutation: &[Condition]) -> bool {
        let mut char = permutation.iter();

        self.spring
            .iter()
            .cycle()
            .take(self.spring.len() * 5)
            .map(|c| match c {
                Condition::Unknown => char.next().expect("enough permutations exist"),
                v => v,
            })
            .dedup_with_count()
            .filter(|&(_, v)| matches!(v, Condition::Broken))
            .map(|(count, _)| count as u8)
            .collect_vec()
            == self.required
    }

}

type Input = Vec<Line>;

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, (spring, required)) = separated_pair(
        many1(alt((
            tag("#").map(|_| Condition::Broken),
            tag(".").map(|_| Condition::Fixed),
            tag("?").map(|_| Condition::Unknown),
        ))),
        space1,
        separated_list1(tag(","), complete::u8),
    )(input)?;

    Ok((input, Line { spring, required }))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    separated_list1(line_ending, parse_line)(input)
}

#[aoc_generator(day12)]
fn day12_generator(input: &str) -> Input {
    let _input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    let (input, output) = parse_input.parse(_input).expect("could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day12, part1)]
fn solve_part1(input: &Input) -> usize {
    input
        .iter()
        .map(|line| {
            line.permutations()
                .filter(|permutation| line.is_valid_permutation(permutation))
                .count()
        })
        .sum()
}

#[aoc(day12, part2)]
fn solve_part2(input: &Input) -> usize {
    input
        .iter()
        .map(|line| {
            line.five_permutations()
                .par_bridge()
                .filter(|permutation| line.is_valid_permutation_five(permutation))
                .count()
        })
        .sum()
}
