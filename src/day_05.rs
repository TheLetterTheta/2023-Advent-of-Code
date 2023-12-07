use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
struct Range {
    source_start: u32,
    destination_start: u32,
    size: u32,
}

impl Range {
    fn source_end(&self) -> u32 {
        self.source_start + self.size - 1
    }

    fn destination_end(&self) -> u32 {
        self.destination_start + self.size - 1
    }

    fn map_within(&self, from: u32) -> u32 {
        if from < self.source_start || from > self.source_end() {
            return from;
        }

        self.destination_start + (from - self.source_start)
    }
}

#[derive(Debug)]
struct SeedMap {
    seeds: Vec<u32>,
    maps: HashMap<String, (String, Vec<Range>)>,
}

type Input = SeedMap;

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, (destination_start, source_start, size)) = tuple((
        terminated(complete::u32, space1),
        terminated(complete::u32, space1),
        complete::u32,
    ))(input)?;

    Ok((
        input,
        Range {
            destination_start,
            source_start,
            size,
        },
    ))
}

fn parse_map(input: &str) -> IResult<&str, (String, (String, Vec<Range>))> {
    let (input, (from, to)) = terminated(
        separated_pair(alpha1, tag("-to-"), alpha1),
        tuple((tag(" map:"), line_ending)),
    )(input)?;
    let (input, range) = separated_list1(line_ending, parse_range)(input)?;

    Ok((input, (from.to_string(), (to.to_string(), range))))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, seeds) = delimited(
        tag("seeds: "),
        separated_list1(tag(" "), complete::u32),
        tuple((line_ending, line_ending)),
    )(input)?;
    let (input, maps) = separated_list1(tuple((line_ending, line_ending)), parse_map)(input)?;

    Ok((
        input,
        SeedMap {
            seeds,
            maps: maps.into_iter().collect::<HashMap<String, _>>(),
        },
    ))
}

#[aoc_generator(day5)]
fn day5_generator(input: &str) -> Input {
    let _input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    let (input, output) = parse_input(input).expect("Could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day5, part1)]
fn solve_part1(input: &Input) -> u32 {
    let mut state = "seed".to_string();
    let mut vals = input.seeds.clone();

    while let Some(next) = input.maps.get(&state) {
        state = next.0.clone();
        vals.iter_mut().for_each(|v| {
            if let Some(map_to) = next
                .1
                .iter()
                .find(|n| *v >= n.source_start && *v <= n.source_end())
            {
                *v = map_to.map_within(*v);
            }
        });
    }

    *vals.iter().min().expect("Some value to exist")
}

trait Diff {
    fn diff(self, subtract: &Self) -> Vec<Self>
    where
        Self: Sized;
}

impl<T> Diff for std::ops::RangeInclusive<T>
where
    T: Sized
        + std::cmp::PartialOrd
        + std::ops::Add<u32, Output = T>
        + std::ops::Sub<u32, Output = T>
        + std::fmt::Debug
        + Copy,
{
    fn diff(self, subtract: &Self) -> Vec<Self> {
        if subtract.end() < self.start() || subtract.start() > self.end() {
            // non-conflicting
            vec![self]
        } else if subtract.start() <= self.start() {
            if subtract.end() >= self.end() {
                vec![]
            } else {
                // starts before, and ends before
                vec![(*subtract.end() + 1)..=*self.end()]
            }
        } else if subtract.end() >= self.end() {
            vec![*self.start()..=(*subtract.start())]
        } else {
            vec![
                *self.start()..=(*subtract.start()),
                (*subtract.end() + 1)..=*self.end(),
            ]
        }
    }
}

#[aoc(day5, part2)]
fn solve_part2(input: &Input) -> u32 {
    let mut state = "seed".to_string();
    let mut vals = input
        .seeds
        .chunks(2)
        .map(|s| (s[0]..=(s[0] + s[1] - 1)))
        .collect_vec();

    while let Some(map_to) = input.maps.get(&state) {
        state = map_to.0.clone();

        vals = vals
            .iter()
            .flat_map(|seed_range| {
                let lookup_ranges = map_to
                    .1
                    .iter()
                    .filter(|lookup_range| {
                        seed_range.end() >= &lookup_range.source_start
                            && seed_range.start() <= &lookup_range.source_end()
                    })
                    .map(|lookup_range| {
                        if lookup_range.source_start <= *seed_range.start() {
                            if lookup_range.source_end() >= *seed_range.end() {
                                // fully encompassed in this range
                                (
                                    *seed_range.start(),
                                    *seed_range.end(),
                                    lookup_range.map_within(*seed_range.start())
                                        ..=lookup_range.map_within(*seed_range.end()),
                                )
                            } else {
                                (
                                    *seed_range.start(),
                                    lookup_range.source_end(),
                                    lookup_range.map_within(*seed_range.start())
                                        ..=lookup_range.destination_end(),
                                )
                            }
                        } else if lookup_range.source_end() <= *seed_range.end() {
                            // conflict is contained within the range
                            (
                                lookup_range.source_start,
                                lookup_range.source_end(),
                                lookup_range.destination_start..=lookup_range.destination_end(),
                            )
                        } else {
                            // conflict ends after source
                            (
                                lookup_range.source_start,
                                *seed_range.end(),
                                lookup_range.destination_start
                                    ..=lookup_range.map_within(*seed_range.end()),
                            )
                        }
                    }).collect_vec();

                let mut new_range = vec![seed_range.clone()];
                for t in lookup_ranges.iter() {
                    new_range = new_range
                        .iter()
                        .flat_map(|v| v.clone().diff(&(t.0..=t.1)))
                        .collect_vec();
                }

                for t in lookup_ranges {
                    new_range.push(t.2);
                }

                new_range
            })
            .collect_vec();
    }

    vals.iter().map(|r| *r.start()).min().expect("min to exist")
}
