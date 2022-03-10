use std::fs::File;
use std::io::{BufRead, BufReader};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub use day1::day1;
pub use day10::day10;
pub use day11::day11;
pub use day12::day12;
pub use day13::day13;
pub use day14::day14;
pub use day2::day2;
pub use day3::day3;
pub use day4::day4;
pub use day5::day5;
pub use day6::day6;
pub use day7::day7;
pub use day8::day8;
pub use day9::day9;

fn split_collection(stack: &[u32], f: &dyn Fn(u32) -> bool) -> (Vec<u32>, Vec<u32>) {
    stack
        .iter()
        .fold((vec![], vec![]), |(mut trues, mut falses), e| {
            if f(*e) {
                trues.push(*e)
            } else {
                falses.push(*e)
            }

            (trues, falses)
        })
}

fn get_input_lines(day: u32) -> Vec<String> {
    let file = File::open(format!("day{}.txt", day)).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

fn get_number_matrix_input(day: u32) -> Vec<Vec<u8>> {
    get_input_lines(day)
        .iter()
        .map(|s| s.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn range_inclusive(start: u32, end: u32) -> Vec<u32> {
    if start < end {
        (start..end + 1).collect()
    } else {
        (end..start + 1).rev().collect()
    }
}

fn matrix_iterator(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..x).map(move |x| (0..y).map(move |y| (x, y))).flatten()
}

fn neighborhood(coord: &[usize; 2], boundaries: &[usize; 2]) -> Vec<[usize; 2]> {
    match (coord, boundaries) {
        /* Corners */
        ([0, 0], _) => vec![[0, 1], [1, 0]],
        ([0, y], [_, yb]) if y == &(yb - 1) => vec![[1, *y], [0, y - 1]],
        ([x, 0], [xb, _]) if x == &(xb - 1) => vec![[*x, 1], [x - 1, 0]],
        ([x, y], [xb, yb]) if x == &(xb - 1) && y == &(yb - 1) => vec![[x - 1, *y], [*x, y - 1]],
        /* Corners */

        /* Borders */
        ([0, y], _) => vec![[1, *y], [0, y - 1], [0, y + 1]],
        ([x, 0], _) => vec![[*x, 1], [x - 1, 0], [x + 1, 0]],
        ([x, y], [_, yb]) if y == &(yb - 1) => vec![[x + 1, *y], [x - 1, *y], [*x, y - 1]],
        ([x, y], [xb, _]) if x == &(xb - 1) => vec![[*x, y + 1], [*x, y - 1], [x - 1, *y]],
        /* Borders */
        ([x, y], _) => vec![[x - 1, *y], [x + 1, *y], [*x, y - 1], [*x, y + 1]],
    }
}

fn diagonal_neighborhood(coord: &[usize; 2], boundaries: &[usize; 2]) -> Vec<[usize; 2]> {
    match (coord, boundaries) {
        /* Corners */
        ([0, 0], _) => vec![[0, 1], [1, 0], [1, 1]],
        ([0, y], [_, yb]) if y == &(yb - 1) => vec![[1, *y], [0, y - 1], [1, y - 1]],
        ([x, 0], [xb, _]) if x == &(xb - 1) => vec![[*x, 1], [x - 1, 0], [x - 1, 1]],
        ([x, y], [xb, yb]) if x == &(xb - 1) && y == &(yb - 1) => {
            vec![[x - 1, *y], [*x, y - 1], [x - 1, y - 1]]
        }
        /* Corners */

        /* Borders */
        ([0, y], _) => vec![[1, *y], [0, y - 1], [0, y + 1], [1, y - 1], [1, y + 1]],
        ([x, 0], _) => vec![[*x, 1], [x - 1, 0], [x + 1, 0], [x - 1, 1], [x + 1, 1]],
        ([x, y], [_, yb]) if y == &(yb - 1) => vec![
            [x + 1, *y],
            [x - 1, *y],
            [*x, y - 1],
            [x + 1, y - 1],
            [x - 1, y - 1],
        ],
        ([x, y], [xb, _]) if x == &(xb - 1) => vec![
            [*x, y + 1],
            [*x, y - 1],
            [x - 1, *y],
            [x - 1, y + 1],
            [x - 1, y - 1],
        ],
        /* Borders */
        ([x, y], _) => vec![
            [x - 1, *y],
            [x + 1, *y],
            [*x, y - 1],
            [*x, y + 1],
            [x - 1, y - 1],
            [x - 1, y + 1],
            [x + 1, y - 1],
            [x + 1, y + 1],
        ],
    }
}
