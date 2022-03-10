use super::{diagonal_neighborhood, get_number_matrix_input, matrix_iterator};

const ACTIVATED: u8 = 255;
const ACTIVATION_THRESHOLD: u8 = 10;

pub fn day11() {
    let mut input = get_number_matrix_input(11);
    let boundaries = [input.len(), input[0].len()];
    let mut result1 = 0;
    let mut result2 = 0;

    for i in 0.. {
        input.iter_mut().for_each(|r| {
            r.iter_mut()
                .for_each(|v| *v = if *v == ACTIVATED { 1 } else { *v + 1 })
        });

        let mut activated = 0;
        matrix_iterator(boundaries[0], boundaries[1])
            .for_each(|(x, y)| activated += activate(&mut input, &[x, y], &boundaries));

        if i < 100 {
            result1 += activated;
        }

        if i >= 100 && activated == (boundaries[0] * boundaries[1]) as u32 {
            result2 = i + 1;
            break;
        }
    }

    println!("DAY 11\nSolution 1: {}\nSolution 2: {}", result1, result2);
}

fn activate(matrix: &mut Vec<Vec<u8>>, coord: &[usize; 2], boundaries: &[usize; 2]) -> u32 {
    let v = &mut matrix[coord[0]][coord[1]];

    if *v >= ACTIVATION_THRESHOLD && *v != ACTIVATED {
        *v = ACTIVATED;

        let n = diagonal_neighborhood(&coord, boundaries);
        n.iter().for_each(|[x, y]| {
            if matrix[*x][*y] != ACTIVATED {
                matrix[*x][*y] += 1
            }
        });

        let mut activated = 0;
        n.iter()
            .for_each(|c| activated += activate(matrix, &c, boundaries));

        activated + 1
    } else {
        0
    }
}
