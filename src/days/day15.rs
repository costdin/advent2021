use super::{get_number_matrix_input, neighborhood};
use std::cmp::{min, Reverse};
use std::collections::{BinaryHeap, HashSet};

pub fn day15() {
    let lines = get_number_matrix_input(15);
    let destination = [lines.len() - 1, lines[0].len() - 1];
    let result = spf(&lines, destination);

    let destination2 = [lines.len() * 5 - 1, lines[0].len() * 5 - 1];
    let result2 = spf(&lines, destination2);

    println!("DAY 15\nSolution 1: {}\nSolution 2: {}", result, result2);
}

fn spf(nodes: &Vec<Vec<u8>>, destination: [usize; 2]) -> u32 {
    let mut visited = HashSet::<[usize; 2]>::new();
    visited.insert([0, 0]);
    let boundaries = [destination[0] + 1, destination[1] + 1];

    let mut frontier_index = HashSet::<[usize; 2]>::new();
    let mut frontier: BinaryHeap<Reverse<(u32, [usize; 2])>> = neighborhood(&[0, 0], &boundaries)
        .into_iter()
        .map(|n| (get_node(nodes, &n), n))
        .map(Reverse)
        .collect();

    loop {
        let Reverse((next_weight, next)) = frontier.pop().unwrap();
        visited.insert(next);

        if destination == next {
            return next_weight;
        } else {
            for n in neighborhood(&next, &boundaries)
                .into_iter()
                .filter(|n| !visited.contains(n))
                .filter(|n| frontier_index.insert(*n))
            {
                frontier.push(Reverse((get_node(nodes, &n) + next_weight, n)));
            }
        }
    }
}

fn get_node(nodes: &Vec<Vec<u8>>, ix: &[usize; 2]) -> u32 {
    if ix[0] < nodes.len() && ix[1] < nodes[0].len() {
        nodes[ix[0]][ix[1]] as u32
    } else {
        let add = ix[0] / nodes.len() + ix[1] / nodes[0].len();
        let x = ix[0] % nodes.len();
        let y = ix[1] % nodes[0].len();
        let value = (nodes[x][y] as u32 + add as u32) % 9;

        if value == 0 {
            9
        } else {
            value
        }
    }
}
