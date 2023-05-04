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

fn gaussian(n: u32) -> u32 {
    n * (n + 1) / 2
}

fn part1(data: &str) -> u32 {
    let mut initial_positions = parse_data(data);

    let median_position = median(&mut initial_positions);

    initial_positions
        .iter()
        .map(|position| (*position - median_position).unsigned_abs())
        .sum()
}

fn calc_sum(positions: &[i32], target: i32) -> u32 {
    positions
        .iter()
        .map(|position| gaussian((*position - target).unsigned_abs()))
        .sum()
}

fn part2(data: &str) -> u32 {
    let mut initial_positions = parse_data(data);
    initial_positions.sort();

    let mut min = *initial_positions.iter().min().unwrap();
    let mut max = *initial_positions.iter().max().unwrap();

    let mut min_sum = u32::max_value();

    while min <= max {
        let mid = (min + max) / 2;

        let current_sum: u32 = calc_sum(&initial_positions, mid);
        let next_sum: u32 = calc_sum(&initial_positions, mid + 1);

        if current_sum > next_sum {
            min = mid + 1;
        } else {
            max = mid - 1;
        }

        min_sum = std::cmp::min(min_sum, current_sum);
    }

    min_sum
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
        let data = include_str!("data_small.txt");
        let result = part1(data);
        assert_eq!(result, 37);
    }
    #[test]
    fn part2_example() {
        let data = include_str!("data_small.txt");
        let result = part2(data);
        assert_eq!(result, 168);
    }
}
