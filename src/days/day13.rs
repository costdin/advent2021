use super::get_input_lines;

pub fn day13() {
    let input = get_input_lines(13);

    let (mut coords, folds) = parse_input(input);

    coords = fold_paper(coords, folds.iter().nth(0).unwrap());
    let result1 = coords.len();

    coords = folds
        .iter()
        .skip(1)
        .fold(coords, |acc, f| fold_paper(acc, f));

    println!("DAY 13\nSolution 1: {}\nSolution 2:", result1);
    print_output(coords);
    println!()
}

fn parse_input(lines: Vec<String>) -> (Vec<[u32; 2]>, Vec<Fold>) {
    let (coords, folds, _) = lines.iter().fold(
        (vec![], vec![], false),
        |(mut coords, mut folds, in_fold), l| {
            if l.len() == 0 {
                (coords, folds, true)
            } else if in_fold {
                folds.push(Fold::parse(l).unwrap());

                (coords, folds, in_fold)
            } else {
                coords.push(parse_coord(l).unwrap());

                (coords, folds, in_fold)
            }
        },
    );

    (coords, folds)
}

fn print_output(mut coords: Vec<[u32; 2]>) {
    coords.sort_by(|[_, y], [_, y2]| y.partial_cmp(y2).unwrap());

    let mut cy = 0;
    let mut cx = 0;
    for [x, y] in coords {
        while y > cy {
            println!("");
            cy += 1;
            cx = 0;
        }

        while x > cx {
            print!(" ");
            cx += 1;
        }

        print!("*");
        cx += 1;
    }
}

fn fold_paper(mut coords: Vec<[u32; 2]>, fold: &Fold) -> Vec<[u32; 2]> {
    let ix = if fold.direction == FoldDirection::X {
        0
    } else {
        1
    };

    for c in coords.iter_mut().filter(|c| c[ix] > fold.index) {
        c[ix] = 2 * fold.index - c[ix] as u32;
    }

    coords.sort();
    coords.dedup();

    coords
}

#[derive(PartialEq)]
enum FoldDirection {
    X,
    Y,
}

struct Fold {
    direction: FoldDirection,
    index: u32,
}

impl Fold {
    fn parse(line: &str) -> Option<Fold> {
        if let [_, _, d] = line.split(" ").collect::<Vec<_>>()[..] {
            if let [direction, ix] = d.split("=").collect::<Vec<_>>()[..] {
                Some(Fold {
                    direction: if direction == "x" {
                        FoldDirection::X
                    } else {
                        FoldDirection::Y
                    },
                    index: ix.parse::<u32>().unwrap(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn parse_coord(line: &str) -> Option<[u32; 2]> {
    if let [x, y] = line.split(",").collect::<Vec<_>>()[..] {
        Some([x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()])
    } else {
        None
    }
}
