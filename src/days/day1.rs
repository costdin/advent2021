use super::get_input_lines;

pub fn day1() {
    let lines = get_input_lines(1);

    let values = lines
        .iter()
        .flat_map(|l| l.parse::<i32>())
        .collect::<Vec<_>>();

    let count = values
        .iter()
        .zip(values.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count();

    let count_window = values
        .iter()
        .zip(values.iter().skip(3))
        .filter(|(a, b)| a < b)
        .count();

    println!("DAY 1\nSolution 1: {}\nSolution 2: {}", count, count_window);
}
