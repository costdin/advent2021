use super::get_input_lines;
use std::collections::HashMap;
use std::hash::Hash;

pub fn day14() {
    let input = get_input_lines(14);
    let (pairs, rules) = parse(input);

    let pairs_10 = (0..10).fold(pairs, |acc, _| apply_rules(&acc, &rules));
    let pairs_11 = apply_rules(&pairs_10, &rules);
    let pairs_40 = (0..29).fold(pairs_11, |acc, _| apply_rules(&acc, &rules));

    let char_counts_10 = pairs_10
        .into_iter()
        .map(|(pair, count)| pair.chars().map(move |c| (c, count)).collect::<Vec<_>>())
        .flatten();

    let char_counts_40 = pairs_40
        .into_iter()
        .map(|(pair, count)| pair.chars().map(move |c| (c, count)).collect::<Vec<_>>())
        .flatten();

    let mut final_10 = aggregate(char_counts_10)
        .into_iter()
        .map(|(_, count)| (count + 1) / 2)
        .collect::<Vec<_>>();
    final_10.sort_by(|c1, c2| c2.partial_cmp(c1).unwrap());

    let mut final_40 = aggregate(char_counts_40)
        .into_iter()
        .map(|(_, count)| (count + 1) / 2)
        .collect::<Vec<_>>();
    final_40.sort_by(|c1, c2| c2.partial_cmp(c1).unwrap());

    let result10 = final_10.first().unwrap() - final_10.last().unwrap();
    let result40 = final_40.first().unwrap() - final_40.last().unwrap();

    println!("DAY 14\nSolution 1: {}\nSolution 2: {}", result10, result40);
}

fn parse(input: Vec<String>) -> (HashMap<String, u128>, HashMap<String, [String; 2]>) {
    (
        parse_template(&input[0]),
        input.iter().skip(2).flat_map(parse_rule).collect(),
    )
}

fn parse_template(template: &String) -> HashMap<String, u128> {
    aggregate(
        template
            .chars()
            .zip(template.chars().skip(1))
            .map(|(c1, c2)| (String::from_utf8(vec![c1 as u8, c2 as u8]).unwrap(), 1)),
    )
}

fn parse_rule(line: &String) -> Option<(String, [String; 2])> {
    if let [pair, insertion] = line.split(" -> ").collect::<Vec<_>>()[..] {
        let mut p1 = String::with_capacity(2);
        p1.push(pair.chars().nth(0).unwrap());
        p1.push_str(insertion);
        let mut p2 = String::with_capacity(2);
        p2.push_str(insertion);
        p2.push(pair.chars().nth(1).unwrap());

        Some((pair.to_string(), [p1, p2]))
    } else {
        None
    }
}

fn apply_rules(
    pairs: &HashMap<String, u128>,
    rules: &HashMap<String, [String; 2]>,
) -> HashMap<String, u128> {
    let flat_collection = pairs
        .into_iter()
        .map(|(p, count)| rules[p].iter().map(move |m| (m.clone(), *count)))
        .flatten();

    aggregate(flat_collection)
}

pub fn aggregate<T>(elements: impl Iterator<Item = (T, u128)>) -> HashMap<T, u128>
where
    T: Eq,
    T: Hash,
    T: Clone,
{
    let mut hashmap = HashMap::<T, u128>::new();

    for (key, count) in elements {
        match hashmap.get_mut(&key) {
            Some(c) => *c += count,
            None => {
                hashmap.insert(key.clone(), count);
            }
        }
    }

    hashmap
}
