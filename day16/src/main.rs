use std::collections::VecDeque;

#[repr(u8)]
#[derive(Debug)]
enum OperatorType {
    Sum = 0,
    Product = 1,
    Min = 2,
    Max = 3,
    Gt = 5,
    Lt = 6,
    Eq = 7,
}

impl OperatorType {
    fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::Sum),
            1 => Some(Self::Product),
            2 => Some(Self::Min),
            3 => Some(Self::Max),
            5 => Some(Self::Gt),
            6 => Some(Self::Lt),
            7 => Some(Self::Eq),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Packet {
    /*
    Literal packet structure
    +---------+---------+-------------------------+
    | version | type ID | payload (binary number) |
    |  3bits  |  3bits  | n * 5bit (front-padded) |
    +---------+---------+-------------------------+
    */
    Literal {
        version: u32,
        data: u64,
    },
    /*
    Operator packet structure

    Length type ID == 0:
    +---------+---------+---+----------------+----------------------+
    | version | type ID | 0 | length in bits | payload (subpackets) |
    |  3bits  |  3bits  |   |     15bits     | n                    |
    +---------+---------+--------------------+----------------------+

    Length type ID == 1:
    +---------+---------+---+----------------------+----------------------+
    | version | type ID | 1 | number of subpackets | payload (subpackets) |
    |  3bits  |  3bits  |   |       11bits         |           ?          |
    +---------+---------+--------------------------+----------------------+
    */
    Operator {
        version: u32,
        tp: OperatorType,
        children: Vec<Packet>,
    },
}

fn parse_packet(packet: &[bool], pointer: &mut usize) -> Result<Packet, String> {
    let idx = *pointer;
    let version = bits_to_decimal(&packet[idx..idx + 3]) as u32;
    let type_id = bits_to_decimal(&packet[idx + 3..idx + 6]) as u8;

    println!("version: {version}");
    println!("type_id: {type_id}");

    if type_id == 4 {
        let mut packet_end_pointer: usize = idx + 6;
        let mut data_bits = Vec::new();

        loop {
            for &bit in &packet[packet_end_pointer + 1..packet_end_pointer + 5] {
                data_bits.push(bit);
            }

            // last group found
            if !packet[packet_end_pointer] {
                *pointer = packet_end_pointer + 5;
                break;
            } else {
                packet_end_pointer += 5
            }
        }

        return Ok(Packet::Literal {
            version,
            data: bits_to_decimal(&data_bits),
        });
    }

    let length_type_id = bits_to_decimal(&packet[idx + 6..idx + 7]);
    let mut children = Vec::new();

    if length_type_id == 0 {
        let subpacket_length = bits_to_decimal(&packet[idx + 7..idx + 22]) as usize;
        let mut subpacket_payload_start = idx + 22;
        let subpacket_payload_end = idx + 22 + subpacket_length;

        while subpacket_payload_start < subpacket_payload_end {
            let packet = parse_packet(packet, &mut subpacket_payload_start).unwrap();
            children.push(packet);
        }

        *pointer = subpacket_payload_start;

        return Ok(Packet::Operator {
            version,
            tp: OperatorType::from_u8(type_id).unwrap(),
            children,
        });
    }

    let mut subpacket_count = bits_to_decimal(&packet[idx + 7..idx + 18]) as usize;
    let mut subpacket_payload_start = idx + 18;

    while subpacket_count > 0 {
        let packet = parse_packet(packet, &mut subpacket_payload_start).unwrap();
        children.push(packet);

        subpacket_count -= 1;
    }

    *pointer = subpacket_payload_start;

    Ok(Packet::Operator {
        version,
        tp: OperatorType::from_u8(type_id).unwrap(),
        children,
    })
}

fn parse_message(packet_str: &str) -> Result<Packet, String> {
    let binary = packet_str
        .chars()
        .filter_map(|c| u8::from_str_radix(&c.to_string(), 16).ok())
        .flat_map(|num| (0..4).rev().map(move |i| (num & (1 << i)) != 0))
        .collect::<Vec<_>>();

    let mut pointer = 0;
    parse_packet(&binary, &mut pointer)
}

fn calculate_version(packet: &Packet) -> u32 {
    let mut result = 0;
    let mut queue = VecDeque::new();
    queue.push_back(packet);

    while let Some(current) = queue.pop_front() {
        match current {
            Packet::Literal { version, .. } => {
                result += version;
            }
            Packet::Operator {
                version, children, ..
            } => {
                // add children to the queue
                for child in children {
                    queue.push_back(child);
                }
                result += version;
            }
        }
    }

    result
}

fn bits_to_decimal(bits: &[bool]) -> u64 {
    let mut out = 0;

    for &bit in bits {
        out = (out << 1) | (bit as u64);
    }

    out
}

fn calculate(packet: &Packet) -> Option<u64> {
    match packet {
        Packet::Literal { data, .. } => Some(*data),
        Packet::Operator { tp, children, .. } => match tp {
            OperatorType::Sum => Some(children.iter().filter_map(calculate).sum()),
            OperatorType::Product => Some(children.iter().filter_map(calculate).product()),
            OperatorType::Min => children.iter().filter_map(calculate).min(),
            OperatorType::Max => children.iter().filter_map(calculate).max(),
            OperatorType::Gt | OperatorType::Lt | OperatorType::Eq => {
                if let (Some(a), Some(b)) = (calculate(&children[0]), calculate(&children[1])) {
                    Some(match tp {
                        OperatorType::Gt if a > b => 1,
                        OperatorType::Lt if a < b => 1,
                        OperatorType::Eq if a == b => 1,
                        _ => 0,
                    })
                } else {
                    None
                }
            }
        },
    }
}

fn part1(packet_str: &str) -> u32 {
    let packet = parse_message(packet_str).unwrap();
    calculate_version(&packet)
}

fn part2(packet_str: &str) -> u64 {
    let packet = parse_message(packet_str).unwrap();
    calculate(&packet).unwrap()
}

fn main() {
    let string = include_str!("data.txt");
    let result = part1(string);
    println!("RESULT: {result}");
    let result = part2(string);
    println!("RESULT: {result}");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1_examples() {
        let result = part1("8A004A801A8002F478");
        assert_eq!(result, 16);
        let result = part1("620080001611562C8802118E34");
        assert_eq!(result, 12);
        let result = part1("C0015000016115A2E0802F182340");
        assert_eq!(result, 23);
        let result = part1("A0016C880162017C3686B18A3D4780");
        assert_eq!(result, 31);
    }
    #[test]
    fn part2_examples_sum() {
        // sum
        let result = part2("C200B40A82");
        assert_eq!(result, 3);
        // product
        let result = part2("04005AC33890");
        assert_eq!(result, 54);
        // min
        let result = part2("880086C3E88112");
        assert_eq!(result, 7);
        // max
        let result = part2("CE00C43D881120");
        assert_eq!(result, 9);
        // greater than
        let result = part2("D8005AC2A8F0");
        assert_eq!(result, 1);
        // less than
        let result = part2("F600BC2D8F");
        assert_eq!(result, 0);
        // equals
        let result = part2("9C005AC2F8F0");
        assert_eq!(result, 0);
        // composite
        let result = part2("9C0141080250320F1802104A08");
        assert_eq!(result, 1);
    }
}
