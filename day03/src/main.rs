fn main() {
    let data = include_str!("data.txt");
    let result1 = match part1(data) {
        Ok(result) => result,
        Err(err) => panic!("Failed to calculate result for part 1: {}", err),
    };

    println!("result #1: {}", result1);
}

fn bits_to_u32(bits: &[u8]) -> u32 {
    bits.iter()
        .map(|&bit| bit as u32)
        .fold(0, |result, bit| (result << 1) | bit)
}

fn invert_binary_vector(bits: &[u8]) -> Vec<u8> {
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

    let epsilon = bits_to_u32(&invert_binary_vector(&gamma_bits));
    let gamma = bits_to_u32(&gamma_bits);

    println!("gamma: {:?}, epsilon bits: {:?}", gamma, epsilon);

    let product: u32 = gamma * epsilon;
    Ok(product)
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let result = crate::part1(include_str!("data_small.txt")).unwrap();
        assert_eq!(result, 198);
    }
}
