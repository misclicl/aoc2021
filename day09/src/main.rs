use std::collections::HashSet;

type MapCoords = (usize, usize);
type Map = Vec<Vec<u32>>;

#[derive(Debug)]
struct HeightMap {
    map: Map,
    low_points: HashSet<MapCoords>,
}

impl HeightMap {
    const NEIGHBOR_INDICES: [(i32, i32); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];

    fn size(map: &Map) -> (usize, usize) {
        (map.len(), map[0].len())
    }

    fn is_low_point(map: &Map, row_idx: usize, col_idx: usize) -> bool {
        let current_value = map[row_idx][col_idx];
        let (map_height, map_width) = Self::size(map);

        Self::NEIGHBOR_INDICES
            .iter()
            .all(|&(row_offset, col_offset)| {
                let neighbor_row = row_idx as i32 + row_offset;
                let neighbor_col = col_idx as i32 + col_offset;

                if neighbor_row < 0
                    || neighbor_col < 0
                    || neighbor_row >= map_height as i32
                    || neighbor_col >= map_width as i32
                {
                    return true;
                }

                let neighbor_value = map[neighbor_row as usize][neighbor_col as usize];
                neighbor_value > current_value
            })
    }

    fn new(data: Map) -> Self {
        let mut low_points: HashSet<MapCoords> = HashSet::new();

        for (row_idx, row) in data.iter().enumerate() {
            for (col_idx, _) in row.iter().enumerate() {
                if Self::is_low_point(&data, row_idx, col_idx) {
                    low_points.insert((row_idx, col_idx));
                }
            }
        }

        Self {
            map: data,
            low_points,
        }
    }

    fn calc_basin_area(&self, low_point: MapCoords) -> u32 {
        let (map_height, map_width) = Self::size(&self.map);

        let mut stack: Vec<MapCoords> = vec![low_point];
        let mut visited: HashSet<MapCoords> = HashSet::new();

        while !stack.is_empty() {
            let curr_coords = stack.pop().unwrap();
            let (row_idx, col_idx) = curr_coords;

            visited.insert(curr_coords);

            Self::NEIGHBOR_INDICES
                .iter()
                .for_each(|&(row_offset, col_offset)| {
                    let neighbor_row = row_idx as i32 + row_offset;
                    let neighbor_col = col_idx as i32 + col_offset;

                    if neighbor_row >= 0
                        && neighbor_col >= 0
                        && neighbor_row < map_height as i32
                        && neighbor_col < map_width as i32
                        && !visited.contains(&(neighbor_row as usize, neighbor_col as usize))
                        && self.map[neighbor_row as usize][neighbor_col as usize] != 9
                    {
                        stack.push((neighbor_row as usize, neighbor_col as usize));
                    }
                })
        }
        visited.len() as u32
    }

    fn calc_risk_level(&self) -> u32 {
        self.low_points.iter().fold(0, |acc, (y, x)| {
            let value = &self.map[*y][*x];
            acc + *value + 1
        })
    }

    fn calc_basins_risk_level(&self) -> u32 {
        let mut sorted = self
            .low_points
            .iter()
            .map(|(y, x)| self.calc_basin_area((*y, *x)))
            .collect::<Vec<_>>();

        sorted.sort_by(|a, b| b.cmp(a));
        sorted[..3].iter().product()
    }
}

fn parse_input(data: &str) -> Map {
    data.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part1(data: &str) -> u32 {
    let map_data = parse_input(data);

    let height_map = HeightMap::new(map_data);
    height_map.calc_risk_level()
}

fn part2(data: &str) -> u32 {
    let map_data = parse_input(data);

    let height_map = HeightMap::new(map_data);
    height_map.calc_basins_risk_level()
}

fn main() {
    let data = include_str!("data.txt");
    let result = part1(data);
    println!("result#1: {}", result);

    let data = include_str!("data_example.txt");
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
        assert_eq!(result, 15);
    }

    #[test]
    fn part1_large() {
        let data = include_str!("data.txt");
        let result = part1(data);
        assert_eq!(result, 496);
    }

    #[test]
    fn part2_example() {
        let data = include_str!("data_example.txt");
        let result = part2(data);
        assert_eq!(result, 1134);
    }

    #[test]
    fn part2_large() {
        let data = include_str!("data.txt");
        let result = part2(data);
        assert_eq!(result, 902880);
    }
}
