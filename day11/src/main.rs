mod energy_grid {
    use std::{
        collections::{HashSet, VecDeque},
        fmt::{self, Display},
    };

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct GridPos {
        x: usize,
        y: usize,
    }

    #[derive(Debug)]
    pub struct EnergyGrid {
        grid: Vec<Vec<u8>>,
        pub flash_count: u32,
        size_y: usize,
        size_x: usize,
        flash_queue: VecDeque<GridPos>,
    }

    const NEIGHBOR_COORDS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    impl EnergyGrid {
        pub fn new(data: Vec<Vec<u8>>) -> Self {
            let size_y = data.len();
            let size_x = data[0].len();

            EnergyGrid {
                grid: data,
                flash_count: 0,
                size_x,
                size_y,
                flash_queue: VecDeque::new(),
            }
        }

        fn increase_by_one(&mut self) {
            for y in 0..self.size_y {
                for x in 0..self.size_x {
                    let value = &mut self.grid[y][x];
                    *value += 1;

                    if *value > 9 {
                        self.flash_queue.push_back(GridPos { x, y });
                    }
                }
            }
        }

        pub fn step(&mut self) {
            let mut flashed: HashSet<GridPos> = HashSet::new();
            self.increase_by_one();

            while let Some(current_pos) = self.flash_queue.pop_front() {
                if flashed.contains(&current_pos) {
                    continue;
                }

                let value = &mut self.grid[current_pos.y][current_pos.x];
                *value = 0;

                flashed.insert(current_pos);

                for (offset_x, offset_y) in NEIGHBOR_COORDS.iter() {
                    let n_pos_x = current_pos.x as i32 + offset_x;
                    let n_pos_y = current_pos.y as i32 + offset_y;

                    if n_pos_x >= 0
                        && n_pos_y >= 0
                        && n_pos_x < self.size_x as i32
                        && n_pos_y < self.size_y as i32
                        && !flashed.contains(&GridPos {
                            x: n_pos_x as usize,
                            y: n_pos_y as usize,
                        })
                    {
                        let val = &mut self.grid[n_pos_y as usize][n_pos_x as usize];
                        *val += 1;

                        if *val > 9 {
                            self.flash_queue.push_back(GridPos {
                                x: n_pos_x as usize,
                                y: n_pos_y as usize,
                            });
                        }
                    }
                }
            }

            self.flash_count += flashed.len() as u32;
        }

        pub fn is_all_zeros(&self) -> bool {
            self.grid
                .iter()
                .all(|row| row.iter().all(|&value| value == 0))
        }

        pub fn find_sync_step(&mut self) -> u32 {
            let mut step_count = 0;
            while !self.is_all_zeros() {
                self.step();
                step_count += 1;
            }

            step_count
        }
    }

    impl Display for EnergyGrid {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for y in 0..self.size_y {
                let mut line = String::new();
                for x in 0..self.size_x {
                    let value = self.grid[y][x];

                    line.push_str(&format!("{} ", value));
                }
                writeln!(f, "{}", line)?;
            }
            Ok(())
        }
    }
}

fn parse_input(data: &str) -> Vec<Vec<u8>> {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part1(data: &str) -> u32 {
    let data = parse_input(data);

    let mut grid = energy_grid::EnergyGrid::new(data);

    for _ in 0..100 {
        grid.step();
    }

    println!("{}", grid);

    grid.flash_count
}

fn part2(data: &str) -> u32 {
    let data = parse_input(data);

    let mut grid = energy_grid::EnergyGrid::new(data);
    grid.find_sync_step()
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
        assert_eq!(result, 1656);
    }
    #[test]
    fn part2_example() {
        let data = include_str!("data_example.txt");
        let result = part2(data);
        assert_eq!(result, 195);
    }
}
