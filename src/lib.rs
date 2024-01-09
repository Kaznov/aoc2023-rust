#![allow(clippy::reversed_empty_ranges)]

use std::fs;
use itertools::Itertools;
use ndarray::{Array2, ArrayView2, azip, s};

pub fn read_input_lines(input_path: &str) -> Vec<String> {
    fs::read_to_string(input_path)
        .unwrap_or_else(|err| panic!("Could not open the file {}, {}", input_path, err))
        .lines()
        .map(&String::from)
        .collect()
}

pub fn read_2d_board(input_path: &str) -> Array2<u8> {
    let lines = read_input_lines(input_path);
    let width = lines[0].len();
    let height = lines.len();
    let input = lines.join("").into_bytes();

    Array2::from_shape_vec((height, width), input).unwrap()
}

pub fn print_ascii_board(board: ArrayView2<u8>) {
    let board_string = board
        .rows()
        .into_iter()
        .map(|row| row
            .iter()
            .map(|c| *c as char)
            .join(""))
        .join("\n");
    println!("{}", board_string);
}

pub fn print_board<T>(board: ArrayView2<T>)
    where T: std::fmt::Display
{
    let widest_number = board.iter().map(|x| format!("{x:.}").len()).max().unwrap();
    for row in board.rows() {
        for element in row {
            print!("{: >width$.}", element, width = (widest_number+1));
        }
        println!()
    }
}

pub fn frame_board<T>(board: ArrayView2<T>, fill: T) -> Array2<T>
    where T: Clone
{
    let dim = board.dim();
    let mut result = Array2::from_elem((dim.0 + 1, dim.1 + 1), fill);
    let mut result_inner = result.slice_mut(s![1..-1, 1..-1]);
    azip!((r in &mut result_inner, e in &board) *r = e.clone());
    result
}
