use std::str::FromStr;

#[derive(Debug)]
enum Movement {
    Vertical(i32),
    Horizontal(i32),
}

impl FromStr for Movement {
    type Err = String;

    fn from_str(input: &str) -> Result<Movement, Self::Err> {
        let entries: Vec<&str> = input.split_whitespace().collect();

        if entries.len() != 2 {
            todo!();
        }

        let direction = entries[0];
        let value = entries[1]
            .parse::<i32>()
            .expect("Failed to parse intruction value");

        match direction {
            "forward" => Ok(Movement::Horizontal(value)),
            "up" => Ok(Movement::Vertical(-value)),
            "down" => Ok(Movement::Vertical(value)),
            _ => Err("Invalid instruction recieved".to_string()),
        }
    }
}

fn part1() -> Result<i32, String> {
    let course_instructions: Vec<Movement> = include_str!("data.txt")
        .lines()
        .map(|line| Movement::from_str(line).unwrap())
        .collect();

    let (pos_x, pos_y) = course_instructions
        .iter()
        .fold((0, 0), |(x, y), m| match m {
            Movement::Horizontal(value) => (x + value, y),
            Movement::Vertical(value) => (x, y + value),
        });

    println!("pos_x: {}", pos_x);
    println!("pos_y: {}", pos_y);

    Ok(pos_x * pos_y)
}

fn main() {
    let result1 = part1().expect("Failed to calculate result for part 1");
    println!("result #1: {}", result1);
}
