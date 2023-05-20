use std::collections::{HashMap, HashSet};

use priority_queue::DoublePriorityQueue;
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

    fn reconstruct_path(&self, path_map: &HashMap<Position, Option<Position>>) -> Vec<Position> {
        let mut result: Vec<Position> = Vec::new();

        let grid_size = self.size();
        let mut next_node = Some(Position {
            x: grid_size.0 - 1,
            y: grid_size.1 - 1,
        });

        while let Some(node_position) = next_node {
            result.push(node_position);
            next_node = path_map.get(&node_position).cloned().unwrap_or(None);
        }

        result.reverse();
        result
    }

    /*

    Explaining the Dijkstra algorithm to myself

    Consider the following graph with node weights:

    1 ---- 5 ---- 6
           |      |
           3a-----3b
           |      |
           8------1

    The shortest path is: 1-5-3-3-1 (overall weight is 13).

    Dijkstra's algorithm uses a priority queue to always select the node with
    the smallest known distance from the start. It updates the distances of its
    neighbors, replacing the known distance if a shorter one is found.
    This gradually reveals the shortest path to all nodes.

    node path_weight* prev_node queue
    ----------------------------------
    1    0            None      [5]
    5    5            Some(1)   [3a, 6]
    3a   8            Some(5)   [3b, 6, 8]
    3b   11           Some(3a)  [1, 6, 6, 8]
    ...and so on
    -------------------------

    Reconsructing path: 1 -> 3b -> 3a -> 5 -> 1
    */
    fn find_path(&self) -> u32 {
        let mut pq = DoublePriorityQueue::new();
        let mut prev_map: HashMap<Position, Option<Position>> = HashMap::new();
        let mut weights: HashMap<Position, u32> = HashMap::new();
        let mut visited = HashSet::new();

        let start_position = Position { x: 0, y: 0 };
        weights.insert(start_position, 0);
        pq.push(start_position, 0);

        while let Some((position_current, _)) = pq.pop_min() {
            visited.insert(position_current);
            let weight_current = *weights.get(&position_current).unwrap();

            for neighbor in self.get_neighbors(&position_current) {
                let weight_neighbor = *weights.get(&neighbor).unwrap_or(&u32::MAX);
                let candidate_value = weight_current + self.get_cell_value(neighbor);

                if candidate_value < weight_neighbor && !visited.contains(&neighbor) {
                    weights.insert(neighbor, candidate_value);
                    prev_map.insert(neighbor, Some(position_current));
                    if pq.change_priority(&neighbor, candidate_value).is_none() {
                        pq.push(neighbor, candidate_value);
                    }
                }
            }
        }

        let path = self.reconstruct_path(&prev_map);

        path.iter().skip(1).map(|p| self.grid[p.y][p.x]).sum()
    }

    fn _visualize_path(matrix: &[Vec<u32>], path: &[Position]) {
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

    fn _visualize_point_map(&self, danger_table: &HashMap<Position, u32>) {
        let grid_size = self.size();

        let mut printable = vec![vec![0; grid_size.0]; grid_size.1];

        for (key, entry) in danger_table {
            printable[key.y][key.x] = *entry;
        }

        print_matrix(&printable);
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

fn extend_matrix(matrix: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let (size_x, size_y) = get_matrix_size(&matrix);
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

    matrix_extended
}

fn part1(data: &str) -> u32 {
    let matrix = parse_input(data);

    let path_finder = PathFinder::new(matrix);
    path_finder.find_path()
}

fn part2(data: &str) -> u32 {
    let matrix_extended = extend_matrix(parse_input(data));

    let path_finder = PathFinder::new(matrix_extended);
    path_finder.find_path()
}

fn main() {
    let data = include_str!("data.txt");

    let res = part1(data);
    println!("result#1: {res}");

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
