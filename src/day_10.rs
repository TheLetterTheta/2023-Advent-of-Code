use std::collections::{HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;


#[derive(Debug, Eq, Hash, Copy, Clone, PartialEq)]
struct Coord {
    x: i16,
    y: i16,
}

#[derive(Debug)]
enum Pipe {
    Start,
    Connection(Coord, Coord, bool),
    Blank,
}

#[derive(Debug)]
struct Input {
    map: Vec<Vec<Pipe>>,
    start: Coord,
}

fn parse_input(input: &str) -> Input {
    let mut start = Coord { x: 0, y: 0 };
    let map = input
        .lines()
        .enumerate()
        .map(|(y, l)| (y as i16, l))
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| (x as i16, c))
                .map(|(x, c)| match c {
                    '|' => Pipe::Connection(Coord { x, y: y - 1 }, Coord { x, y: y + 1 }, true),
                    'J' => Pipe::Connection(Coord { x, y: y - 1 }, Coord { x: x - 1, y }, true),
                    '7' => Pipe::Connection(Coord { x: x - 1, y }, Coord { x, y: y + 1 }, true),
                    '-' => Pipe::Connection(Coord { x: x - 1, y }, Coord { x: x + 1, y }, false),
                    'L' => Pipe::Connection(Coord { x, y: y - 1 }, Coord { x: x + 1, y }, false),
                    'F' => Pipe::Connection(Coord { x: x + 1, y }, Coord { x, y: y + 1 }, false),
                    '.' => Pipe::Blank,
                    'S' => {
                        start = Coord { x, y };
                        Pipe::Start
                    }
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    Input { map, start }
}

#[aoc_generator(day10)]
fn day10_generator(input: &str) -> Input {
    let _input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    parse_input(input)
}

fn get_start_node(start: Coord, map: &[Vec<Pipe>]) -> Coord {
    // check if up connects to start node
    if let Some(&Pipe::Connection(one, two, _)) = map
        .get(start.y as usize - 1)
        .and_then(|v| v.get(start.x as usize))
    {
        if one == start || two == start {
            return Coord {
                x: start.x,
                y: start.y - 1,
            };
        }
    }

    if let Some(&Pipe::Connection(one, two, _)) = map
        .get(start.y as usize + 1)
        .and_then(|v| v.get(start.x as usize))
    {
        if one == start || two == start {
            return Coord {
                x: start.x,
                y: start.y + 1,
            };
        }
    }

    if let Some(&Pipe::Connection(one, two, _)) = map
        .get(start.y as usize)
        .and_then(|v| v.get(start.x as usize - 1))
    {
        if one == start || two == start {
            return Coord {
                x: start.x - 1,
                y: start.y,
            };
        }
    }

    if let Some(&Pipe::Connection(one, two, _)) = map
        .get(start.y as usize)
        .and_then(|v| v.get(start.x as usize + 1))
    {
        if one == start || two == start {
            return Coord {
                x: start.x + 1,
                y: start.y,
            };
        }
    }

    unreachable!("Input must have valid direction")
}

#[aoc(day10, part1)]
fn solve_part1(input: &Input) -> u16 {
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut curr = get_start_node(input.start, &input.map);
    let mut dist = 2;
    visited.insert(curr);
    visited.insert(input.start);

    while let Some(Pipe::Connection(left, right, _)) = input
        .map
        .get(curr.y as usize)
        .and_then(|v| v.get(curr.x as usize))
    {
        if visited.insert(*left) {
            curr = *left;
        } else if visited.insert(*right) {
            curr = *right;
        } else {
            break;
        }
        dist += 1;
    }

    dist / 2
}

#[aoc(day10, part2)]
fn solve_part2(input: &Input) -> i32 {
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut curr = get_start_node(input.start, &input.map);
    visited.insert(curr);
    visited.insert(input.start);

    while let Some(Pipe::Connection(left, right, _)) = input
        .map
        .get(curr.y as usize)
        .and_then(|v| v.get(curr.x as usize))
    {
        if visited.insert(*left) {
            curr = *left;
        } else if visited.insert(*right) {
            curr = *right;
        } else {
            break;
        }
    }

    let mut count = 0;
    for y in 0..input.map.len() {
        let mut inside = false;
        for x in 0..input.map[y].len() {
            match input.map[y][x] {
                Pipe::Blank => {
                    if inside {
                        count += 1;
                    }
                }
                Pipe::Start => {
                    inside = !inside;
                }
                Pipe::Connection(_, _, wall) => {
                    // check if current wall is inside visited
                    if visited.contains(&Coord {
                        x: x as i16,
                        y: y as i16,
                    }) {
                        if wall {
                            inside = !inside;
                        }
                    } else if inside {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}
