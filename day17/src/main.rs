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

fn get_max_y_velocity(target_area: Rect) -> i64 {
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

fn parse_input(input: &str) -> Rect {
    let ranges = input
        .trim_start_matches("target area: x=")
        .split(", y=")
        .map(|range_str| {
            let parts = range_str.split("..").collect::<Vec<_>>();

            (
                parts[0].parse::<i64>().unwrap(),
                parts[1].parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    Rect {
        x: ranges[0].0,
        y: ranges[1].0,
        width: (ranges[0].1 - ranges[0].0).unsigned_abs() as u32,
        height: (ranges[1].1 - ranges[1].0).unsigned_abs() as u32,
    }
}

fn part1(input: &str) -> i64 {
    let target = parse_input(input);
    let max_velocity = get_max_y_velocity(target);
    gaussian(max_velocity)
}

fn main() {
    let input = include_str!("data.txt");
    part1(input);
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
}
