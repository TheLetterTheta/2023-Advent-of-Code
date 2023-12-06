use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, space1, digit1},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, (times, distances)) = separated_pair(
        preceded(
            tuple((tag("Time:"), space1)),
            separated_list1(space1, complete::u64),
        ),
        line_ending,
        preceded(
            tuple((tag("Distance:"), space1)),
            separated_list1(space1, complete::u64),
        ),
    )(input)?;

    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &record_distance)| Race {
            time,
            record_distance,
        })
        .collect_vec();

    Ok((input, races))
}

fn parse_input_big_number(input: &str) -> IResult<&str,Race> {
    let (input, times) = 
        preceded(
            tuple((tag("Time:"), space1)),
            separated_list1(space1, digit1),
        )(input)?;
    let (input, distances) = 
        preceded(
            tuple((line_ending,tag("Distance:"), space1)),
            separated_list1(space1, digit1),
        )(input)?;

    let time = times.join("").parse::<u64>().expect("large number");
    let record_distance = distances.join("").parse::<u64>().expect("large number");
    
    Ok((input, Race {
        time,
        record_distance
    }))
}

#[aoc_generator(day6)]
fn day6_generator(input: &str) -> String {
    let _input = "Time:      7  15   30
Distance:  9  40  200";

    input.to_string()
}

#[aoc(day6, part1)]
fn solve_part1(input: &String) -> usize {
    let (input, output) = parse_input(&input).expect("Could not parse input");
    assert!(input.is_empty());
    let input = output;

    input.iter()
        .map(|race| {
            let first_time = (1..race.time).find(|time| time * (race.time - time) > race.record_distance).expect("race not possible");
            (first_time..=(race.time - first_time)).count()
        })
        .product()
}

#[aoc(day6, part2)]
fn solve_part2(input: &String) -> usize {
    let (input, output) = parse_input_big_number(&input).expect("Could not parse input");
    assert!(input.is_empty());
    let input = output;

    let first_time = (1..input.time).find(|time| time * (input.time - time) > input.record_distance).expect("race not possible");
    (first_time..=(input.time - first_time)).count()

}
