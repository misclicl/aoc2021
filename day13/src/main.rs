use std::{
    collections::HashSet,
    fmt::{self, Display},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct GridPos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct PaperSheet {
    dots: Vec<GridPos>,
    size_x0: usize,
    size_y: usize,
}

impl PaperSheet {
    pub fn new(dots: Vec<GridPos>, size_y: usize) -> Self {
        PaperSheet {
            size_x0: 0,
            size_y,
            dots,
        }
    }

    pub fn fold_x_left(&mut self, fold_pos: usize) {
        let fold_pos_local = self.size_x0 + fold_pos;
        for point in &mut self.dots {
            if point.x < fold_pos_local {
                point.x = fold_pos_local * 2 - point.x;
            }
        }

        self.size_x0 = fold_pos_local + 1;
    }

    pub fn fold_y_up(&mut self, fold_pos: usize) {
        for point in &mut self.dots {
            if (point.y) > fold_pos {
                point.y = 2 * fold_pos - point.y;
            }
        }

        self.size_y = fold_pos - 1;
    }

    fn count_dots(&self) -> u32 {
        self.dots
            .clone()
            .into_iter()
            .collect::<HashSet<GridPos>>()
            .len() as u32
    }
}

impl Display for PaperSheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (min_x, max_x, min_y, max_y) =
            self.dots
                .iter()
                .fold((usize::MAX, 0, usize::MAX, 0), |acc, point| {
                    (
                        acc.0.min(point.x),
                        acc.1.max(point.x),
                        acc.2.min(point.y),
                        acc.3.max(point.y),
                    )
                });

        let mut grid: Vec<Vec<char>> = vec![vec!['.'; max_x - min_x + 1]; max_y - min_y + 1];
        for point in &self.dots {
            grid[point.y - min_y][point.x - min_x] = '#';
        }

        for row in grid {
            for value in &row[0..=max_x - min_x] {
                write!(f, "{} ", value)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
enum Instruction {
    X(u32),
    Y(u32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts = input.split('=').collect::<Vec<_>>();
        let value = parts[1].parse::<u32>().unwrap();

        if input.contains('x') {
            Ok(Instruction::X(value))
        } else {
            Ok(Instruction::Y(value))
        }
    }
}

fn parse_input(data: &str) -> (PaperSheet, Vec<Instruction>) {
    let mut points: Vec<GridPos> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut size_x = usize::MIN;
    let mut size_y = usize::MIN;

    let mut lines = data.lines();

    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let coords = line
            .split(',')
            .map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let x = coords[0] as usize;
        let y = coords[1] as usize;

        points.push(GridPos { x, y });

        size_x = size_x.max(x);
        size_y = size_y.max(y);
    }

    for line in lines {
        if let Ok(instruction) = Instruction::from_str(line) {
            instructions.push(instruction);
        }
    }

    (PaperSheet::new(points, size_y), instructions)
}

fn part1(data: &str) -> u32 {
    let (mut paper_sheet, instructions) = parse_input(data);
    let mut instructions = instructions.iter();

    match instructions.next().unwrap() {
        Instruction::X(value) => paper_sheet.fold_x_left(*value as usize),
        Instruction::Y(value) => paper_sheet.fold_y_up(*value as usize),
    }

    paper_sheet.count_dots()
}

fn part2(data: &str) {
    let (mut paper_sheet, instructions) = parse_input(data);

    for instruction in &instructions {
        match instruction {
            Instruction::X(value) => paper_sheet.fold_x_left(*value as usize),
            Instruction::Y(value) => paper_sheet.fold_y_up(*value as usize),
        }
    }

    println!("{}", paper_sheet);
}

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
        assert_eq!(part1(data), 17);
    }
}
