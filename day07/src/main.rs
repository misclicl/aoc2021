fn parse_data(data: &str) -> Vec<i32> {
    data.lines()
        .next()
        .unwrap()
        .split(',')
        .map(|timer| timer.parse::<i32>().unwrap())
        .collect()
}

fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn part1(data: &str) -> u32 {
    let mut initial_positions = parse_data(data);

    let median_position = median(&mut initial_positions);

    initial_positions
        .iter()
        .map(|position| (*position - median_position).unsigned_abs())
        .sum()
}

fn main() {
    let data = include_str!("data.txt");

    let result = part1(data);
    println!("result#1: {}", result);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part1_example() {
        let data = include_str!("data_small.txt");
        let result = part1(data);
        assert_eq!(result, 37);
    }
    #[test]
    fn part2_example() {}
}
