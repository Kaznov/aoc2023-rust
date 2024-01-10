use std::num::ParseIntError;
use aoc2023_rust::read_input_lines;
use itertools::Itertools;
use thiserror::Error;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandKind {
    HighCard = 0,
    Pair = 1,
    TwoPairs = 2,
    ThreeOfKind = 3,
    FullHouse = 4,
    FourOfKind = 5,
    FiveOfKind = 6
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct HandOfCards {
    kind: HandKind,
    cards: [u8; 5],
}

#[derive(Error, Debug)]
enum ParseHandError {
    #[error("Hand does not have 5 cards")]
    WrongHandSize,
    
    #[error("One of characters does not represent a card")]
    UnexpectedCharacter
}

impl std::str::FromStr for HandOfCards {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const CARD_ORDER: &str = "$23456789TJQKA";
        const JOKER: u8 = 0; // position of '$'
        
        let s: [char; 5] = s
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| ParseHandError::WrongHandSize)?;
        
        if !s.iter().all(|&c| CARD_ORDER.contains(c)) {
            return Err(ParseHandError::UnexpectedCharacter);
        }

        // values of the cards
        let cards = s
            .map(|c| CARD_ORDER
                .chars()
                .position(|card| c == card).unwrap() as u8);

        let mut counts_map = cards
            .iter()
            .counts();

        let joker_count = *counts_map.get(&JOKER).unwrap_or(&0);
        counts_map.remove(&JOKER);

        let mut counts = counts_map
            .values()
            .cloned()
            .sorted_by(|x1, x2| x2.cmp(x1)) // decreasingly
            .collect_vec(); 

        if counts.is_empty() { counts.push(0); } // if only jokers
        counts[0] += joker_count;

        let kind = match counts.as_slice() {
            [5] => HandKind::FiveOfKind,
            [4, 1] => HandKind::FourOfKind,
            [3, 2] => HandKind::FullHouse,
            [3, 1, 1] => HandKind::ThreeOfKind,
            [2, 2, 1] => HandKind::TwoPairs,
            [2, ..] => HandKind::Pair,
            [1, ..] => HandKind::HighCard,
            _ => unreachable!()
        };

        Ok(HandOfCards {kind, cards})
    }
}


struct Play {
    hand: HandOfCards,
    bid: u32
}

#[derive(Error, Debug)]
enum ParsePlayError {
    #[error(transparent)]
    Hand(#[from] ParseHandError),
    
    #[error(transparent)]
    Int(#[from] ParseIntError),

    #[error("Could not split play into a hand and an error")]
    Split
}

impl std::str::FromStr for Play {
    type Err = ParsePlayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once(' ').ok_or(ParsePlayError::Split)?;
        Ok(Play {
            hand: l.parse()?,
            bid: r.parse()? 
        })
    }
}

fn solve(lines: &[String]) -> u32 {
    let mut plays: Vec<Play> = lines
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    // sort by key has issues with lifetime, as its lambda returns by value
    plays.sort_by(|p1, p2| p1.hand.cmp(&p2.hand));

    plays
        .iter()
        .enumerate()
        .map(|(idx, play)| play.bid * (idx as u32 + 1))
        .sum()
}

fn main() {
    let mut lines = read_input_lines("input/day7.txt");

    let part1 = solve(&lines);
    println!("Part 1: {}", part1);

    lines.iter_mut().for_each(|s| *s = s.replace('J', "$"));
    let part2 = solve(&lines);
    println!("Part 2: {}", part2);
}
