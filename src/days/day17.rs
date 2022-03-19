use super::{div_roundup, get_input_lines};
use std::collections::HashSet;
use std::iter::empty;

pub fn day17() {
    let line = get_input_lines(17).into_iter().nth(0).unwrap();
    let area = parse(line).unwrap();

    let solution1 = area.y[0] * (area.y[0] + 1) / 2;

    let min_x = ((((8 * area.x[0] + 1) as f32).sqrt() + 1.0) / 2.0).floor() as i32;
    let max_asymptotic_x = ((((8 * area.x[1] - 1) as f32).sqrt() - 1.0) / 2.0).floor() as i32;

    let solution2 = (area.y[0]..(solution1 + 1))
        .map(|y| (y, y_interceptions(y, area.y)))
        .flat_map(|(y, i)| x_ranges(&area, &i, min_x, max_asymptotic_x).map(move |x| (x, y)))
        .count();

    println!(
        "DAY 17\nSolution 1: {}\nSolution 2: {}",
        solution1,
        solution2,
    );
}

fn y_interceptions(y: i32, range: [i32; 2]) -> Vec<i32> {
    let mut result = vec![];

    let (mut i, mut pos) = if y > 0 {
        (1 + 2 * y, 0)
    } else {
        (1, y)
    };

    while pos >= range[0] {
        if pos <= range[1] {
            result.push(i);
        }

        pos += y - i;

        i += 1;
    }

    result
}

fn x_range(range: &[i32; 2], steps: i32, min_x: i32, max_asymptotic_x: i32) -> impl Iterator<Item = i32> {
    if steps == 1 {
        range[0]..=range[1]
    } else if steps > min_x {
        min_x..=max_asymptotic_x
    } else {
        let aaa = steps * (steps - 1) / 2;
        let start = div_roundup(range[0] + aaa, steps);
        let end = (range[1] + aaa) / steps;

        start..=end
    }
}

fn x_ranges(
    area: &Area,
    steps: &[i32],
    min_x: i32,
    max_asymptotic_x: i32,
) -> Box<dyn Iterator<Item = i32>> {
    if steps.len() == 0 {
        return Box::new(empty())
    } else if steps.len() == 1 {
        return Box::new(x_range(&area.x, steps[0], min_x, max_asymptotic_x));
    } else {
        return Box::new(
            steps
                .iter()
                .flat_map(|&v| x_range(&area.x, v, min_x, max_asymptotic_x))
                .collect::<HashSet<_>>()
                .into_iter(),
        );
    }
}

struct Area {
    x: [i32; 2],
    y: [i32; 2],
}

fn parse(line: String) -> Result<Area, ()> {
    if let [x, y] = &line[15..].split(',').collect::<Vec<_>>()[..] {
        let v1 = x
            .split("..")
            .flat_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>();
        let v2 = y[3..]
            .split("..")
            .flat_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>();

        match (v1, v2) {
            (v1, v2) if v1.len() == 2 && v2.len() == 2 => Ok(Area {
                x: [v1[0].min(v1[1]), v1[0].max(v1[1])],
                y: [v2[0].min(v2[1]), v2[0].max(v2[1])],
            }),
            _ => Err(()),
        }
    } else {
        Err(())
    }
}
