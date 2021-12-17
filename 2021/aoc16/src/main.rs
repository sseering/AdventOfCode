use std::cmp::min;

#[allow(unused)]
const TEST_INPUT_A: &str = "D2FE28";

#[allow(unused)]
const TEST_INPUT_B: &str = "38006F45291200";

#[allow(unused)]
const TEST_INPUT_C: &str = "EE00D40C823060";

#[allow(unused)]
const TEST_INPUT_D: &str = "8A004A801A8002F478";

#[allow(unused)]
const TEST_INPUT_E: &str = "620080001611562C8802118E34";

#[allow(unused)]
const TEST_INPUT_F: &str = "C0015000016115A2E0802F182340";

#[allow(unused)]
const TEST_INPUT_G: &str = "A0016C880162017C3686B18A3D4780";

#[allow(unused)]
const TEST_INPUT_H: &str = "C200B40A82";

#[allow(unused)]
const TEST_INPUT_I: &str = "04005AC33890";

#[allow(unused)]
const TEST_INPUT_J: &str = "880086C3E88112";

#[allow(unused)]
const TEST_INPUT_K: &str = "CE00C43D881120";

#[allow(unused)]
const TEST_INPUT_L: &str = "D8005AC2A8F0";

#[allow(unused)]
const TEST_INPUT_M: &str = "F600BC2D8F";

#[allow(unused)]
const TEST_INPUT_N: &str = "9C005AC2F8F0";

#[allow(unused)]
const TEST_INPUT_O: &str = "9C0141080250320F1802104A08";

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

const SUM: u64 = 0;
const PRODUCT: u64 = 1;
const MINIMUM: u64 = 2;
const MAXIMUM: u64 = 3;
const LITERAL_VALUE: u64 = 4;
const GREATER_THAN: u64 = 5;
const LESS_THAN: u64 = 6;
const EQUAL_TO: u64 = 7;
const LENGTH_TYPE_ID_BITS: u64 = 0;
#[allow(unused)]
const LENGTH_TYPE_ID_PACKETS: u64 = 1;

fn hex_char_2_byte(c: char) -> u64 {
    // I could have used the rust standard library for this. But it felt like cheating on the
    // challenge. So I rolled my own.
    match c {
        '0' => 0x00,
        '1' => 0x01,
        '2' => 0x02,
        '3' => 0x03,
        '4' => 0x04,
        '5' => 0x05,
        '6' => 0x06,
        '7' => 0x07,
        '8' => 0x08,
        '9' => 0x09,
        'A' => 0x0A,
        'B' => 0x0B,
        'C' => 0x0C,
        'D' => 0x0D,
        'E' => 0x0E,
        'F' => 0x0F,
        _ => panic!("unknow char {}", c),
    }
}

fn get_bits(start_bit: usize, num_bits: usize, transmission: &Vec<u64>) -> u64 {
    // I didn't want to use external crates. Thus I use a Vec<u64> as byte array and wrote my own
    // accessor as a challenge.
    if num_bits > 64 {
        panic!();
    }

    let arr_idx = start_bit / 64;
    let start_bit = start_bit % 64;
    let mut v = (0xFFFFFFFFFFFFFFFF >> start_bit) & transmission[arr_idx];

    if start_bit + num_bits > 64 {
        let num_missing_bits = start_bit + num_bits - 64;
        let missing_bits = transmission[arr_idx + 1] >> (64 - num_missing_bits);

        v = (v << num_missing_bits) | missing_bits;
    } else {
        v = v >> (64 - num_bits - start_bit)
    }

    return v;
}

fn part_1_walk(start_bit: usize, transmission: &Vec<u64>) -> (usize, u64) {
    let mut ver = get_bits(start_bit, 3, transmission);

    let packet_type = get_bits(start_bit + 3, 3, transmission);

    let mut end: usize;
    if packet_type == LITERAL_VALUE {
        end = start_bit + 6;
        while get_bits(end, 5, transmission) > 0x0f {
            end += 5;
        }
        end += 5;
    } else {
        let length_type_id = get_bits(start_bit + 6, 1, transmission);
        if length_type_id == LENGTH_TYPE_ID_BITS {
            let length_in_bits = get_bits(start_bit + 7, 15, transmission);
            let mut walk = start_bit + 7 + 15;
            end = walk + (length_in_bits as usize);
            while walk < end {
                let (sub_end, sub_ver_sum) = part_1_walk(walk, transmission);
                walk = sub_end;
                ver += sub_ver_sum;
            }
        } else {
            let num_sub_packets = get_bits(start_bit + 7, 11, transmission);
            let mut walk = start_bit + 7 + 11;
            for _ in 0..num_sub_packets {
                let (sub_end, sub_ver_sum) = part_1_walk(walk, transmission);
                walk = sub_end;
                ver += sub_ver_sum;
            }
            end = walk;
        }
    }
    if end == 0 {
        panic!();
    }

    return (end, ver);
}

