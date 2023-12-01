use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

type Input = Vec<String>;

#[aoc_generator(day1)]
pub fn day1_generator(input: &str) -> Input {
    input.lines()
        .map(|l| l.to_string())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Input) -> u32 {
    input.iter()
        .map(|l| {
             let first = l.chars().find(char::is_ascii_digit).expect("digit to be in string").to_digit(10).unwrap();
            let last = l.chars().rev().find(char::is_ascii_digit).expect("digit to be in string").to_digit(10).unwrap();

            (10*first) + last
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Input) -> u32 {
    input.iter()
        .map(|l| {
            let l = l.replace("one", "o1ne");
            let l = l.replace("two", "t2wo");
            let l = l.replace("three", "th3ree");
            let l = l.replace("four", "fo4ur");
            let l = l.replace("five", "fi5ve");
            let l = l.replace("six", "si6x");
            let l = l.replace("seven", "se7ven");
            let l = l.replace("eight", "ei8ght");
            let l = l.replace("nine", "ni9ne");

            let first = l.chars().find(char::is_ascii_digit).expect("digit to be in string").to_digit(10).unwrap();
            let last = l.chars().rev().find(char::is_ascii_digit).expect("digit to be in string").to_digit(10).unwrap();

            (10*first) + last
        })
        .sum()
}
