use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::line_ending,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser,
};
use num::Integer;

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
struct Directions {
    steps: Vec<Turn>,
    nodes: HashMap<String, (String, String)>,
}

type Input = Directions;

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, steps) = terminated(
        many1(alt((
            tag("L").map(|_| Turn::Left),
            tag("R").map(|_| Turn::Right),
        ))),
        line_ending,
    )(input)?;

    let (input, nodes) = preceded(
        line_ending,
        separated_list1(
            line_ending,
            separated_pair(
                take(3_usize).map(String::from),
                tag(" = "),
                delimited(
                    tag("("),
                    separated_pair(
                        take(3_usize).map(String::from),
                        tag(", "),
                        take(3_usize).map(String::from),
                    ),
                    tag(")"),
                ),
            ),
        ),
    )(input)?;

    Ok((
        input,
        Directions {
            steps,
            nodes: nodes
                .into_iter()
                .collect::<HashMap<String, (String, String)>>(),
        },
    ))
}

#[aoc_generator(day8)]
fn day8_generator(input: &str) -> Input {
    let _input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    let _input2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    let _input3 = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    let (input, output) = parse_input(input).expect("Could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day8, part1)]
fn solve_part1(input: &Input) -> u32 {
    let mut steps = input.steps.iter().cycle();

    let mut curr = String::from("AAA");
    let mut count = 0;

    while curr != "ZZZ" {
        count += 1;
        let Some(step) = steps.next() else {
            panic!("Cycling - can't run out of steps!")
        };
        let (left, right) = input.nodes.get(&curr).expect("Direction doesn't exist");
        match step {
            Turn::Left => curr = left.to_string(),
            Turn::Right => curr = right.to_string(),
        }
    }
    count
}

#[aoc(day8, part2)]
fn solve_part2(input: &Input) -> u64 {
    let mut instructions = input.steps.iter().cycle();
    input
        .nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|mut v| {
            let mut count = 0;
            while !v.ends_with('Z') {
                count += 1;
                let Some(step) = instructions.next() else {
                    panic!("Cycling - can't run out");
                };

                let (left, right) = input.nodes.get(v).expect("Direction doesn't exist");

                v = match step {
                    Turn::Left => left,
                    Turn::Right => right,
                }
            }
            count as u64
        })
        .reduce(|acc, e| acc.lcm(&e))
        .expect("Not able to compute lcm")
}
