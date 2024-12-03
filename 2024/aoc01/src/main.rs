use std::collections::HashMap;

#[allow(unused)]
const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

const INPUT: &str = include_str!("../input.txt");

fn parse_1_2(location_list: &str) -> Option<(Vec<u32>, Vec<u32>)> {
    let mut locations_a: Vec<u32> = Vec::new();
    let mut locations_b: Vec<u32> = Vec::new();
    for line in location_list.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut spl = line.split_whitespace();
        locations_a.push(spl.next()?.parse().ok()?);
        locations_b.push(spl.next()?.parse().ok()?);
    }

    return Some((locations_a, locations_b));
}

fn part_1(location_list: &str) -> Option<u32> {
    let (mut locations_a, mut locations_b) = parse_1_2(location_list)?;
    locations_a.sort();
    locations_b.sort();
    let res: u32 = locations_a
        .iter()
        .zip(locations_b)
        .map(|(a, b)| {
            return a.abs_diff(b);
        })
        .sum();

    return Some(res);
}

fn part_2(location_list: &str) -> Option<u32> {
    let (locations_a, locations_b) = parse_1_2(location_list)?;
    let mut pop_count: HashMap<u32, u32> = HashMap::new();
    for b in locations_b {
        pop_count.entry(b).and_modify(|ctr| *ctr += 1).or_insert(1);
    }
    let res: u32 = locations_a
        .iter()
        .map(|a| {
            return a * pop_count.get(a).unwrap_or(&0);
        })
        .sum();

    return Some(res);
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), Some(11));
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), Some(31));
}

fn main() {
    match part_1(INPUT) {
        Some(cv) => {
            println!("Part 1: {0}.", cv);
        }
        None => {
            println!("Part 1 failed.");
        }
    }
    match part_2(INPUT) {
        Some(cv) => {
            println!("Part 2: {0}.", cv);
        }
        None => {
            println!("Part 2 failed.");
        }
    }
    println!("Done");
}
