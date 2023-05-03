use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pos(usize, usize);

const GRID_SIZE: usize = 5;

#[derive(Debug)]
struct Board {
    cache: HashMap<u32, Pos>,
    matched: HashSet<Pos>,
}

impl Board {
    fn new(data: Vec<Vec<u32>>) -> Self {
        let mut cache: HashMap<u32, Pos> = HashMap::new();

        for y in 0..GRID_SIZE {
            for (x, row) in data.iter().enumerate() {
                let entry = row[y];
                cache.insert(entry, Pos(x, y));
            }
        }

        Board {
            cache,
            matched: HashSet::new(),
        }
    }

    fn check_neighbors(&self, position: Pos) -> bool {
        let vertical_match = (0..GRID_SIZE).all(|i| self.matched.contains(&Pos(position.0, i)));
        let horizontal_match = (0..GRID_SIZE).all(|i| self.matched.contains(&Pos(i, position.1)));

        vertical_match || horizontal_match
    }

    fn mark(&mut self, value: u32) -> bool {
        let position = match self.cache.get(&value) {
            Some(pos) => pos,
            None => {
                return false;
            }
        };

        self.matched.insert(*position);
        self.check_neighbors(*position)
    }

    fn calc_score(&self) -> u32 {
        self.cache
            .iter()
            .filter_map(|(value, position)| {
                if self.matched.contains(position) {
                    None
                } else {
                    Some(value)
                }
            })
            .sum()
    }
}

fn parse_boards(board_data: Vec<&str>) -> Vec<Board> {
    board_data
        .split(|line| line.is_empty())
        .map(|chunk| {
            chunk
                .iter()
                .map(|row| {
                    row.split_whitespace()
                        .map(|value| value.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .map(Board::new)
        .collect()
}

fn parse_data(data: &str) -> (Vec<u32>, Vec<Board>) {
    let mut lines = data.lines();

    let numbers = lines
        .next()
        .unwrap()
        .split_terminator(',')
        .map(|n| n.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()
        .unwrap();

    lines.next();

    (numbers, parse_boards(lines.collect()))
}

fn part1(data: &str) -> u32 {
    let (numbers, mut boards) = parse_data(data);
    let mut result = 0;

    'outer: for number in numbers {
        for board in boards.iter_mut() {
            if board.mark(number) {
                result = number * board.calc_score();
                break 'outer;
            }
        }
    }

    result
}

fn part2(data: &str) -> u32 {
    let (numbers, mut boards) = parse_data(data);
    let mut won: HashSet<usize> = HashSet::new();
    let mut result = 0;

    let boards_count = boards.len();
    'outer: for number in numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if board.mark(number) && !won.contains(&i) {
                won.insert(i);
                result = number * board.calc_score();

                if won.len() == boards_count {
                    break 'outer;
                }
            }
        }
    }

    result
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
    #[test]
    fn part1() {
        let result = crate::part1(include_str!("data_small.txt"));
        assert_eq!(result, 4512);
    }
    #[test]
    fn part2() {
        let result = crate::part2(include_str!("data_small.txt"));
        assert_eq!(result, 1924);
    }
}
