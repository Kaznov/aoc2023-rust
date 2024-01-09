#![feature(is_sorted)]

use aoc2023_rust::*;
use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::ops::Range;

/// Maps values from src_range into range (src.begin+offset..src.end+offset), other remain unchanged
struct RangeMap {
    src_range: Range<i64>,
    offset: i64
}

/// A collection of `RangeMap`s
struct RangeMapper (
    Vec<RangeMap>,
);

fn intersects(r1: &Range<i64>, r2: &Range<i64>) -> bool {
    r1.start < r2.end && r2.start < r1.end
}

fn intersection(r1: &Range<i64>, r2: &Range<i64>) -> Option<Range<i64>> {
    if !intersects(r1, r2) {
        None
    } else {
        Some(r1.start.max(r2.start)..r1.end.min(r2.end))
    }
}

fn offset_by(r: &Range<i64>, offset: i64) -> Range<i64> {
    (r.start + offset)..(r.end + offset)
}

impl RangeMapper {
    fn map_value(&self, value: i64) -> i64 {
        let matching_range =
            self.0
            .iter()
            .find(|ir| ir.src_range.contains(&value));
        if let Some(mapped_range) = matching_range {
            value + mapped_range.offset
        } else {
            value
        }
    }

    fn map_range(&self, range: &Range<i64>) -> Vec<Range<i64>> {
        let overlapping_ranges: Vec<_> =
            self.0
            .iter()
            .filter(|&mapping_range|
                intersects(&mapping_range.src_range, range))
            .collect();

        debug_assert!(overlapping_ranges.is_sorted_by_key(|r| r.src_range.start));

        let mut mapped_parts: Vec<Range<i64>> = Vec::new();
        let mut current_pos = range.start;

        for RangeMap { src_range, offset } in overlapping_ranges {
            if src_range.start > current_pos {
                mapped_parts.push(current_pos..(src_range.start));
                current_pos = src_range.start;
            }

            let intersect_part = intersection(&(current_pos..range.end), src_range).unwrap();
            mapped_parts.push(offset_by(&intersect_part, *offset));
            current_pos = intersect_part.end;
        }

        if current_pos < range.end {
            mapped_parts.push(current_pos..range.end);
        }

        mapped_parts
    }
}

fn parse_range_mapper(lines: &[String]) -> RangeMapper {
    assert!(lines[0].contains(':'));

    RangeMapper(
        lines[1..].iter()
        .map(
            |line| {
                let (dest_start, src_start, length)
                    = scan_fmt!(line, "{} {} {}", i64, i64, i64).unwrap();
                RangeMap { src_range: src_start..(src_start + length), offset: dest_start-src_start }
            }
        )
        .sorted_by_key(|mapping_range| mapping_range.src_range.start)
        .collect()
    )
}

fn main() {
    let lines = read_input_lines("input/day5.txt");
    let initial_values: Vec<i64> = lines[0]
        .strip_prefix("seeds: ").unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let groups: Vec<_> = lines
        .split(|line| line.is_empty())
        // the first section describes initial state not mappings, already parsed above
        .skip(1)
        .map(parse_range_mapper)
        .collect();

    let part1 = initial_values
        .iter()
        .map(|&v| groups.iter().fold(v, |v, g| g.map_value(v)))
        .min().unwrap();

    println!("Part 1: {}", part1);

    let initial_ranges: Vec<Range<i64>> = initial_values
        .into_iter()
        .tuples()
        .map(|(start, size)| start..(start+size))
        .collect();

    let mapped_ranges = groups
        .iter()
        .fold(initial_ranges, |ranges, mapper| {
            // map all ranges to new ranges with the current mapper
            ranges.iter().flat_map(|r| mapper.map_range(r)).collect()
        });

    let part2 = mapped_ranges
        .iter()
        .map(|r| r.start)
        .min().unwrap();

    println!("Part 2: {}", part2);
}

