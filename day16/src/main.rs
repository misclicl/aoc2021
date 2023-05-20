use std::collections::VecDeque;

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
        // data: u32,
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
        // length_type_id: u32,
        children: Vec<Packet>,
    },
}

fn parse_packet(packet: &[bool], pointer: &mut usize) -> Result<Packet, String> {
    let idx = *pointer;
    let version = bits_to_decimal(&packet[idx..idx + 3]);
    let type_id = bits_to_decimal(&packet[idx + 3..idx + 6]);

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
            // data: bits_to_decimal(&data_bits),
        });
    }

    let length_type_id = bits_to_decimal(&packet[idx + 6..idx + 7]);
    let mut children = Vec::new();

    if length_type_id == 0 {
        let subpacket_length = bits_to_decimal(&packet[idx + 7..idx + 22]) as usize;
        let mut subpacket_payload_start = idx + 22;
        let subpacket_payload_end = idx + 22 + subpacket_length;
        println!(
            "subpacket length: {}, {}:{}",
            subpacket_length, subpacket_payload_start, subpacket_payload_end,
        );

        while subpacket_payload_start < subpacket_payload_end {
            println!("subpacket pointer: {}", subpacket_payload_start);
            let packet = parse_packet(packet, &mut subpacket_payload_start).unwrap();
            children.push(packet);
        }

        *pointer = subpacket_payload_start;

        return Ok(Packet::Operator {
            version,
            // length_type_id,
            children,
        });
    }

    let mut subpacket_count = bits_to_decimal(&packet[idx + 7..idx + 18]) as usize;
    let mut subpacket_payload_start = idx + 18;
    println!(
        "subpacket count: {}, start: {}",
        subpacket_count, subpacket_payload_start,
    );

    while subpacket_count > 0 {
        println!("subpacket pointer: {}", subpacket_payload_start);
        let packet = parse_packet(packet, &mut subpacket_payload_start).unwrap();
        children.push(packet);

        subpacket_count -= 1;
    }

    *pointer = subpacket_payload_start;

    Ok(Packet::Operator {
        version,
        // length_type_id,
        children,
    })
}

fn parse_message(packet_str: &str) -> Result<Packet, String> {
    let binary = packet_str
        .chars()
        .filter_map(|c| u8::from_str_radix(&c.to_string(), 16).ok())
        .flat_map(|num| (0..4).rev().map(move |i| (num & (1 << i)) != 0))
        .collect::<Vec<_>>();
    println!("{binary:?}\nlen: {}", binary.len());

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

fn bits_to_decimal(bits: &[bool]) -> u32 {
    let mut out = 0;

    for &bit in bits {
        out = (out << 1) | (bit as u32);
    }

    out
}

fn part1(packet_str: &str) -> u32 {
    let packet = parse_message(packet_str).unwrap();
    calculate_version(&packet)
}

fn main() {
    let string = include_str!("data.txt");
    let result = part1(string);
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
}
