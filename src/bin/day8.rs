use std::collections::HashMap;

use aoc2023_rust::read_input_lines;
use scan_fmt::scan_fmt;
use simple_scan::IteratorSimpleScanExt;

struct DesertCrossing {
    left: String,
    right: String
}

fn parse_line(line: &str) -> (String, DesertCrossing) {
    let (node, left, right) = scan_fmt!(line, "{} = ({}, {})", String, String, String).unwrap();
    (node, DesertCrossing{left, right})
}

fn get_position_sequence<'a>(
        directions: &'a str,
        map: &'a HashMap<String, DesertCrossing>,
        start: &'a str) -> impl Iterator<Item=&'a str> {
    directions
        .chars()
        .cycle()
        .trace(start, |&position, direction| {
            let node = map.get(position).unwrap();
            match direction {
                'L' => node.left.as_str(),
                'R' => node.right.as_str(),
                _ => panic!("Unexpected character")
            }
        })
}

fn main() {
    let lines = read_input_lines("input/day8.txt");
    let directions: &str = &lines[0];
    let desert_map: HashMap<_, _> = lines
        .iter()
        // Second line is empty, irst is for directions - already parsed above
        .skip(2)
        .map(|line| parse_line(line))
        .collect();

    const START: &str = "AAA";
    const FINISH: &str = "ZZZ";

    let part1 = get_position_sequence(directions, &desert_map, START)
        .position(|x| x == FINISH)
        .unwrap() + 1;

    println!("Part 1: {}", part1);

    let starting_nodes: Vec<_> = desert_map
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|s| s.as_str())
        .collect();

    let get_cycle_length = |&start| -> usize {
        let mut iter = get_position_sequence(directions, &desert_map, start);

        // Assumption: it will terminate
        let (end1, end2) = (
            iter.position(|pos| pos.ends_with('Z')).unwrap() + 1,
            iter.position(|pos| pos.ends_with('Z')).unwrap() + 1
        );

        assert!(end1 == end2);
        assert!(end1 % directions.len() == 0);
        end1
    };

    let subcycles: Vec<_> = starting_nodes.iter().map(get_cycle_length).collect();

    let part2 = subcycles.into_iter().reduce(num_integer::lcm).unwrap();
    println!("Part 2: {}", part2);
}
