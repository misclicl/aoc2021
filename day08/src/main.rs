type Entry = (Vec<String>, Vec<String>);

fn process_entry(entry: &str) -> Vec<String> {
    entry.trim().split(' ').map(|s| s.to_owned()).collect()
}

fn parse_input(data: &str) -> Vec<Entry> {
    data.lines()
        .map(|line| {
            let entry: Vec<&str> = line.split('|').collect();
            (process_entry(entry[0]), process_entry(entry[1]))
        })
        .collect()
}

fn part1(data: &str) -> u32 {
    let outputs: Vec<Vec<String>> = parse_input(data)
        .into_iter()
        .map(|(_, output)| output)
        .collect();

    outputs
        .iter()
        .flatten()
        .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
        .count() as u32
}

fn decode(entry: &Entry) -> u32 {
    let (patterns, outputs) = entry;
    let one = patterns.iter().find(|d| d.len() == 2).unwrap();
    let four = patterns.iter().find(|d| d.len() == 4).unwrap();

    let decoded_digits: Vec<_> = outputs
        .iter()
        .map(|digit| match digit.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            len => match (
                len,
                digit.chars().filter(|&d| one.contains(d)).count(),
                digit.chars().filter(|&d| four.contains(d)).count(),
            ) {
                (5, 2, 3) => 3,
                (5, 1, 3) => 5,
                (5, _, 2) => 2,

                (6, 1, _) => 6,
                (6, _, 3) => 0,
                (6, _, 4) => 9,

                _ => unreachable!(),
            },
        })
        .collect();

    decoded_digits
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, d)| acc + d * 10u32.pow(i as u32))
}

fn part2(data: &str) -> u32 {
    parse_input(data).iter().map(decode).sum()
}

fn main() {
    let data = include_str!("data.txt");
    let result = part1(data);
    println!("result#1: {}", result);

    let data = include_str!("data.txt");
    let result = part2(data);
    println!("result#2: {}", result);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part1_example() {
        let data = include_str!("data_small.txt");
        let result = part1(data);
        assert_eq!(result, 26);
    }

    #[test]
    fn part2_example() {
        let data = include_str!("data_small.txt");
        let result = part2(data);
        assert_eq!(result, 61229);
    }
}
