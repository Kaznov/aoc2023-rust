use aoc2023_rust::*;

fn line_to_win_count(line: &str) -> usize {
    let line = line.split_once(':').unwrap().1;
    let (left, right) = line.split_once('|').unwrap();

    let parse_list_of_numbers = |s: &str| -> Vec<usize> {
        s
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
    };

    let nums_left = parse_list_of_numbers(left);
    let nums_right = parse_list_of_numbers(right);

    nums_left.iter().filter(|x| nums_right.contains(x)).count()
}

fn main() {
    let win_counts: Vec<_> =
        read_input_lines("input/day4.txt")
        .iter()
        .map(|s| line_to_win_count(s))
        .collect();

    let part1: usize = win_counts
        .iter()
        .map(|&win_count| if win_count > 0 { 1 << (win_count - 1) } else { 0 }).sum();
    println!("Part 1: {}", part1);

    let mut cards_count = vec![1usize; win_counts.len()];

    for (i, win_count) in win_counts.iter().enumerate() {
        for j in (i + 1)..(i + 1 + win_count) {
            cards_count[j] += cards_count[i]
        }
    }

    let part2: usize = cards_count.iter().sum();
    println!("Part 2: {}", part2);
}
