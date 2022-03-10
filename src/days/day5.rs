use super::{get_input_lines, range_inclusive};
use std::collections::HashMap;

pub fn day5() {
    let lines = get_input_lines(5);

    let vents = lines
        .into_iter()
        .filter_map(Vent::parse)
        .collect::<Vec<_>>();

    let straight_result = vents
        .iter()
        .filter(|v| !v.diagonal)
        .fold(Vent::empty(), |acc, n| acc.merge(n));
    let diagonal_result = vents.iter().fold(Vent::empty(), |acc, n| acc.merge(n));

    println!(
        "DAY 5\nSolution 1: {}\nSolution 2: {}",
        straight_result.count_overlaps(),
        diagonal_result.count_overlaps(),
    );
}

#[derive(Debug)]
struct Vent {
    coordinates: HashMap<(u32, u32), u32>,
    diagonal: bool,
}

impl Vent {
    fn empty() -> Vent {
        Vent {
            coordinates: HashMap::new(),
            diagonal: false,
        }
    }

    fn parse(input: String) -> Option<Vent> {
        if let [beginning, end] = &input
            .split(" -> ")
            .map(|s| {
                s.split(',')
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()[..]
        {
            if beginning[0] == end[0] {
                Some(Vent {
                    coordinates: range_inclusive(beginning[1], end[1])
                        .into_iter()
                        .map(|v| ((beginning[0], v), 1))
                        .collect(),
                    diagonal: false,
                })
            } else if beginning[1] == end[1] {
                Some(Vent {
                    coordinates: range_inclusive(beginning[0], end[0])
                        .into_iter()
                        .map(|v| ((v, beginning[1]), 1))
                        .collect(),
                    diagonal: false,
                })
            } else {
                let r1 = range_inclusive(beginning[0], end[0]);
                let r2 = range_inclusive(beginning[1], end[1]);

                if r1.len() == r2.len() {
                    Some(Vent {
                        coordinates: r1.into_iter().zip(r2.into_iter()).map(|c| (c, 1)).collect(),
                        diagonal: true,
                    })
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    fn merge(mut self, other: &Vent) -> Vent {
        for (c, v) in other.coordinates.iter() {
            if let Some(sv) = self.coordinates.get_mut(c) {
                *sv += v;
            } else {
                self.coordinates.insert(*c, *v);
            }
        }

        self
    }

    fn count_overlaps(&self) -> u32 {
        self.coordinates.iter().filter(|(_, v)| v > &&1u32).count() as u32
    }
}
