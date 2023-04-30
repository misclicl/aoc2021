use bitvec::prelude::*;

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

fn bits_slice_to_u32(bits: &BitVec<u32, Msb0>) -> u32 {
    bits.iter().fold(0, |result, bit| {
        let bit_value = if *bit { 1 } else { 0 };
        (result << 1) | bit_value
    })
}

fn str_to_bit_array(input: &str) -> BitVec<u32, Msb0> {
    let mut bit_vec = bitvec![u32, Msb0;];

    input.chars().for_each(|char| {
        let bit: bool = match char {
            '0' => false,
            '1' => true,
            _ => panic!("Invalid character found when parsing measurement: {}", char),
        };
        bit_vec.push(bit);
    });

    bit_vec
}

fn part1(data: &str) -> Result<u32, String> {
    let bit_lines: Vec<BitVec<u32, Msb0>> =
        data.lines().map(|line| str_to_bit_array(line)).collect();

    let mut gamma_bits = bitvec![u32, Msb0;];

    for i in 0..bit_lines[0].len() {
        let counts = bit_lines
            .iter()
            .fold((0, 0), |(zeros, ones), line| match line[i] {
                true => (zeros, ones + 1),
                false => (zeros + 1, ones),
            });

        gamma_bits.push(if counts.0 > counts.1 { false } else { true });
    }

    let gamma = bits_slice_to_u32(&gamma_bits);
    let mut epsilon_bits = BitVec::<u32, Msb0>::repeat(true, gamma_bits.len());
    epsilon_bits = epsilon_bits ^ gamma_bits;
    let epsilon = bits_slice_to_u32(&epsilon_bits);

    Ok(gamma * epsilon)
}

fn part2(data: &str) -> Result<u32, String> {
    let bit_lines: Vec<BitVec<u32, Msb0>> =
        data.lines().map(|line| str_to_bit_array(line)).collect();

    let oxygen_list: Vec<&BitVec<u32, Msb0>> = bit_lines.iter().collect();
    let oxygen = find_oxygen_measurement(oxygen_list, true);
    let co2_list: Vec<&BitVec<u32, Msb0>> = bit_lines.iter().collect();
    let co2 = find_oxygen_measurement(co2_list, false);

    Ok(oxygen * co2)
}

fn find_oxygen_measurement(mut list: Vec<&BitVec<u32, Msb0>>, positive_bias: bool) -> u32 {
    let mut current_idx = 0;

    while list.len() > 1 && current_idx < list[0].len() {
        let counts = list.iter().fold((0, 0), |(zeros, ones), line| {
            let current_bit = line[current_idx];
            if current_bit == true {
                (zeros, ones + 1)
            } else {
                (zeros + 1, ones)
            }
        });

        let target_value = (counts.0 > counts.1) ^ positive_bias;

        list = list
            .into_iter()
            .filter(|line| line[current_idx] == target_value)
            .collect();

        current_idx += 1;
    }

    bits_slice_to_u32(&list[0])
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
