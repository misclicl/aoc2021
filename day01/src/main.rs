use std::io;

fn part1() -> Result<usize, String> {
    let lines = include_str!("data.txt").lines();
    let counter = lines
        .filter_map(|v| v.parse::<i32>().ok())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count();
    Ok(counter)
}

fn part2() -> Result<usize, String> {
    let lines = include_str!("data.txt").lines();

    let counter = lines
        .filter_map(|v| v.parse::<i32>().ok())
        .collect::<Vec<i32>>()
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count();

    Ok(counter)
}

fn main() -> io::Result<()> {
    let result1 = part1().unwrap();
    println!("result#1: {}", result1); // 1121
    let result2 = part2().unwrap();
    println!("result#2: {}", result2); // 1065
    Ok(())
}
