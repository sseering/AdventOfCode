use std::cmp::{max, min};
use std::collections::HashMap;

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");
#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

fn parse_input(polymer_descr: &str) -> (&str, HashMap<(char, char), char>) {
    let mut lines = polymer_descr.lines();
    let polymer_template = lines.next().unwrap();

    if lines.next().unwrap().len() != 0 {
        panic!();
    }

    let mut pair_insertion_rules: HashMap<(char, char), char> = HashMap::new();

    for line in lines {
        if let [a, b] = line.split(" -> ").collect::<Vec<&str>>()[..] {
            if a.len() != 2 || b.len() != 1 {
                panic!();
            }

            let mut ac = a.chars();
            let a0 = ac.next().unwrap();
            let a1 = ac.next().unwrap();
            let b0 = b.chars().nth(0).unwrap();

            pair_insertion_rules.insert((a0, a1), b0);
        } else {
            panic!();
        }
    }

    return (polymer_template, pair_insertion_rules);
}

fn part_1_2(polymer_descr: &str, iterations: u32) -> u64 {
    let (polymer_template, pair_insertion_rules) = parse_input(polymer_descr);

    let mut letter_freqs: HashMap<char, u64> = HashMap::new();
    let mut pair_freqs: HashMap<(char, char), u64> = HashMap::new();

    let mut a_chars = polymer_template.chars().peekable();

    let first_char: char = *a_chars.peek().unwrap();
    letter_freqs.insert(first_char, 1);

    for (a, b) in a_chars.zip(polymer_template.chars().skip(1)) {
        let counter = pair_freqs.entry((a, b)).or_insert(0);
        *counter += 1;
        let counter = letter_freqs.entry(b).or_insert(0);
        *counter += 1;
    }

    for _ in 0..iterations {
        let old_pair_freqs = pair_freqs.clone();
        for (pair, insertion) in &pair_insertion_rules {
            let num_pairs_to_replace = old_pair_freqs.get(pair).map(|&n| n).unwrap_or(0);

            if num_pairs_to_replace == 0 {
                continue;
            }

            let counter = pair_freqs.entry(*pair).or_insert(0);
            *counter -= num_pairs_to_replace;
            let counter = pair_freqs.entry((pair.0, *insertion)).or_insert(0);
            *counter += num_pairs_to_replace;
            let counter = pair_freqs.entry((*insertion, pair.1)).or_insert(0);
            *counter += num_pairs_to_replace;

            let counter = letter_freqs.entry(*insertion).or_insert(0);
            *counter += num_pairs_to_replace;
        }
    }

    // println!("{0:?}", letter_freqs);
    // println!("{0:?}", pair_freqs);
    // println!("{0:?}", pair_insertion_rules);

    let mut most: u64 = 0;
    let mut least: u64 = u64::MAX;
    for &freq in letter_freqs.values() {
        if freq == 0 {
            continue;
        }
        most = max(most, freq);
        least = min(least, freq);
    }

    if most == 0 || least == u64::MAX {
        panic!();
    }

    return most - least;
}

#[test]
fn test_a() {
    assert_eq!(part_1_2(TEST_INPUT, 10), 1588);
}

#[test]
fn test_b() {
    assert_eq!(part_1_2(TEST_INPUT, 40), 2188189693529);
}

fn main() {
    println!("part 1: {}", part_1_2(INPUT, 10));
    println!("part 2: {}", part_1_2(INPUT, 40));
    println!("Done.");
}
