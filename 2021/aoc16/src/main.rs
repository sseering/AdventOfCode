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
const INPUT: &str = include_str!("../input.txt");

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

    const LITERAL_VALUE: u64 = 4;
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
        if length_type_id == 0 {
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

fn part_1(transmission_str: &str) -> u64 {
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

    let (_, version_sum) = part_1_walk(0, &transmission);
    return version_sum;
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

fn main() {
    println!("part 1: {}", part_1(INPUT));
    println!("Done");
}
