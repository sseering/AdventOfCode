use std::collections::HashSet;

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT_A: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

#[allow(unused)]
const TEST_INPUT_B: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

#[allow(unused)]
const TEST_INPUT_C: &str = "nppdvjthqldpwncqszvftbrmjlhg";

#[allow(unused)]
const TEST_INPUT_D: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

#[allow(unused)]
const TEST_INPUT_E: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

fn part_1(datastream_buffer: &str) -> Option<usize> {
    for (idx, window) in datastream_buffer
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .enumerate()
    {
        let mut window = window.iter();
        let a: char = *window.next()?;
        let b: char = *window.next()?;
        let c: char = *window.next()?;
        let d: char = *window.next()?;

        if a != b && a != c && a != d && b != c && b != d && c != d {
            return Some(idx + 4);
        }
    }
    return None;
}

fn part_2(datastream_buffer: &str) -> Option<usize> {
    'next_window: for (idx, window) in datastream_buffer
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .enumerate()
    {
        let mut dupes_check: HashSet<char> = HashSet::new();
        for c in window {
            if dupes_check.contains(c) {
                continue 'next_window;
            }
            dupes_check.insert(*c);
        }

        return Some(idx + 14);
    }
    return None;
}

#[test]
fn test_aa() {
    assert_eq!(part_1(TEST_INPUT_A), Some(7));
}

#[test]
fn test_ab() {
    assert_eq!(part_1(TEST_INPUT_B), Some(5));
}

#[test]
fn test_ac() {
    assert_eq!(part_1(TEST_INPUT_C), Some(6));
}

#[test]
fn test_ad() {
    assert_eq!(part_1(TEST_INPUT_D), Some(10));
}

#[test]
fn test_ae() {
    assert_eq!(part_1(TEST_INPUT_E), Some(11));
}

#[test]
fn test_ba() {
    assert_eq!(part_2(TEST_INPUT_A), Some(19));
}

#[test]
fn test_bb() {
    assert_eq!(part_2(TEST_INPUT_B), Some(23));
}

#[test]
fn test_bc() {
    assert_eq!(part_2(TEST_INPUT_C), Some(23));
}

#[test]
fn test_bd() {
    assert_eq!(part_2(TEST_INPUT_D), Some(29));
}

#[test]
fn test_be() {
    assert_eq!(part_2(TEST_INPUT_E), Some(26));
}

fn main() {
    println!("part 1: {}", part_1(INPUT).unwrap());
    println!("part 2: {}", part_2(INPUT).unwrap());
    println!("done.");
}
