use std::collections::HashMap;

type Pattern = Vec<String>;
type InsertionDict = HashMap<String, (String, String)>;
type PairFrequency = HashMap<String, u64>;

#[derive(Debug)]
struct Polymer<'a> {
    pair_counter: PairFrequency,
    insertion_dict: &'a InsertionDict,
}

impl<'a> Polymer<'a> {
    fn new(pattern: Vec<String>, insertion_dict: &'a InsertionDict) -> Self {
        let mut pair_counter: PairFrequency = PairFrequency::new();

        pattern.iter().for_each(|pair| {
            pair_counter
                .entry(pair.to_owned())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        });

        Polymer {
            pair_counter,
            insertion_dict,
        }
    }

    fn ceil_div(a: u64, b: u64) -> u64 {
        (a + b - 1) / b
    }

    fn step(&mut self) {
        let mut new_pair_counter = HashMap::new();

        self.pair_counter.iter().for_each(|(pair, v)| {
            let new_insertions = self.insertion_dict.get(pair).unwrap();

            new_pair_counter
                .entry(new_insertions.0.to_owned())
                .and_modify(|counter| *counter += *v)
                .or_insert(*v);
            new_pair_counter
                .entry(new_insertions.1.to_owned())
                .and_modify(|counter| *counter += *v)
                .or_insert(*v);
        });

        self.pair_counter = new_pair_counter;
    }

    fn count_characters(&self) -> HashMap<char, u64> {
        let mut character_freq = HashMap::new();

        for (pair, count) in &self.pair_counter {
            for char in pair.chars() {
                character_freq
                    .entry(char)
                    .and_modify(|counter| *counter += *count)
                    .or_insert(*count);
            }
        }

        let mut element_freq = HashMap::new();

        for (char, count) in character_freq {
            element_freq.insert(char, Self::ceil_div(count, 2));
        }

        element_freq
    }

    fn develop(&mut self, iterations: u32) {
        for _ in 0..iterations {
            self.step();
        }
    }

    fn count_result(&self) -> u64 {
        let char_count = self.count_characters();

        let (min, max) = char_count.iter().fold((u64::MAX, 0), |acc, (_, count)| {
            (acc.0.min(*count), acc.1.max(*count))
        });

        max - min
    }
}

fn parse_input(data: &str) -> (Pattern, InsertionDict) {
    let mut lines = data.lines();
    let mut dict: InsertionDict = HashMap::new();

    let chars = lines.next().unwrap().chars().collect::<Vec<_>>();
    let mut pattern = Vec::new();

    for i in 0..chars.len() - 1 {
        pattern.push(format!("{}{}", chars[i], chars[i + 1]));
    }

    lines.next();
    lines.for_each(|l| {
        let parts = l.split(" -> ").collect::<Vec<_>>();
        let to_insert = parts[1];
        let part_1_chars = parts[0].chars().collect::<Vec<_>>();

        let result_a = format!("{}{to_insert}", part_1_chars[0]);
        let result_b = format!("{to_insert}{}", part_1_chars[1]);

        dict.insert(parts[0].to_owned(), (result_a, result_b));
    });

    (pattern, dict)
}

fn part1(data: &str) -> u64 {
    let (pattern, insertion_map) = parse_input(data);

    let mut polymer = Polymer::new(pattern, &insertion_map);
    polymer.develop(10);
    polymer.count_result()
}

fn part2(data: &str) -> u64 {
    let (pattern, insertion_map) = parse_input(data);

    let mut polymer = Polymer::new(pattern, &insertion_map);
    polymer.develop(40);
    polymer.count_result()
}

fn main() {
    let data = include_str!("data.txt");

    let result = part1(data);
    println!("result#1: {}", result);

    let result = part2(data);

    println!("result#2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let data = include_str!("data_example.txt");
        assert_eq!(part1(data), 1588);
    }
    #[test]
    fn part2_example() {
        let data = include_str!("data_example.txt");
        assert_eq!(part2(data), 2188189693529);
    }
}
