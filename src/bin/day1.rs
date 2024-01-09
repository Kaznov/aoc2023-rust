use aoc2023_rust::*;
use itertools::{Itertools, MinMaxResult};

/// Given a line, find the first and the last digit in the line.
/// Return 10 * d1 + d2
fn process_line_part1(line: &str) -> u32 {
    let digits: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    let (d1, d2) = (digits.first().unwrap(), digits.last().unwrap());
    10 * d1 + d2
}

/// Given a line, find the first and the last digit in the line - written as a number or a word.
/// Returns 10 * d1 + d2
fn process_line_part2(line: &str) -> u32 {
    const DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    const DIGITS2: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut all_matches = Vec::new();
    let digit_patterns = (1..=9).zip(DIGITS).chain((1..=9).zip(DIGITS2));

    for (digit, name) in digit_patterns {
        let digit_matches = line.match_indices(name).map(|pos| (pos.0, digit));
        all_matches.extend(digit_matches);
    }

    // get digits from the first and last position
    let first_last = all_matches.iter().minmax_by_key(|(pos, _)| pos);
    match first_last {
        MinMaxResult::NoElements => panic!("Each line must contain a digit"),
        MinMaxResult::OneElement((_, x)) => 10*x + x,
        MinMaxResult::MinMax((_, x), (_, y)) => 10*x + y
    }
}

fn main() {
    let lines = read_input_lines("input/day1.txt");
    let part1: u32 = lines.iter().map(|s| process_line_part1(s)).sum();
    println!("Part 1: {}", part1);

    let part2: u32 = lines.iter().map(|s| process_line_part2(s)).sum();
    println!("Part 2: {}", part2);
}

#[test]
fn day1_part1_example() {
    assert!(process_line_part1("1abc2")       == 12);
    assert!(process_line_part1("pqr3stu8vwx") == 38);
    assert!(process_line_part1("a1b2c3d4e5f") == 15);
    assert!(process_line_part1("treb7uchet")  == 77);
}

#[test]
fn day1_part2_example() {
    assert!(process_line_part2("two1nine")          == 29);
    assert!(process_line_part2("eightwothree")      == 83);
    assert!(process_line_part2("abcone2threexyz")   == 13);
    assert!(process_line_part2("xtwone3four")       == 24);
    assert!(process_line_part2("4nineeightseven2")  == 42);
    assert!(process_line_part2("zoneight234")       == 14);
    assert!(process_line_part2("7pqrstsixteen")     == 76);
}
