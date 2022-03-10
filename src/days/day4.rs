use super::get_input_lines;

pub fn day4() {
    let lines = get_input_lines(4);

    let draws = lines[0].split(',').map(|n| n.parse::<u32>().unwrap());

    let mut boards = lines
        .iter()
        .skip(1)
        .filter(|l| l != &"")
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|c| Board::parse(c).unwrap())
        .collect::<Vec<_>>();

    let mut won = vec![];
    for draw in draws {
        for board in boards.iter_mut().filter(|b| !b.has_won) {
            if board.check_number(draw) {
                won.push(board.sum_of_remaining() * draw);
            }
        }
    }

    println!(
        "DAY 4\nSolution 1: {}\nSolution 2: {}",
        won[0],
        won.last().unwrap(),
    );
}

struct Board {
    numbers: [[u32; 5]; 5],
    has_won: bool,
}

impl Board {
    fn parse(lines: &[&String]) -> Option<Board> {
        let mut numbers = [[0u32; 5]; 5];

        let mut l = 0;
        for row in lines {
            let mut i = 0;
            for v in row.split(' ').filter_map(|v| match v.parse::<u32>() {
                Ok(r) => Some(r),
                _ => None,
            }) {
                numbers[l][i] = v;
                i += 1;

                if i > 5 {
                    return None;
                }
            }

            l += 1;
            if l == 5 {
                break;
            }
        }

        Some(Board {
            numbers,
            has_won: false,
        })
    }

    fn check_number(&mut self, number: u32) -> bool {
        if self.has_won {
            return true;
        }

        for i in 0..5 {
            for j in 0..5 {
                if self.numbers[i][j] == number {
                    self.numbers[i][j] = u32::MAX;
                    self.has_won = self.has_won(i, j);

                    return self.has_won;
                }
            }
        }

        return false;
    }

    fn has_won(&self, row: usize, column: usize) -> bool {
        self.numbers[row].iter().all(|n| *n == u32::MAX)
            || (0..5).all(|r| self.numbers[r][column] == u32::MAX)
    }

    fn sum_of_remaining(&self) -> u32 {
        self.numbers
            .iter()
            .flatten()
            .filter(|n| **n != u32::MAX)
            .sum()
    }
}
