fn parse_data(data: &str) -> Vec<i32> {
    data.lines()
        .next()
        .unwrap()
        .split(',')
        .map(|timer| timer.parse::<i32>().unwrap())
        .collect()
}

fn part1(data: &str) -> u32 {
    let initial_positions = parse_data(data);

    let min = *initial_positions.iter().min().unwrap() as usize;
    let max = *initial_positions.iter().max().unwrap() as usize;

    (min..=max)
        .map(|candidate| {
            initial_positions
                .iter()
                .map(|position| (*position - candidate as i32).unsigned_abs())
                .sum()
        })
        .min()
        .unwrap()
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
