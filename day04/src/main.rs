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
            for x in 0..GRID_SIZE {
                let entry = data[x][y];
                cache.insert(entry, Pos(x, y));
            }
        }

        Board {
            cache,
            matched: HashSet::new(),
        }
    }

    fn check_neighbors(&self, position: Pos) -> bool {
        let mut v = true;
        let mut h = true;

        for i in 0..GRID_SIZE {
            let v_pos = Pos(position.0, i);
            let h_pos = Pos(i, position.1);

            if !self.matched.contains(&v_pos) {
                v = false;
            }

            if !self.matched.contains(&h_pos) {
                h = false;
            }
        }

        return v || h;
    }

    fn mark(&mut self, value: &u32) -> bool {
        let position = match self.cache.get(value) {
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
                .into_iter()
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
    let (numbers, mut boards) = parse_data(&data);

    let mut winning_board: Option<usize> = None;
    let mut winning_number: Option<u32> = None;

    'outer: for number in numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            match board.mark(&number) {
                true => {
                    winning_board = Some(i);
                    winning_number = Some(number);
                    println!("found board: {}", i);
                    break 'outer;
                }
                false => {}
            }
        }
    }

    println!("winning board: {:?}", winning_board);
    // println!("boards: {:?}", boards);

    let score = boards[winning_board.unwrap()].calc_score();
    let result = score * winning_number.unwrap();
    println!("score: {}", result);

    result
}

fn main() {
    let data = include_str!("data.txt");
    part1(data);
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let result = crate::part1(include_str!("data_small.txt"));
        assert_eq!(result, 4512);
    }
}
