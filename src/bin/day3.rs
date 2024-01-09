use aoc2023_rust::*;
use itertools::Itertools;

// Describes a position of a number in the board.
// The ranges include the border around the number.
// Example:
// It could describe the position of the following region (and number's value):
// .....
// .123.
// .....
struct NumberInGrid {
    x_range: std::ops::RangeInclusive<isize>,
    y_range: std::ops::RangeInclusive<isize>,
    value: u32
}

struct SymbolInGrid {
    x: isize,
    y: isize,
    symbol: char
}

impl NumberInGrid {
    fn new(value: u32, x_end: isize, y: isize) -> Self {
        let len = value.to_string().len() as isize;
        let x_range = (x_end - len - 1)..=(x_end);
        let y_range = (y - 1)..=(y + 1);
        NumberInGrid { x_range, y_range, value }
    }

    fn touches_symbol(&self, symbol: &SymbolInGrid) -> bool {
        self.x_range.contains(&symbol.x) &&
        self.y_range.contains(&symbol.y)
    }
}

fn parse_grid(lines: &[String]) -> (Vec<NumberInGrid>, Vec<SymbolInGrid>) {
    let mut numbers: Vec<NumberInGrid> = Vec::new();
    let mut symbols: Vec<SymbolInGrid> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        let mut next_number = 0;
        for (x, c) in line.chars().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                next_number *= 10;
                next_number += digit;
            }
            else {
                if next_number > 0 {
                    numbers.push(NumberInGrid::new(next_number, x as isize, y as isize));
                    next_number = 0;
                }

                if c != '.' {
                    symbols.push(SymbolInGrid{ x: x as isize, y: y as isize, symbol: c })
                }
            }
        }

        if next_number > 0 {
            numbers.push(NumberInGrid::new(next_number,line.len() as isize, y as isize));
        }
    }

    (numbers, symbols)
}

fn main() {
    let lines = read_input_lines("input/day3.txt");
    let (numbers, symbols) = parse_grid(&lines);

    let part1: u32 = numbers
        .iter()
        .filter(|number| {
            symbols.iter().any(|s| number.touches_symbol(s))
        })
        .map(|number| number.value)
        .sum();

    println!("Part1: {}", part1);

    let part2: u32 = symbols.iter()
        .filter(|s| s.symbol == '*')
        .filter_map(|s| {
            let touching = numbers
                .iter()
                .filter(|n| n.touches_symbol(s))
                .collect_vec();
            if let &[first, second] = &touching[..] {
                Some(first.value * second.value)
            } else {
                None
            }
        })
        .sum();

    println!("Part2: {}", part2);
}