fn parse_tansmission(transmission_str: &str) -> Vec<u64> {
    let transmission_str = transmission_str.trim();
    let mut transmission: Vec<u64> = Vec::new();
    let strlen = transmission_str.len();
    let mut idx = 0;
    while idx < strlen - 1 {
        let substr = &transmission_str[idx..min(idx + 16, strlen)];
        let mut acc: u64 = 0;

        for c in substr.chars() {
            acc = acc << 4;
            acc = acc | hex_char_2_byte(c);
        }

        acc = acc << (4 * (16 - substr.len()));

        transmission.push(acc);

        idx += 16;
    }

    return transmission;
}

fn part_1(transmission_str: &str) -> u64 {
    let transmission = parse_tansmission(transmission_str);

    let (_, version_sum) = part_1_walk(0, &transmission);
    return version_sum;
}

fn part_2_walk(start_bit: usize, transmission: &Vec<u64>) -> (usize, u64) {
    let packet_type = get_bits(start_bit + 3, 3, transmission);

    let mut end: usize = start_bit + 6;
    if packet_type == LITERAL_VALUE {
        let mut value: u64 = 0;
        loop {
            let piece = get_bits(end, 5, transmission);
            end += 5;
            value = (value << 4) | (piece & 0x0F);
            if piece & 0x10 == 0 {
                break;
            }
        }
        return (end, value);
    }

    let mut subpacket_values: Vec<u64> = Vec::new();
    let length_type_id = get_bits(start_bit + 6, 1, transmission);
    if length_type_id == LENGTH_TYPE_ID_BITS {
        let length_in_bits = get_bits(start_bit + 7, 15, transmission);
        let mut walk = start_bit + 7 + 15;
        end = walk + (length_in_bits as usize);
        while walk < end {
            let (sub_end, sub_value) = part_2_walk(walk, transmission);
            walk = sub_end;
            subpacket_values.push(sub_value);
        }
    } else {
        let num_sub_packets = get_bits(start_bit + 7, 11, transmission);
        let mut walk = start_bit + 7 + 11;
        for _ in 0..num_sub_packets {
            let (sub_end, sub_value) = part_2_walk(walk, transmission);
            walk = sub_end;
            subpacket_values.push(sub_value);
        }
        end = walk;
    }

    match packet_type {
        SUM => {
            return (end, subpacket_values.iter().sum());
        }
        PRODUCT => {
            return (end, subpacket_values.iter().product());
        }
        MINIMUM => {
            return (end, *subpacket_values.iter().min().unwrap());
        }
        MAXIMUM => {
            return (end, *subpacket_values.iter().max().unwrap());
        }
        GREATER_THAN => {
            let value = if subpacket_values[0] > subpacket_values[1] {
                1
            } else {
                0
            };
            return (end, value);
        }
        LESS_THAN => {
            let value = if subpacket_values[0] < subpacket_values[1] {
                1
            } else {
                0
            };
            return (end, value);
        }
        EQUAL_TO => {
            let value = if subpacket_values[0] == subpacket_values[1] {
                1
            } else {
                0
            };
            return (end, value);
        }
        _ => {
            panic!();
        }
    }
}

fn part_2(transmission_str: &str) -> u64 {
    let transmission = parse_tansmission(transmission_str);

    let (_, transmission_value) = part_2_walk(0, &transmission);
    return transmission_value;
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT_A), 6);
}

#[test]
fn test_b() {
    assert_eq!(part_1(TEST_INPUT_B), 1 + 6 + 2);
}

#[test]
fn test_c() {
    assert_eq!(part_1(TEST_INPUT_C), 7 + 2 + 4 + 1);
}

#[test]
fn test_d() {
    assert_eq!(part_1(TEST_INPUT_D), 16);
}

#[test]
fn test_e() {
    assert_eq!(part_1(TEST_INPUT_E), 12);
}

#[test]
fn test_f() {
    assert_eq!(part_1(TEST_INPUT_F), 23);
}

#[test]
fn test_g() {
    assert_eq!(part_1(TEST_INPUT_G), 31);
}

#[test]
fn test_h() {
    assert_eq!(part_2(TEST_INPUT_H), 3);
}

#[test]
fn test_i() {
    assert_eq!(part_2(TEST_INPUT_I), 54);
}

#[test]
fn test_j() {
    assert_eq!(part_2(TEST_INPUT_J), 7);
}

#[test]
fn test_k() {
    assert_eq!(part_2(TEST_INPUT_K), 9);
}

#[test]
fn test_l() {
    assert_eq!(part_2(TEST_INPUT_L), 1);
}

#[test]
fn test_m() {
    assert_eq!(part_2(TEST_INPUT_M), 0);
}

#[test]
fn test_n() {
    assert_eq!(part_2(TEST_INPUT_N), 0);
}

#[test]
fn test_o() {
    assert_eq!(part_2(TEST_INPUT_O), 1);
}

fn main() {
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
    println!("Done");
}
