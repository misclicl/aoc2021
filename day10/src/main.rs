use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref BRACKET_MAP: HashMap<char, char> = [(')', '('), (']', '['), ('}', '{'), ('>', '<')]
        .iter()
        .cloned()
        .collect();
}

fn parse_input(data: &str) -> Vec<String> {
    data.lines().map(|line| line.to_string()).collect()
}

fn calc_line_score(line: &str) -> Option<char> {
    let mut stack: Vec<char> = Vec::new();

    for char in line.chars() {
        if let Some(&expected_opening) = BRACKET_MAP.get(&char) {
            if stack.pop() != Some(expected_opening) {
                return Some(char);
            }
        } else {
            stack.push(char);
        }
    }

    None
}

fn calc_line_incomplete_score(line: &str) -> Option<Vec<char>> {
    let mut stack: Vec<char> = Vec::new();

    for char in line.chars() {
        if let Some(&expected_opening) = BRACKET_MAP.get(&char) {
            if stack.pop() != Some(expected_opening) {
                return None;
            }
        } else {
            stack.push(char);
        }
    }

    Some(stack.into_iter().rev().collect())
}

fn part1(data: &str) -> u32 {
    let lines = parse_input(data);
    let score_map: HashMap<char, u32> = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .into_iter()
        .collect();

    let result: u32 = lines
        .iter()
        .filter_map(|line| calc_line_score(line))
        .filter_map(|char| score_map.get(&char))
        .sum();

    result
}

fn part2(data: &str) -> u64 {
    let lines = parse_input(data);
    let score_map: HashMap<char, u64> = [('(', 1), ('[', 2), ('{', 3), ('<', 4)]
        .into_iter()
        .collect();

    let mut result = lines
        .iter()
        .filter_map(|line| calc_line_incomplete_score(line))
        .map(|line| {
            line.iter()
                .filter_map(|char| score_map.get(char))
                .fold(0, |acc, char_score| acc * 5 + *char_score)
        })
        .collect::<Vec<_>>();

    result.sort();

    let middle_index = result.len() / 2;
    result[middle_index]
}

fn main() {
    let data = include_str!("data.txt");

    let result = part1(data);
    println!("result#1: {}", result);

    let result = part2(data);
    println!("result#2: {}", result);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part1_example() {
        let data = include_str!("data_example.txt");
        let result = part1(data);
        assert_eq!(result, 26397);
    }
    #[test]
    fn part2_example() {
        let data = include_str!("data_example.txt");
        let result = part2(data);
        assert_eq!(result, 288957);
    }
}
