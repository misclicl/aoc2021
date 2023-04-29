use std::str::FromStr;

#[derive(Debug)]
enum Movement {
    Vertical(i32),
    Horizontal(i32),
}

impl FromStr for Movement {
    type Err = String;

    fn from_str(input: &str) -> Result<Movement, Self::Err> {
        let mut entries = input.split_whitespace();

        let direction = match entries.next() {
            Some(direction) => direction,
            None => return Err("Failed to parse direction".to_string()),
        };

        let value = match entries.next() {
            Some(value_str) => match value_str.parse::<i32>() {
                Ok(value) => value,
                Err(_) => return Err("Failed to parse value".to_string()),
            },
            None => return Err("Couldn't parse value".to_string()),
        };

        if entries.next().is_some() {
            return Err("Too many arguments".to_string());
        }

        match direction {
            "forward" => Ok(Movement::Horizontal(value)),
            "up" => Ok(Movement::Vertical(-value)),
            "down" => Ok(Movement::Vertical(value)),
            _ => Err("Invalid instruction recieved".to_string()),
        }
    }
}

fn part1(data: &str) -> Result<i32, String> {
    let course_instructions: Vec<Movement> = data
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

fn part2(data: &str) -> Result<i32, String> {
    let course_instructions: Vec<Movement> = data
        .lines()
        .map(|line| Movement::from_str(line).unwrap())
        .collect();

    let (pos_x, pos_y, _) = course_instructions
        .iter()
        .fold((0, 0, 0), |(pos_x, pos_y, aim), m| match m {
            Movement::Horizontal(value) => (pos_x + value, pos_y + value * aim, aim),
            Movement::Vertical(value) => (pos_x, pos_y, aim + value),
        });

    Ok(pos_x * pos_y)
}

fn main() {
    let data = include_str!("data.txt");
    let result1 = part1(data).expect("Failed to calculate result for part 1");
    println!("result #1: {}", result1);
    let result2 = part2(data).expect("Failed to calculate result for part 1");
    println!("result #2: {}", result2);
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let result = crate::part1(include_str!("data_small.txt")).unwrap();
        assert_eq!(result, 150);
    }

    #[test]
    fn part2() {
        let result = crate::part2(include_str!("data_small.txt")).unwrap();
        assert_eq!(result, 900);
    }
}
