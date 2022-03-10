use super::{get_number_matrix_input, matrix_iterator, neighborhood};

pub fn day9() {
    let mut input = get_number_matrix_input(9);

    let len = input.len();
    let height = input[0].len();

    let result1 = matrix_iterator(len, height)
        .filter(|(x, y)| is_low_point(&input, [*x, *y]))
        .map(|(x, y)| input[x][y] as u16 + 1)
        .sum::<u16>();

    let mut basins = matrix_iterator(len, height)
        .map(|(x, y)| basin(&mut input, [x, y]))
        .filter(|b| b.len() > 0)
        .map(|b| b.iter().len() as u32)
        .collect::<Vec<_>>();

    basins.sort();

    let result2 = basins.iter().rev().take(3).fold(1u32, |v, acc| acc * v);

    println!("DAY 9\nSolution 1: {}\nSolution 2: {}", result1, result2);
}

fn is_low_point(readings: &Vec<Vec<u8>>, coord: [usize; 2]) -> bool {
    let r = readings[coord[0]][coord[1]];

    neighborhood(&coord, &[readings.len(), readings[0].len()])
        .iter()
        .all(|n| r < readings[n[0]][n[1]])
}

fn basin(readings: &mut Vec<Vec<u8>>, coord: [usize; 2]) -> Vec<u8> {
    let accumulator = vec![];

    basin_internal(readings, coord, accumulator)
}

fn basin_internal(
    readings: &mut Vec<Vec<u8>>,
    coord: [usize; 2],
    mut accumulator: Vec<u8>,
) -> Vec<u8> {
    let r = readings[coord[0]][coord[1]];

    if r == 9 {
        accumulator
    } else {
        readings[coord[0]][coord[1]] = 9;
        accumulator.push(r);

        for neighbor in neighborhood(&coord, &[readings.len(), readings[0].len()]) {
            accumulator = basin_internal(readings, neighbor, accumulator);
        }

        accumulator
    }
}
