use std::collections::HashSet;

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

fn priority_score(c: char) -> u32 {
    if ('a'..='z').contains(&c) {
        return 1 + (u32::from(c) - u32::from('a'));
    }
    if ('A'..='Z').contains(&c) {
        return 27 + (u32::from(c) - u32::from('A'));
    }
    panic!();
}

fn rucksack_rearrangement_priority(rucksack: &str) -> u32 {
    let l = rucksack.len() / 2;
    let compartment_a = &rucksack[0..l];
    let compartment_b = &rucksack[l..];

    let a: HashSet<char> = compartment_a.chars().collect();
    let mut already_scored: HashSet<char> = HashSet::new();

    return compartment_b
        .chars()
        .map(|bb| {
            if a.contains(&bb) && !already_scored.contains(&bb) {
                already_scored.insert(bb);
                return priority_score(bb);
            } else {
                return 0;
            }
        })
        .sum();
}

fn part_1(rucksacks: &str) -> u32 {
    return rucksacks.lines().map(rucksack_rearrangement_priority).sum();
}

fn part_2(rucksacks: &str) -> u32 {
    let mut lines = rucksacks.lines();

    let mut priority_sum = 0;

    let mut aa = lines.next();
    while let Some(a) = aa {
        let b = lines.next().unwrap();
        let c = lines.next().unwrap();

        let aaa: HashSet<char> = a.chars().collect();
        let bbb: HashSet<char> = b.chars().collect();
        let ccc: HashSet<char> = c.chars().collect();

        let i = &(&aaa & &bbb) & &ccc;
        if i.len() != 1 {
            panic!();
        }

        priority_sum += priority_score(i.into_iter().next().unwrap());

        aa = lines.next();
    }

    return priority_sum;
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), 157);
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), 70);
}

fn main() {
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
    println!("done.");
}
