use aoc2023_rust::read_input_lines;
use itertools::Itertools;

fn parse_line(l: &str) -> Vec<i32> {
    l.split(' ').map(|s| s.parse().unwrap()).collect()
}

fn next_value(v: &[i32]) -> i32 {
    if v.iter().all_equal() { v[0] }
    else {
        let next_layer: Vec<_> = v.iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect();
        next_value(&next_layer) + v.last().unwrap()
    }
}

fn main() {
    let lines = read_input_lines("input/day9.txt");
    let sequences: Vec<_> = lines.iter().map(|l| parse_line(l)).collect();

    let part1: i32 = sequences.iter().map(|s| next_value(s)).sum();
    println!("Part 1: {}", part1);

    let mut sequences = sequences;
    sequences.iter_mut().for_each(|seq| seq.reverse());

    let part2: i32 = sequences.iter().map(|s| next_value(s)).sum();
    println!("Part 2: {}", part2);
}

#[test]
fn day9_example() {
    assert!(next_value(&[ 0,  3,  6,  9, 12, 15]) == 18);
    assert!(next_value(&[ 1,  3,  6, 10, 15, 21]) == 28);
    assert!(next_value(&[10, 13, 16, 21, 30, 45]) == 68);
}
