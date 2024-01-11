use itertools::Itertools;
use ndarray::{Array2, ArrayView2, ArrayView1};
use simple_scan::IteratorSimpleScanExt;

struct Neighbours {
    up:    [usize; 2],
    down:  [usize; 2],
    left:  [usize; 2],
    right: [usize; 2],
}

fn neighbours_of([y, x]: [usize; 2]) -> Neighbours {
    Neighbours {
        up:    [y.wrapping_sub(1), x], // [y+1, x  ]
        down:  [y.wrapping_add(1), x], // [y-1, x  ]
        left:  [y, x.wrapping_sub(1)], // [y,   x-1]
        right: [y, x.wrapping_add(1)], // [y,   x+1]
    }
}

fn pipe_connections(board: ArrayView2<u8>, pipe_position: [usize; 2])
        -> ([usize; 2], [usize; 2]) {
    let Neighbours { up, down, left, right } = neighbours_of(pipe_position);

    match board.get(pipe_position).unwrap() {
        b'|' => (up, down),
        b'-' => (left, right),
        b'L' => (up, right),
        b'J' => (up, left),
        b'7' => (left, down),
        b'F' => (down, right),
        _ => panic!("Unexpeted non-pipe character")
    }
}

fn connections_to_pipe(c1: [usize; 2], c2: [usize; 2]) -> u8 {
    let [y, x] = [(c1[0] + c2[0]) / 2, (c1[1] + c2[1]) / 2];
    let Neighbours {up, down, left, right} = neighbours_of([y, x]);

    if      [c1, c2] == [up, down]    || [c1, c2] == [down, up]    { b'|' }
    else if [c1, c2] == [up, left]    || [c1, c2] == [left, up]    { b'J' }
    else if [c1, c2] == [up, right]   || [c1, c2] == [right, up]   { b'L' }
    else if [c1, c2] == [down, left]  || [c1, c2] == [left, down]  { b'7' }
    else if [c1, c2] == [down, right] || [c1, c2] == [right, down] { b'F' }
    else if [c1, c2] == [left, right] || [c1, c2] == [right, left] { b'-' }
    else { panic!("The two positions should be adjacent to the same block") }
}

fn take_the_other<T: PartialEq>(pair: (T, T), element: T) -> T {
    if pair.0 == element { pair. 1 }
    else if pair.1 == element { pair. 0 }
    else { panic!("Element was not part of the pair") }
}

fn next_pipe_position(board: ArrayView2<u8>,
                      curr_pipe: [usize; 2],
                      prev_pipe: [usize; 2]) -> [usize; 2] {
    let connections = pipe_connections(board, curr_pipe);
    take_the_other(connections, prev_pipe)
}

fn get_starting_pipes(board: ArrayView2<u8>, start: [usize; 2])
        -> [[usize; 2]; 2] {
    assert!(board[start] == b'S');

    let Neighbours {up, down, left, right} = neighbours_of(start);

    let connected_positions: Vec<_> = [up, down, left, right]
        .into_iter()
        .filter(|pos| {
            // skip indices outside of the board, skip non-pipes
            board.get(*pos).is_some_and(|field| *field != b'.')
        })
        .filter(|pos| {
            // take the pipes that connect to the start
            let (p1, p2) = pipe_connections(board.view(), *pos);
            p1 == start || p2 == start
        })
        .collect();

    connected_positions.try_into().unwrap()
}

fn scan_board_line_for_inner_part(line: ArrayView1<u8>) -> usize {
    // https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm

    #[derive(Clone, Copy, PartialEq)]
    enum ScanState {
        Inside,
        Outside,
        BorderAboveFilled,
        BorderBelowFilled
    }
    use ScanState::*;

    fn next_state(state: ScanState, c: u8) -> ScanState {
        match (state, c) {
            (Outside, b'|')            => Inside,
            (Outside, b'L')            => BorderAboveFilled,
            (Outside, b'F')            => BorderBelowFilled,
            (Outside, b'.')            => Outside,
            (Inside, b'|')             => Outside,
            (Inside, b'L')             => BorderBelowFilled,
            (Inside, b'F')             => BorderAboveFilled,
            (Inside, b'.')             => Inside,

            (BorderAboveFilled, b'-' ) => BorderAboveFilled,
            (BorderAboveFilled, b'J' ) => Outside,
            (BorderAboveFilled, b'7' ) => Inside,
            (BorderBelowFilled, b'-' ) => BorderBelowFilled,
            (BorderBelowFilled, b'J' ) => Inside,
            (BorderBelowFilled, b'7' ) => Outside,

            (_, _) => panic!("Unexpected state transition")
        }
    }

    let inout = line.iter()
        .trace(Outside, |&s, &c| next_state(s, c))
        .collect_vec();

    // we count '.'s marked inside
    inout.iter().zip(line.iter())
        .filter(|&(state, _)| *state == Inside)
        .filter(|&(_, c)| *c == b'.')
        .count()
}

fn main() {
    let input = aoc2023_rust::read_input_lines("input/day10.txt");
    let width = input[0].len();
    let height = input.len();
    let input = input.join("").into_bytes();

    // 2D board with pipes
    let mut board = Array2::from_shape_vec((height, width), input)
        .expect("Board dimensions should match");
    // 2D board with boolean flags, is this field part of the loop we are looking for?
    let mut board_is_loop = Array2::from_elem(board.raw_dim(), false);

    // Position of S on the board
    let start: [usize; 2] = board.indexed_iter().find(|(_, c)| **c == b'S' ).unwrap().0.into();

    // Pipes connected to S
    let start_pipes = get_starting_pipes(board.view(), start);
    board[start] = connections_to_pipe(start_pipes[0], start_pipes[1]);

    let move_to_next_pipe =
        |(prev, curr)|
            (
                curr,
                next_pipe_position(board.view(), curr, prev)
            );

    board_is_loop[start] = true;
    let mut counter = 1;
    let mut direction = (start, start_pipes[0]);

    while direction.1 != start {
        board_is_loop[direction.1] = true;
        direction = move_to_next_pipe(direction);
        counter += 1;
    }

    println!("Part 1: {}", counter / 2);

    // Replace all elements that are not part of the loop with b'.'
    ndarray::Zip::from(&mut board)
        .and(&board_is_loop)
        .for_each(|board_field, is_loop|
            if !*is_loop { *board_field = b'.' } );

    let part2: usize = board
        .rows()
        .into_iter()
        .map(scan_board_line_for_inner_part)
        .sum();

    println!("Part 2: {}", part2);
}
