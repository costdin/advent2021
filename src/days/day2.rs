use super::get_input_lines;

pub fn day2() {
    let lines = get_input_lines(2);

    let instructions = lines
        .into_iter()
        .map(|l| l.into())
        .map(|l: SubmarineDirection| l.into())
        .collect::<Vec<_>>();

    let (h, d) = instructions
        .iter()
        .fold((0, 0), |(h1, d1), (h2, d2)| (h1 + h2, d1 + d2));

    let (_, h2, d2) = instructions
        .iter()
        .fold((0, 0, 0), |(aim, h, d), (f, a_change)| {
            (aim + a_change, h + f, d + (aim + a_change) * f)
        });

    println!("DAY 2\nSolution 1: {}\nSolution 2: {}", h * d, h2 * d2);
}

enum SubmarineDirection {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Into<(i32, i32)> for SubmarineDirection {
    fn into(self) -> (i32, i32) {
        match self {
            SubmarineDirection::Forward(v) => (v as i32, 0),
            SubmarineDirection::Down(v) => (0, v as i32),
            SubmarineDirection::Up(v) => (0, -(v as i32)),
        }
    }
}

impl From<String> for SubmarineDirection {
    fn from(input: String) -> Self {
        if let [d, v] = input.split(' ').collect::<Vec<_>>()[..] {
            match d {
                "forward" => SubmarineDirection::Forward(v.parse::<u32>().unwrap()),
                "down" => SubmarineDirection::Down(v.parse::<u32>().unwrap()),
                "up" => SubmarineDirection::Up(v.parse::<u32>().unwrap()),
                _ => unreachable!(),
            }
        } else {
            panic!("Invalid input");
        }
    }
}
