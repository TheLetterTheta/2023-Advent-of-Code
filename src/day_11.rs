use std::collections::{HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;


struct Input {
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_columns: HashSet<usize>,
}

fn parse_input(input: &str) -> Input {
    let line_length = input.lines().next().expect("No input").chars().count();
    let mut empty_columns = (0..line_length).collect::<HashSet<usize>>();
    let mut empty_rows = vec![];
    let mut galaxies = vec![];

    input.lines().enumerate().for_each(|(y, line)| {
        let mut found_galaxy = false;

        line.chars().enumerate().filter(|&(_, c)| c == '#').for_each(|(x, _)| {
                found_galaxy = true;
                galaxies.push((y, x));
                empty_columns.remove(&x);
        });

        if !found_galaxy {
            empty_rows.push(y);
        }
    });

    Input {
        galaxies,
        empty_columns,
        empty_rows,
    }
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
    let empty_rows = &input.empty_rows;
    let empty_columns = &input.empty_columns;
    let galaxies = &input.galaxies;

    galaxies
        .iter()
        .combinations(2)
        .map(|distance| {
            let first = distance[0];
            let second = distance[1];
            let distance = (first.0.max(second.0) - first.0.min(second.0))
                + (first.1.max(second.1) - first.1.min(second.1));

            let gap_columns = empty_rows
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
    let empty_rows = &input.empty_rows;
    let empty_columns = &input.empty_columns;
    let galaxies = &input.galaxies;

    galaxies
        .iter()
        .combinations(2)
        .map(|distance| {
            let first = distance[0];
            let second = distance[1];
            let distance = (first.0.max(second.0) - first.0.min(second.0))
                + (first.1.max(second.1) - first.1.min(second.1));

            let gap_columns = empty_rows
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
