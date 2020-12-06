// --- Day 5: Binary Boarding ---
//
// You board your plane only to discover a new problem: you dropped your boarding pass! You aren't sure which seat is yours, and all of the flight attendants are busy with the flood of people that suddenly made it through passport control.
//
// You write a quick program to use your phone's camera to scan all of the nearby boarding passes (your puzzle input); perhaps you can find your seat through process of elimination.
//
// Instead of zones or groups, this airline uses binary space partitioning to seat people. A seat might be specified like FBFBBFFRLR, where F means "front", B means "back", L means "left", and R means "right".
//
// The first 7 characters will either be F or B; these specify exactly one of the 128 rows on the plane (numbered 0 through 127). Each letter tells you which half of a region the given seat is in. Start with the whole list of rows; the first letter indicates whether the seat is in the front (0 through 63) or the back (64 through 127). The next letter indicates which half of that region the seat is in, and so on until you're left with exactly one row.
//
// For example, consider just the first seven characters of FBFBBFFRLR:
//
//     Start by considering the whole range, rows 0 through 127.
//     F means to take the lower half, keeping rows 0 through 63.
//     B means to take the upper half, keeping rows 32 through 63.
//     F means to take the lower half, keeping rows 32 through 47.
//     B means to take the upper half, keeping rows 40 through 47.
//     B keeps rows 44 through 47.
//     F keeps rows 44 through 45.
//     The final F keeps the lower of the two, row 44.
//
// The last three characters will be either L or R; these specify exactly one of the 8 columns of seats on the plane (numbered 0 through 7). The same process as above proceeds again, this time with only three steps. L means to keep the lower half, while R means to keep the upper half.
//
// For example, consider just the last 3 characters of FBFBBFFRLR:
//
//     Start by considering the whole range, columns 0 through 7.
//     R means to take the upper half, keeping columns 4 through 7.
//     L means to take the lower half, keeping columns 4 through 5.
//     The final R keeps the upper of the two, column 5.
//
// So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.
//
// Every seat also has a unique seat ID: multiply the row by 8, then add the column. In this example, the seat has ID 44 * 8 + 5 = 357.
//
// Here are some other boarding passes:
//
//     BFFFBBFRRR: row 70, column 7, seat ID 567.
//     FFFBBBFRRR: row 14, column 7, seat ID 119.
//     BBFFBBFRLL: row 102, column 4, seat ID 820.
//
// As a sanity check, look through your list of boarding passes. What is the highest seat ID on a boarding pass?
//
// The first half of this puzzle is complete! It provides one gold star: *
// --- Part Two ---
//
// Ding! The "fasten seat belt" signs have turned on. Time to find your seat.
//
// It's a completely full flight, so your seat should be the only missing boarding pass in your list. However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft, so they'll be missing from your list as well.
//
// Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.
//
// What is the ID of your seat?

use std::str::FromStr;

#[allow(unused)]
const TEST_INPUT_A: &str = "FBFBBFFRLR";

#[allow(unused)]
const TEST_INPUT_B: &str = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

#[allow(unused)]
const INPUT: &str = include_str!("input");

#[derive(PartialEq, Eq, Debug)]
struct SeatSpec {
    row: u32,
    col: u32,
    id: u32,
}

impl SeatSpec {
    fn new(row: u32, col: u32) -> Self {
        Self {
            row,
            col,
            id: row * 8 + col,
        }
    }
}

impl FromStr for SeatSpec {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.len() != 10 {
            return Err(());
        }

        let mut row: u32 = 0;
        let mut col: u32 = 0;

        for (idx, c) in input.chars().enumerate() {
            if idx < 7 {
                if c == 'B' {
                    row = row | (1 << (6 - idx));
                } else if c != 'F' {
                    return Err(());
                }
            } else {
                if c == 'R' {
                    col = col | (1 << (9 - idx));
                } else if c != 'L' {
                    return Err(());
                }
            }
        }

        return Ok(SeatSpec::new(row, col));
    }
}

fn part_1_2_parse<'a>(boarding_passes: &'a str) -> impl Iterator<Item = SeatSpec> + 'a {
    return boarding_passes
        .lines()
        .filter_map(|s| SeatSpec::from_str(s).ok());
}

#[allow(unused)]
fn part_1<'a>(boarding_passes: &'a str) -> Option<u32> {
    part_1_2_parse(boarding_passes)
        .max_by_key(|ss| ss.id)
        .map(|ss| ss.id)
}

#[allow(unused)]
fn part_2_simple<'a>(boarding_passes: &'a str) -> Option<u32> {
    let mut ids: Vec<u32> = part_1_2_parse(boarding_passes).map(|ss| ss.id).collect();
    ids.sort_unstable();
    for idx in 1..ids.len() {
        let expected = ids[idx - 1] + 1;
        if expected != ids[idx] {
            return Some(expected);
        }
    }

    return None;
}

#[allow(unused)]
fn part_2_smart_with_xor<'a>(boarding_passes: &'a str) -> Option<u32> {
    let mut res: u32 = 0;
    let mut min: u32 = std::u32::MAX;
    let mut max: u32 = std::u32::MIN;

    for seen in part_1_2_parse(boarding_passes).map(|ss| ss.id) {
        min = std::cmp::min(min, seen);
        max = std::cmp::max(max, seen);

        res = res ^ seen;
    }

    for bitmask in min..=max {
        res = res ^ bitmask;
    }

    return Some(res);
}

#[test]
fn test_a() {
    assert_eq!(SeatSpec::new(1, 2), SeatSpec::new(1, 2));
}

#[test]
fn test_b() {
    assert_eq!(SeatSpec::new(2, 2), SeatSpec::new(1 + 1, 2));
}

#[test]
fn test_c() {
    assert_ne!(SeatSpec::new(1, 2), SeatSpec::new(2, 2));
}

#[test]
fn test_d() {
    assert_eq!(SeatSpec::new(44, 5).id, 357);
    assert_eq!(SeatSpec::new(70, 7).id, 567);
    assert_eq!(SeatSpec::new(14, 7).id, 119);
    assert_eq!(SeatSpec::new(102, 4).id, 820);
}

#[test]
fn test_e() {
    let mut t = part_1_2_parse(TEST_INPUT_A);
    assert_eq!(t.next(), Some(SeatSpec::new(44, 5)));
    assert_eq!(t.next(), None);
}

#[test]
fn test_f() {
    let mut t = part_1_2_parse(TEST_INPUT_B);
    assert_eq!(t.next(), Some(SeatSpec::new(70, 7)));
    assert_eq!(t.next(), Some(SeatSpec::new(14, 7)));
    assert_eq!(t.next(), Some(SeatSpec::new(102, 4)));
    assert_eq!(t.next(), None);
}

#[test]
fn test_g() {
    assert_eq!(part_1(TEST_INPUT_A), Some(357));
}

#[test]
fn test_h() {
    assert_eq!(part_1(TEST_INPUT_B), Some(820));
}

#[test]
fn test_i() {
    assert_eq!(part_2_simple(INPUT), part_2_smart_with_xor(INPUT));
}

fn main() {
    println!("Part 1: {}", part_1(INPUT).unwrap());
    println!("Part 2: {}", part_2_smart_with_xor(INPUT).unwrap());
    println!("done.");
}
