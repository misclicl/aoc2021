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
    let nums: Vec<&str> = s.split(',').collect();

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

    if points.next().is_some() {
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

    if point1.x == point2.x {
        let min_y = min(point1.y, point2.y);
        let max_y = max(point1.y, point2.y);

        return Some(
            (min_y..=max_y)
                .map(|y| Point { x: point1.x, y })
                .collect::<Vec<Point>>(),
        );
    } else if point1.y == point2.y {
        let min_x = min(point1.x, point2.x);
        let max_x = max(point1.x, point2.x);
        return Some(
            (min_x..=max_x)
                .map(|x| Point { x, y: point2.y })
                .collect::<Vec<Point>>(),
        );
    }

    None
}

fn line_to_points_advanced(pair: &Pair) -> Option<Vec<Point>> {
    let mut points = line_to_points(pair).unwrap_or(Vec::new());

    let (left_most, right_most) = if pair.0.x < pair.1.x {
        (&pair.0, &pair.1)
    } else {
        (&pair.1, &pair.0)
    };

    if (left_most.x - right_most.x).unsigned_abs() == (left_most.y - right_most.y).unsigned_abs() {
        for x in left_most.x..=right_most.x {
            let y = if left_most.y < right_most.y {
                left_most.y + x - left_most.x
            } else {
                left_most.y - (x - left_most.x)
            };
            points.push(Point { x, y });
        }
    }

    if points.is_empty() {
        return None;
    }

    Some(points)
}

fn part1(data: &str) -> usize {
    let mut board: HashMap<Point, u32> = HashMap::new();

    let intersection_count =
        parse_lines(data)
            .iter()
            .filter_map(line_to_points)
            .fold(0, |mut acc, points| {
                for point in points {
                    let point_count = board.entry(point).or_insert(0);
                    *point_count += 1;

                    if *point_count == 2 {
                        acc += 1;
                    }
                }

                acc
            });

    intersection_count
}

fn part2(data: &str) -> usize {
    let mut board: HashMap<Point, u32> = HashMap::new();

    parse_lines(data)
        .iter()
        .filter_map(line_to_points_advanced)
        .fold(0, |mut acc, points| {
            for point in points {
                let new_value = board.entry(point).or_insert(0);
                *new_value += 1;

                if *new_value == 2 {
                    acc += 1;
                }
            }

            acc
        })
}

fn main() {
    let _data = include_str!("data_small.txt");
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
        let data = include_str!("data_small.txt");
        let result = part1(data);
        assert_eq!(result, 5);
    }
    #[test]
    fn part2_example() {
        let data = include_str!("data_small.txt");
        let result = part2(data);
        assert_eq!(result, 12);
    }
}
