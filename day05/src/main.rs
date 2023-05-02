use std::{
    cmp::{max, min},
    collections::HashMap,
    error,
    io::{Error, ErrorKind},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Pair(Point, Point);

fn parse_tuple(s: &str) -> Result<Point, Box<dyn error::Error>> {
    let nums: Vec<&str> = s.split(",").collect();

    if nums.len() != 2 {
        return Err(Box::new(Error::new(ErrorKind::Other, "invalid arguments")));
    }

    let x = nums[0].parse::<i32>()?;
    let y = nums[1].parse::<i32>()?;

    Ok(Point { x, y })
}

fn parse_line(s: &str) -> Result<Pair, Box<dyn error::Error>> {
    let mut points = s.split(" -> ");
    let point_a = points.next().unwrap();
    let point_b = points.next().unwrap();

    if let Some(_) = points.next() {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "Line parsing failed: invalid arguments",
        )));
    }

    let point_a = parse_tuple(point_a).unwrap();
    let point_b = parse_tuple(point_b).unwrap();

    Ok(Pair(point_a, point_b))
}

fn parse_lines(data: &str) -> Vec<Pair> {
    data.lines().map(|line| parse_line(line).unwrap()).collect()
}

fn line_to_points(pair: &Pair) -> Option<Vec<Point>> {
    let point1 = &pair.0;
    let point2 = &pair.1;

    if pair.0.x == pair.1.x {
        let min_y = min(point1.y, point2.y);
        let max_y = max(point1.y, point2.y);

        return Some(
            (min_y..=max_y)
                .map(|y| Point { x: pair.0.x, y })
                .collect::<Vec<Point>>(),
        );
    } else if pair.0.y == pair.1.y {
        let min_x = min(point1.x, point2.x);
        let max_x = max(point1.x, point2.x);
        return Some(
            (min_x..=max_x)
                .map(|x| Point { x, y: pair.0.y })
                .collect::<Vec<Point>>(),
        );
    }

    None
}

fn part1(data: &str) -> usize {
    let pairs = parse_lines(data);

    let mut board: HashMap<Point, u32> = HashMap::new();
    let mut count: usize = 0;

    pairs
        .iter()
        .filter_map(|pair| {
            let points = line_to_points(pair);
            // println!("Line: {:?}, Points: {:?}", pair, points);
            return points;
        })
        .for_each(|points| {
            for point in points {
                let new_value = board.get(&point).unwrap_or(&0) + 1;

                if new_value == 2 {
                    count += 1;
                }

                board.insert(point, new_value);
            }
        });

    return count;
}

fn main() {
    let _data = include_str!("data_small.txt");
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
        assert_eq!(result, 5);
    }
}
