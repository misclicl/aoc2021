fn main() {
    let data = include_str!("data.txt");
    let result1 = match part1(data) {
        Ok(result) => result,
        Err(err) => panic!("Failed to calculate result for part 1: {}", err),
    };

    println!("result #1: {}", result1);

    let data = include_str!("data.txt");
    let result2 = match part2(data) {
        Ok(result) => result,
        Err(err) => panic!("Failed to calculate result for part 1: {}", err),
    };

    println!("result #2: {}", result2);
}

fn bits_slice_to_u32(bits: &[u8]) -> u32 {
    bits.iter()
        .map(|&bit| bit as u32)
        .fold(0, |result, bit| (result << 1) | bit)
}

fn invert_binary_slice(bits: &[u8]) -> Vec<u8> {
    bits.iter().map(|bit| bit ^ 1).collect()
}

fn part1(data: &str) -> Result<u32, String> {
    let lines: Vec<&str> = data.lines().collect();
    let mut gamma_bits: Vec<u8> = Vec::new();

    for (i, _) in lines[0].chars().enumerate() {
        let mut counts = (0, 0);
        for line in &lines {
            match line.chars().nth(i).unwrap() {
                '0' => counts.0 += 1,
                '1' => counts.1 += 1,
                _ => {}
            }
        }

        gamma_bits.push(if counts.0 > counts.1 { 0 } else { 1 });
    }

    let epsilon = bits_slice_to_u32(&invert_binary_slice(&gamma_bits));
    let gamma = bits_slice_to_u32(&gamma_bits);

    let product: u32 = gamma * epsilon;
    Ok(product)
}

fn find_oxygen_measurement(mut list: Vec<&Vec<u8>>, positive_bias: bool) -> u32 {
    // let mut list: Vec<&Vec<u8>> = measurements.iter().collect();
    let mut current_idx = 0;

    while list.len() > 1 && current_idx < list[0].len() {
        let mut counts = (0, 0);

        for line in &list {
            match line[current_idx] {
                0 => counts.0 += 1,
                1 => counts.1 += 1,
                _ => {}
            }
        }

        let max;

        if positive_bias {
            if counts.0 > counts.1 {
                max = 0;
            } else {
                max = 1;
            };
        } else {
            if counts.1 >= counts.0 {
                max = 0;
            } else {
                max = 1;
            };
        }

        list = list
            .into_iter()
            .filter(|line| line[current_idx] == max)
            .collect::<Vec<&Vec<u8>>>();

        current_idx += 1;
    }

    bits_slice_to_u32(list[0])
}

fn part2(data: &str) -> Result<u32, String> {
    let byte_lines: Result<Vec<Vec<u8>>, _> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| {
                    char.to_digit(10)
                        .ok_or("Failed to parse byte")
                        .map(|d| d as u8)
                })
                .collect()
        })
        .collect();

    let byte_lines = byte_lines?;
    let oxygen_list: Vec<&Vec<u8>> = byte_lines.iter().collect();
    let oxygen = find_oxygen_measurement(oxygen_list, true);
    let co2_list: Vec<&Vec<u8>> = byte_lines.iter().collect();
    let co2 = find_oxygen_measurement(co2_list, false);

    Ok(oxygen * co2)
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let result = crate::part1(include_str!("data_small.txt")).unwrap();
        assert_eq!(result, 198);
    }
    #[test]
    fn part2() {
        let result = crate::part2(include_str!("data_small.txt")).unwrap();
        assert_eq!(result, 230);
    }
}
