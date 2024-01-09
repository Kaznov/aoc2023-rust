use aoc2023_rust::read_input_lines;
use itertools::Itertools;

fn parse_list(s: &str) -> Vec<f64> {
    s
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

/// Number of integral solutions for `a*x*x + b*x + c > 0`
fn quadratic_integral_solutions_count(a: f64, b: f64, c: f64) -> u64 {
    let delta = b * b - 4. * a * c;

    if delta < 0. { return 0; }

    let x1 = (-b + delta.sqrt()) / (2. * a); 
    let x2 = (-b - delta.sqrt()) / (2. * a);
    let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };

    // this ceil-floor is specifically for > strict inequality
    // extends (x1, x2) range to integral bounds, -1. removes <= sides
    (x2.ceil() - x1.floor() - 1.) as u64 
}

fn main() {
    let lines = read_input_lines("input/day6.txt");
    
    let line1 = lines[0].strip_prefix("Time:").unwrap();
    let line2 = lines[1].strip_prefix("Distance:").unwrap();
    
    let times: Vec<f64> = parse_list(line1);
    let distances: Vec<f64> = parse_list(line2);

    let part1: u64 = times.iter().zip(distances.iter())
        .map(|(&time, &distance)| {
            // x * (t - x) > distance
            // -xx + xt - dist > 0
            let a = -1.;
            let b = time;
            let c = -distance;
            quadratic_integral_solutions_count(a, b, c)
    }).product();

    println!("Part 1: {}", part1);

    let parse_line_as_number = |line: &str| {
        line.chars().filter(|c|c.is_ascii_digit()).join("").parse().unwrap()
    };

    let time2 = parse_line_as_number(line1);
    let distance2 = parse_line_as_number(line2);

    let part2 = quadratic_integral_solutions_count(-1., time2, -distance2);
    println!("Part 2: {}", part2);
}
