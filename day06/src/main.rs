use std::collections::LinkedList;

type State = Vec<u8>;

fn parse_data(data: &str) -> Vec<u8> {
    data.lines()
        .next()
        .unwrap()
        .split(",")
        .map(|timer| timer.parse::<u8>().unwrap())
        .collect()
}

fn process_iteration(state: &mut State) {
    for i in 0..state.len() {
        let entry = &mut state[i];
        if *entry == 0 {
            *entry = 6;
            state.push(8);
        } else {
            *entry -= 1;
        }
    }
}

fn part1(data: &str, iterations: i32) -> usize {
    let mut state = parse_data(data);
    (0..iterations).for_each(|_| process_iteration(&mut state));
    state.len()
}

fn part2(data: &str, iterations: u32) -> u64 {
    let state = parse_data(data);

    let mut list = [0; 9];

    for entry in state {
        list[entry as usize] += 1;
    }

    let mut list: LinkedList<u64> = LinkedList::from_iter(list);

    for _ in 0..iterations {
        let overflow = list.pop_front().unwrap();
        list.push_back(overflow);

        let mut count = 2;
        let mut current_node = list.iter_mut().rev();

        while let Some(node) = current_node.next() {
            if count == 0 {
                *node += overflow;
                break;
            }

            count -= 1;
        }
    }

    list.iter().sum()
}

fn main() {
    let data = include_str!("data_small.txt");

    let result = part1(data, 12);
    println!("result#1: {}", result);

    let result = part2(data, 12);
    println!("result#2: {}", result);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part1_example() {
        let data = include_str!("data_small.txt");
        let result = part1(data, 18);
        assert_eq!(result, 26);
        let result = part1(data, 80);
        assert_eq!(result, 5934);
    }
    #[test]
    fn part2_example() {
        let data = include_str!("data_small.txt");
        let result = part2(data, 18);
        assert_eq!(result, 26);
        let result = part2(data, 80);
        assert_eq!(result, 5934);
        let result = part2(data, 256);
        assert_eq!(result, 26984457539);
    }
}
