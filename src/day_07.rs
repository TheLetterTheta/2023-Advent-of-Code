use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, one_of},
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};

#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
    bid: u16,
}

fn card_to_u8(c: char) -> u8 {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

type Input = Vec<Hand>;

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let mut cards = [0, 0, 0, 0, 0];

    let (input, card) = one_of("23456789TJQKA").map(card_to_u8).parse(input)?;
    cards[0] = card;
    let (input, card) = one_of("23456789TJQKA").map(card_to_u8).parse(input)?;
    cards[1] = card;
    let (input, card) = one_of("23456789TJQKA").map(card_to_u8).parse(input)?;
    cards[2] = card;
    let (input, card) = one_of("23456789TJQKA").map(card_to_u8).parse(input)?;
    cards[3] = card;
    let (input, card) = one_of("23456789TJQKA").map(card_to_u8).parse(input)?;
    cards[4] = card;

    let (input, bid) = preceded(tag(" "), complete::u16)(input)?;

    Ok((input, Hand { cards, bid }))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    separated_list1(line_ending, parse_hand)(input)
}

#[aoc_generator(day7)]
fn day7_generator(input: &str) -> Input {
    let _input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
J2T3K 11";

    let (input, output) = parse_input(input).expect("Could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day7, part1)]
fn solve_part1(input: &Input) -> u32 {
    struct Helper {
        counts: Vec<usize>,
        cards: [u8; 5],
        bid: u16,
    }

    input
        .iter()
        .map(|hand| Helper {
            counts: hand
                .cards
                .iter()
                .counts()
                .values()
                .sorted_unstable()
                .rev()
                .copied()
                .collect_vec(),
            cards: hand.cards,
            bid: hand.bid,
        })
        .sorted_unstable_by(|left, right| match left.counts.cmp(&right.counts) {
            std::cmp::Ordering::Equal => {
                // compare elements
                // Vec ord is element by element
                left.cards.cmp(&right.cards)
            }
            o => o,
        })
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1) * hand.bid as u32)
        .sum::<u32>()
}

#[aoc(day7, part2)]
fn solve_part2(input: &Input) -> u32 {
    #[derive(Debug)]
    struct Helper {
        counts: Vec<usize>,
        jokers: u8,
        cards: [u8; 5],
        bid: u16,
    }

    input
        .iter()
        .map(|hand| Helper {
            counts: hand
                .cards
                .into_iter()
                .filter(|&v| v != 11) // filter jokers from this count
                .counts()
                .values()
                .sorted_unstable()
                .rev()
                .copied()
                .collect_vec(),
            jokers: hand.cards.iter().filter(|&&card| card == 11).count() as u8,
            cards: hand.cards.map(|v| if v == 11 { 1 } else { v }),
            bid: hand.bid,
        })
        .map(|mut v| {
            v.counts
                .first_mut()
                .map(|count| *count += v.jokers as usize)
                .or_else(|| {
                    v.counts.push(v.jokers as usize);
                    Some(())
                });
            v
        })
        .sorted_unstable_by(|left, right| {
            // compare elements
            // Vec ord is element by element
            match left.counts.cmp(&right.counts) {
                std::cmp::Ordering::Equal => {
                    // compare elements
                    // Vec ord is element by element
                    left.cards.cmp(&right.cards)
                }
                o => o,
            }
        })
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1) * hand.bid as u32)
        .sum::<u32>()
}
