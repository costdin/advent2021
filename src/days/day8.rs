use super::get_input_lines;

pub fn day8() {
    let input = get_input_lines(8);
    let lines = input
        .iter()
        .map(|l| l.split(" | ").collect::<Vec<_>>())
        .map(|l| decode_display(l[1], create_decode_table(l[0])))
        .collect::<Vec<_>>();

    let result1 = lines
        .iter()
        .map(|l| l.iter().filter(|v| [1, 4, 7, 8].contains(v)).count())
        .sum::<usize>();

    let result2 = lines
        .iter()
        .map(|l| l.iter().fold(0u32, |acc, n| acc * 10 + *n as u32))
        .sum::<u32>();

    println!("DAY 8\nSolution 1: {}\nSolution 2: {}", result1, result2);
}

fn decode_display(s: &str, decode_table: [u8; 10]) -> [u8; 4] {
    let mut result = [0; 4];

    for (ix, b) in s
        .split_ascii_whitespace()
        .map(string_to_byte)
        .filter_map(|b| decode_table.iter().position(|&r| r == b))
        .enumerate()
    {
        result[ix] = b as u8
    }

    result
}

fn create_decode_table(s: &str) -> [u8; 10] {
    let mut result = [0; 10];

    let bytes = s
        .split_ascii_whitespace()
        .map(string_to_byte)
        .collect::<Vec<_>>();

    for n in bytes.iter() {
        decode(*n, &mut result);
    }

    for n in bytes {
        second_decode(n, &mut result);
    }

    result
}

fn string_to_byte(s: &str) -> u8 {
    s.chars()
        .map(|c| 1 << (c as u8 - b'a'))
        .fold(0, |acc, n| acc | n)
}

fn decode(byte: u8, vec: &mut [u8]) {
    let ptr = vec.as_mut_ptr();

    match byte.count_ones() {
        2 => unsafe { *ptr.offset(1) = byte },
        4 => unsafe { *ptr.offset(4) = byte },
        3 => unsafe { *ptr.offset(7) = byte },
        7 => unsafe { *ptr.offset(8) = byte },
        _ => {}
    }
}

fn second_decode(byte: u8, vec: &mut [u8]) {
    let ptr = vec.as_mut_ptr();

    match byte.count_ones() {
        6 if (byte & vec[1]).count_ones() == 1 => unsafe { *ptr.offset(6) = byte },
        6 if (byte | vec[4]) == vec[8] => unsafe { *ptr = byte },
        6 => unsafe { *ptr.offset(9) = byte },

        5 if (byte & vec[4]).count_ones() == 2 => unsafe { *ptr.offset(2) = byte },
        5 if (byte & vec[1]).count_ones() == 1 => unsafe { *ptr.offset(5) = byte },
        5 => unsafe { *ptr.offset(3) = byte },

        _ => {}
    }
}
