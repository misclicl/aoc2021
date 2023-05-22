use std::cmp::Ordering;

fn parse_input(input: &str) -> Rect {
    let mut ranges_iter = input
        .trim_start_matches("target area: x=")
        .split(", y=")
        .map(|range_str| {
            let parts = range_str.split("..").collect::<Vec<_>>();

            (
                parts[0].parse::<i64>().unwrap(),
                parts[1].parse::<i64>().unwrap(),
            )
        });

    let x_range = ranges_iter.next().expect("Expected x range");
    let y_range = ranges_iter.next().expect("Expected y range");

    Rect {
        x: x_range.0,
        y: y_range.0,
        width: (x_range.1 - x_range.0).unsigned_abs() as u32,
        height: (y_range.1 - y_range.0).unsigned_abs() as u32,
    }
}

#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Rect {
    x: i64,
    y: i64,
    width: u32,
    height: u32,
}

fn gaussian(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn get_velocity_range_x(target_area: &Rect) -> (i64, i64) {
    let max_x = target_area.x + target_area.width as i64;
    let mut min_x = 0;
    let mut min_range_end = max_x;

    while min_x < min_range_end {
        let mid = (min_x + min_range_end) / 2;

        if gaussian(mid) < target_area.x {
            min_x = mid + 1;
        } else {
            min_range_end = mid;
        }
    }

    (min_x, max_x)
}

fn get_max_velocity_y(target_area: &Rect) -> i64 {
    /*
        Case A: target are is below X axis

                          ^  -  v
                       ^           |
                       |           v
                    ^                 |
           vel == 3 |                 |
                    |                 v
        * S (0, 0)------------------------------
                        +----------+     |
                        |          |     |
                        |          |     |
                        +----------+     v min_y == -4

        Setting vertical speed larger than this will lead to a guaranteed
        overshooting on the way down

        Cases other than A turned out to be useless
    */
    if target_area.y + (target_area.height as i64) < 0 {
        return target_area.y.abs() - 1;
    }
    /*
        Case B: target are is above X axis

        Similar to A, but now I'm restricted by y_max
    */

    if target_area.y > 0 {
        return target_area.y + target_area.height as i64;
    }

    // Otherwise it's always going to land on target
    i64::MAX
}

fn adjust_velocity(vel_x: i64) -> i64 {
    match vel_x.cmp(&0) {
        Ordering::Greater => vel_x - 1,
        Ordering::Less => vel_x + 1,
        Ordering::Equal => 0,
    }
}

fn will_collide(start_velocity: Vec2, target: &Rect) -> bool {
    let (mut vel_x, mut vel_y) = (start_velocity.x, start_velocity.y);
    let (mut pos_x, mut pos_y) = (0, 0);

    while pos_x <= target.x + (target.width as i64) && pos_y >= target.y {
        if pos_x >= target.x && pos_y <= target.y + (target.height as i64) {
            return true;
        }

        pos_x += vel_x;
        pos_y += vel_y;

        vel_y -= 1;
        vel_x = adjust_velocity(vel_x);
    }

    false
}

fn part1(input: &str) -> i64 {
    let target = parse_input(input);
    let max_velocity = get_max_velocity_y(&target);
    gaussian(max_velocity)
}

fn part2(input: &str) -> u32 {
    let target = parse_input(input);

    println!("{:?}", target);

    let min_velocity_y = target.y;
    let max_velocity_y = get_max_velocity_y(&target);

    let (min_velocity_x, max_velocity_x) = get_velocity_range_x(&target);
    let mut counter = 0;

    for x in min_velocity_x..=max_velocity_x {
        for y in min_velocity_y..=max_velocity_y {
            if will_collide(Vec2 { x, y }, &target) {
                counter += 1;
            }
        }
    }

    counter
}

fn main() {
    let input = include_str!("data.txt");
    part1(input);
    part2(input);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1_examples() {
        let input = include_str!("data.txt");
        let result = part1(input);
        assert_eq!(result, 3160);
    }
    #[test]
    fn part2_examples() {
        let input = include_str!("data_example.txt");
        let result = part2(input);
        assert_eq!(result, 112);
    }
}
