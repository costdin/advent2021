use std::ops::Add;

use super::{get_input_lines, split_collection};

pub fn day3() {
    let lines = get_input_lines(3);

    let sum = lines
        .iter()
        .map(|l| l.into())
        .fold(DiagnosticsReading::zero(), |acc, n| acc + n);

    let (gamma, epsilon) = sum.finalize();

    let numbers = lines
        .iter()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

    // splitting the collection here, we save one iteration
    let bit_position = sum.reading.len() as u32 - 1;
    let mask = 1 << bit_position;
    let (high, low) = sort_by_len(split_collection(&numbers, &|e| (e & mask) == 0)); // try using numbers.iter().partition()?

    let oxygen = search_rating(&high, mask >> 1, true);
    let co2 = search_rating(&low, mask >> 1, false);

    println!(
        "DAY 3\nSolution 1: {}\nSolution 2: {}",
        gamma * epsilon,
        oxygen * co2
    );
}

fn search_rating(stack: &[u32], mask: u32, is_oxygen: bool) -> u32 {
    if stack.len() == 1 {
        return stack[0];
    }

    let (high, low) = sort_by_len(split_collection(stack, &|e| (e & mask) > 0));
    let next_stack = if is_oxygen { high } else { low };

    search_rating(&next_stack, mask >> 1, is_oxygen)
}

fn sort_by_len(vs: (Vec<u32>, Vec<u32>)) -> (Vec<u32>, Vec<u32>) {
    let (v1, v2) = vs;

    if v1.len() >= v2.len() {
        (v1, v2)
    } else {
        (v2, v1)
    }
}

struct DiagnosticsReading {
    reading: Vec<u32>,
    count: u32,
}

impl DiagnosticsReading {
    fn finalize(&self) -> (u32, u32) {
        let gamma = self.reading.iter().fold(0, |acc, r| {
            (acc << 1) + if r > &(self.count / 2) { 1 } else { 0 }
        });

        let mask = 2u32.pow(self.reading.len() as u32) - 1;
        (gamma, (!gamma) & mask)
    }

    fn zero() -> Self {
        DiagnosticsReading {
            reading: vec![],
            count: 0,
        }
    }
}

impl Add for DiagnosticsReading {
    type Output = Self;

    fn add(self, d2: Self) -> Self {
        if self.reading.len() == 0 {
            d2
        } else if d2.reading.len() == 0 {
            self
        } else {
            let reading = self
                .reading
                .iter()
                .zip(d2.reading)
                .map(|(a, b)| a + b)
                .collect();

            DiagnosticsReading {
                reading,
                count: self.count + d2.count,
            }
        }
    }
}

impl From<&String> for DiagnosticsReading {
    fn from(input: &String) -> Self {
        let reading = input
            .chars()
            .map(|c| match c {
                '0' => 0,
                '1' => 1,
                _ => unreachable!(),
            })
            .collect();

        DiagnosticsReading { reading, count: 1 }
    }
}
