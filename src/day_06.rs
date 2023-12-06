use aoc_runner_derive::{aoc};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};


#[derive(Debug)]
struct Race<T> {
    time: T,
    record_distance: T,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Race<u16>>> {
    let (input, (times, distances)) = separated_pair(
        preceded(
            tuple((tag("Time:"), space1)),
            separated_list1(space1, complete::u16),
        ),
        line_ending,
        preceded(
            tuple((tag("Distance:"), space1)),
            separated_list1(space1, complete::u16),
        ),
    )(input)?;

    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &record_distance)| Race::<u16> {
            time,
            record_distance,
        })
        .collect_vec();

    Ok((input, races))
}

fn parse_input_big_number(input: &str) -> IResult<&str, Race<u64>> {
    let (input, times) = preceded(
        tuple((tag("Time:"), space1)),
        separated_list1(space1, digit1),
    )(input)?;
    let (input, distances) = preceded(
        tuple((line_ending, tag("Distance:"), space1)),
        separated_list1(space1, digit1),
    )(input)?;

    let time = times.join("").parse::<u64>().expect("large number");
    let record_distance = distances.join("").parse::<u64>().expect("large number");

    Ok((
        input,
        Race::<u64> {
            time,
            record_distance,
        },
    ))
}

#[aoc(day6, part1)]
fn solve_part1(input: &str) -> usize {
    let (_, input) = parse_input(input).expect("Could not parse input");

    input
        .iter()
        .map(|race| {
            let first_time = (1..race.time)
                .find(|time| time * (race.time - time) > race.record_distance)
                .expect("race not possible");
            (first_time..=(race.time - first_time)).count()
        })
        .product()
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> usize {
    let (_, input) = parse_input_big_number(input).expect("Could not parse input");

    let first_time = (1..input.time)
        .find(|time| time * (input.time - time) > input.record_distance)
        .expect("race not possible");
    (first_time..=(input.time - first_time)).count()
}
