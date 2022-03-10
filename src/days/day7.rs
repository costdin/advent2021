use super::get_input_lines;

pub fn day7() {
    let lines = get_input_lines(7);
    let mut crabs = lines[0]
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // The first solution is the median
    crabs.sort();
    let median = crabs[crabs.len() / 2];
    let result1 = calculate_fuel_linear(&crabs, median);

    // The second solution is the average
    let average = crabs.iter().sum::<i32>() as f32 / crabs.len() as f32;
    let result2 = calculate_fuel_increasing(&crabs, average.floor() as i32)
        .min(calculate_fuel_increasing(&crabs, average.ceil() as i32));

    println!("DAY 7\nSolution 1: {}\nSolution 2: {}", result1, result2);
}

pub fn calculate_fuel_linear(crabs: &[i32], solution: i32) -> i32 {
    crabs.iter().map(|c| (c - solution).abs()).sum()
}

pub fn calculate_fuel_increasing(crabs: &[i32], solution: i32) -> i32 {
    crabs
        .iter()
        .map(|c| (c - solution).abs())
        .map(|c| c * (c + 1) / 2)
        .sum()
}
