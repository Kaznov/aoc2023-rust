use ndarray::{Array2, ArrayView1};

fn distances_after_expansion(line_counts: &[u64], expansion_rate: u64) -> u64 {
    let mut acc = 0;
    let mut distance_to_prev_sum = 0;
    let mut count_prev = 0;

    // find distances each-to-each, line_counts describe number of elements
    // at given position.
    // If == 0, this line expands according to expansion_rate
    for &next_count in line_counts {
        // distances from the ones in the current line to the previous ones
        acc += next_count * distance_to_prev_sum;

        // number of previous ones
        count_prev += next_count;

        // cumulative distance to all the previous ones
        if next_count == 0 {
            distance_to_prev_sum += expansion_rate * count_prev
        } else {
            distance_to_prev_sum += count_prev
        }
    }

    acc
}

fn main() {
    let input = aoc2023_rust::read_input_lines("input/day11.txt");
    let width = input[0].len();
    let height = input.len();
    let input = input.join("").into_bytes();

    let board = Array2::from_shape_vec((height, width), input).unwrap();

    let sum_line =
        |line: ArrayView1<u8>| line.iter().filter(|&&c| c == b'#').count() as u64;

    let rows_counts: Vec<_> = board.rows()   .into_iter().map(sum_line).collect();
    let cols_counts: Vec<_> = board.columns().into_iter().map(sum_line).collect();

    let row_distances = distances_after_expansion(&rows_counts, 2);
    let col_distances = distances_after_expansion(&cols_counts, 2);

    println!("Part 1: {}", row_distances + col_distances);

    let row_distances2 = distances_after_expansion(&rows_counts, 1_000_000);
    let col_distances2 = distances_after_expansion(&cols_counts, 1_000_000);

    println!("Part 2: {}", row_distances2 + col_distances2);
}
