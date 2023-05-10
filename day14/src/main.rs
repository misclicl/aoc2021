use std::collections::HashMap;

type Pattern = String;
type InsertionDict = HashMap<String, String>;

fn parse_input(data: &str) -> (Pattern, InsertionDict) {
    let mut lines = data.lines();
    let mut dict: InsertionDict = HashMap::new();

    // let pattern = lines.next().unwrap().chars().collect::<Vec<_>>();
    let pattern_str = lines.next().unwrap().to_owned();
    lines.next();

    // TODO: try to use references for strings here
    // maybe it's worth it?
    lines.for_each(|l| {
        let parts = l.split(" -> ").collect::<Vec<_>>();
        dict.insert(String::from(parts[0]), String::from(parts[1]));
    });

    (pattern_str, dict)
}

fn step(pattern: &String, dict: &InsertionDict) -> String {
    //      NNCB
    //      v v v
    //     NCNBCHB
    // ii   1 3 5
    // it   0 1 2
    // of   1 2 3
    let mut new_pattern = pattern.clone();
    let mut insertion_offset = 1;

    // println!("{dict:?}");

    for i in 0..pattern.len() - 1 {
        let pair_slice = &pattern[i..i + 2];
        // println!("{pair_slice}");

        let fallback = String::from("not_found");
        let insertion = dict.get(pair_slice).unwrap_or(&fallback);
        new_pattern.insert_str(i + insertion_offset, insertion);
        insertion_offset += 1;
    }

    new_pattern
}

fn part1(data: &str) -> u32 {
    let (pattern_str, pair_insertions) = parse_input(data);

    let mut result = pattern_str;
    let iterations_count = 10;

    for _ in 0..iterations_count {
        result = step(&result, &pair_insertions);
    }

    let mut frequency_counter: HashMap<char, u32> = HashMap::new();

    for char in result.chars() {
        frequency_counter
            .entry(char)
            .and_modify(|c| *c += 1)
            .or_insert(0);
    }

    let (min, max) = frequency_counter
        .iter()
        .fold((u32::MAX, 0), |acc, (_, c)| (acc.0.min(*c), acc.1.max(*c)));

    max - min
}

fn part2(data: &str) {}

fn main() {
    let data = include_str!("data.txt");

    let result = part1(data);
    println!("result#1: {}", result);

    part2(data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let data = include_str!("data_example.txt");
        assert_eq!(part1(data), 1588);
    }
}
