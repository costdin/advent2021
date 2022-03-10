use super::get_input_lines;

pub fn day6() {
    let lines = get_input_lines(6);
    let fishes = lines[0]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let offspring_count_80 = [1421, 1401, 1191, 1154, 1034, 950, 905, 779, 768];

    let offspring_count_256 = [
        6703087164, 6206821033, 5617089148, 5217223242, 4726100874, 4368232009, 3989468462,
        3649885552, 3369186778,
    ];

    let result80: usize = fishes.iter().map(|f| offspring_count_80[*f]).sum();
    let result256: usize = fishes.iter().map(|f| offspring_count_256[*f]).sum();

    println!("DAY 6\nSolution 1: {}\nSolution 2: {}", result80, result256);
}

#[allow(dead_code)]
fn print_offspring_counts(generations: usize) {
    let mut results = [0u128; 9];

    for i in 0..9 {
        let mut f = [0u128; 9];
        f[i] = 1;

        for _ in 0..generations {
            let mut gg = [0u128; 9];
            gg[8] = f[0];
            gg[7] = f[8];
            gg[6] = f[7] + f[0];
            gg[5] = f[6];
            gg[4] = f[5];
            gg[3] = f[4];
            gg[2] = f[3];
            gg[1] = f[2];
            gg[0] = f[1];

            f = gg;
        }

        results[i] = f.iter().sum::<u128>();
    }

    println!("{:#?}", results);
}
