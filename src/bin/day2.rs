use aoc2023_rust::*;
use itertools::Itertools;

fn process_line(line: &str) -> (u32, u32, u32) {
    // "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    let line = line.split_once(": ").unwrap().1;
    // "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"

    fn parse_num_first(s: &str) -> u32 {
        // "12 text"
        s.split_once(' ').unwrap().0.parse().unwrap()
    }

    fn parse_draw(s: &str) -> (u32, u32, u32) {
        // "1 red, 2 green, 6 blue"
        let mut balls = (0, 0, 0);
        for color in s.split(", ") {
            if color.ends_with("red") {
                balls.0 = parse_num_first(color);
            } else if color.ends_with("green") {
                balls.1 = parse_num_first(color);
            } else if color.ends_with("blue") {
                balls.2 = parse_num_first(color);
            } else {
                panic!("Could not find any color in the fragment")
            }
        }

        balls
    }

    fn tuple_max<T: Ord> (t1: (T, T, T), t2: (T, T, T)) -> (T, T, T) {
        (t1.0.max(t2.0), t1.1.max(t2.1), t1.2.max(t2.2))
    }

    line.split("; ")
        .map(parse_draw)
        .fold((0, 0, 0), tuple_max)
}


fn main() {
    let lines = read_input_lines("input/day2.txt");
    let games_maxes = lines
        .iter()
        .map(|s| process_line(s))
        .collect_vec();

    const BALLS_MAX: (u32, u32, u32) = (12, 13, 14);
    let part1: u32 = games_maxes
        .iter()
        .enumerate()
        .filter_map(
            |(idx, balls)|
                if balls.0 <= BALLS_MAX.0
                && balls.1 <= BALLS_MAX.1
                && balls.2 <= BALLS_MAX.2 {
                    Some((idx + 1) as u32)
                } else {
                    None
                })
        .sum();

    println!("Part 1: {}", part1);

    let part2: u32 = games_maxes.iter().map(|balls| balls.0 * balls.1 * balls.2).sum();
    println!("Part 2: {}", part2);
}


#[test]
fn day2_example() {
    assert!(process_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green") == (4, 2, 6));
    assert!(process_line("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue") == (1, 3, 4));
    assert!(process_line("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red") == (20, 13, 6));
    assert!(process_line("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red") == (14, 3, 15));
    assert!(process_line("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green") == (6, 3, 2));
}
