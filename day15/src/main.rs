use std::collections::{HashMap, HashSet, VecDeque};
use utils::print_matrix;

type Grid = Vec<Vec<u32>>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

struct PathFinder {
    grid: Grid,
}

#[derive(Debug)]
struct DistanceEntry {
    value: u32,
    prev: Option<Position>,
}

type DangerTable = HashMap<Position, DistanceEntry>;

fn get_matrix_size<T>(data: &[Vec<T>]) -> (usize, usize) {
    (data[0].len(), data.len())
}

impl PathFinder {
    const NEIGHBOR_COORS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn new(grid: Grid) -> Self {
        PathFinder { grid }
    }

    fn get_neighbors(&self, position: &Position) -> Vec<Position> {
        let (size_x, size_y) = { (self.grid[0].len(), self.grid.len()) };

        Self::NEIGHBOR_COORS
            .iter()
            .map(|offset| (position.x as i32 + offset.0, position.y as i32 + offset.1))
            .filter(|(x, y)| *x >= 0 && *x < size_x as i32 && *y >= 0 && *y < size_y as i32)
            .map(|(x, y)| Position {
                x: x as usize,
                y: y as usize,
            })
            .collect::<Vec<_>>()
    }

    fn get_cell_value(&self, position: Position) -> &u32 {
        &self.grid[position.y][position.x]
    }

    fn size(&self) -> (usize, usize) {
        get_matrix_size(&self.grid)
    }

    fn reconstruct_path(&self, danger_table: &DangerTable) -> Vec<Position> {
        let mut result: Vec<Position> = Vec::new();

        let grid_size = self.size();
        let mut next_node: Option<Position> = Some(Position {
            x: grid_size.0 - 1,
            y: grid_size.1 - 1,
        });

        while let Some(node_position) = next_node {
            let entry = danger_table.get(&node_position).unwrap();
            result.push(node_position);
            next_node = entry.prev;
        }

        result.reverse();
        result
    }

    fn print_danger_map(&self, danger_table: &DangerTable) {
        let grid_size = self.size();

        let mut printable = vec![vec![0; grid_size.0]; grid_size.1];

        for (key, entry) in danger_table {
            printable[key.y][key.x] = entry.value;
        }

        print_matrix(&printable);
    }

    /*

    Explaining the Dijkstra algorithm to myself

    Consider the following graph with node weights:

    1 ---- 5 ---- 6
           |      |
           3------3
                  |
                  1

    The shortest path is: 1-5-3-3-1 (overall weight is 13)
    For this example, I'm going to perform a depth-first traversal.

    node path_weight* prev_node
    1    1            None
    5    6            Some(1) (0, 0) -- note that i need to store coordinates here
    6    12           Some(5)
    3b   15           Some(6)
    1    16           Some(17)
    -------------------------
    ! The first result is found, but travelsal isn't over
    (i need to visit each node of the graph)
    -------------------------
    3a   9            Some(5)
    3b   12           Some(3a) <-- 12 < 15, updating the map
    1    13           Some(3b) <-- 13 < 16, updating the map again

    *path_weight is calculated as follows:
    current_node_weigth + prev_node_weight

    Reconsructing path: 1 -> 3b -> 3a -> 5 -> 1
    */

    fn find_path(&self) -> u32 {
        let mut visited: HashSet<Position> = HashSet::new();
        let mut min_danger_levels: HashMap<Position, DistanceEntry> = HashMap::new();
        min_danger_levels.insert(
            Position { x: 0, y: 0 },
            DistanceEntry {
                value: self.grid[0][0],
                prev: None,
            },
        );

        let mut queue: VecDeque<Position> = VecDeque::new();
        queue.push_back(Position { x: 0, y: 0 });

        while let Some(position_current) = queue.pop_front() {
            visited.insert(position_current);

            let value_current = min_danger_levels.get(&position_current).unwrap().value;

            for neighbor in self.get_neighbors(&position_current) {
                let candidate_value = value_current + self.get_cell_value(neighbor);

                if let Some(existing_entry) = min_danger_levels.get_mut(&neighbor) {
                    if existing_entry.value > candidate_value {
                        existing_entry.prev = Some(position_current);
                        existing_entry.value = candidate_value;
                    }
                } else {
                    min_danger_levels.insert(
                        neighbor,
                        DistanceEntry {
                            value: candidate_value,
                            prev: Some(position_current),
                        },
                    );
                }

                if !visited.contains(&neighbor) && !queue.contains(&neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        let path = self.reconstruct_path(&min_danger_levels);

        PathFinder::vis_path(&self.grid, &path);

        path.iter().skip(1).map(|p| self.grid[p.y][p.x]).sum()
    }

    fn vis_path(matrix: &[Vec<u32>], path: &[Position]) {
        let mut cloned: Vec<Vec<char>> = matrix
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&cell| std::char::from_digit(cell, 10).unwrap())
                    .collect()
            })
            .collect();

        for node in path {
            cloned[node.y][node.x] = '-';
        }

        print_matrix(&cloned);
    }
}

fn parse_input(data: &str) -> Vec<Vec<u32>> {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn extend_matrix(matrix: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let (size_x, size_y) = get_matrix_size(matrix);
    let multiplier = 5;
    let mut matrix_extended = vec![vec![0; size_x * multiplier]; size_y * multiplier];

    for offset_y in 0..multiplier {
        for offset_x in 0..multiplier {
            for y in 0..size_y {
                for x in 0..size_x {
                    let value = matrix[y][x] + offset_x as u32 + offset_y as u32;
                    let new_value = ((value - 1) % 9) + 1;
                    matrix_extended[offset_y * size_y + y][offset_x * size_x + x] = new_value;
                }
            }
        }
    }

    print_matrix(&matrix_extended);

    matrix_extended
}

fn part1(data: &str) -> u32 {
    let matrix = parse_input(data);

    let path_finder = PathFinder::new(matrix);
    path_finder.find_path()
}

fn part2(data: &str) -> u32 {
    let matrix = parse_input(data);
    let matrix_extended = extend_matrix(&matrix);

    let path_finder = PathFinder::new(matrix_extended);
    path_finder.find_path()
}

fn main() {
    let data = include_str!("data.txt");

    let res = part1(data);
    println!("result#1: {res}");

    // let data = include_str!("data_tiny.txt");
    // let data = include_str!("data_example.txt");
    let data = include_str!("data.txt");
    let res = part2(data);
    println!("result#2: {res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_example() {
        let data = include_str!("data_example.txt");
        let res = part1(data);

        assert_eq!(res, 40)
    }

    #[test]
    fn test2_example() {
        let data = include_str!("data_example.txt");
        let res = part2(data);

        assert_eq!(res, 315)
    }
}
