use regex::Regex;
use std::collections::VecDeque;

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

struct CraneMove {
    ammount: u32,
    from_idx: usize,
    to_idx: usize,
}

impl CraneMove {
    fn new(_ammount: u32, from_no: usize, to_no: usize) -> Self {
        Self {
            ammount: _ammount,
            from_idx: from_no - 1,
            to_idx: to_no - 1,
        }
    }
}

fn parse(rearrangement: &str) -> Option<(Vec<CraneMove>, Vec<VecDeque<char>>)> {
    let move_re = Regex::new(r"move\s+(\d+)\s+from\s+(\d+)\s+to\s+(\d+)").ok()?;
    let crate_re = Regex::new(r"\[(.)\]").ok()?;

    let mut parsing_crates = true;
    let mut crane_moves: Vec<CraneMove> = Vec::new();
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for line in rearrangement.lines() {
        if line.len() == 0 {
            parsing_crates = false;
            continue;
        }

        if parsing_crates {
            if line.contains("[") {
                for crane_crate in crate_re.captures_iter(line) {
                    let cap = crane_crate.get(0)?;
                    let stack_idx: usize = cap.start() / 4;
                    let crate_label: char = cap.as_str().chars().skip(1).next()?;
                    while stacks.len() <= stack_idx {
                        stacks.push(VecDeque::new());
                    }
                    stacks[stack_idx].push_back(crate_label);
                }
            }
        } else {
            let caps = move_re.captures(line)?;
            let a: u32 = caps[1].parse().ok()?;
            let b: usize = caps[2].parse().ok()?;
            let c: usize = caps[3].parse().ok()?;
            crane_moves.push(CraneMove::new(a, b, c));
        }
    }

    return Some((crane_moves, stacks));
}

fn stacks_to_string(stacks: &Vec<VecDeque<char>>) -> String {
    let mut result = String::new();
    for s in stacks {
        if let Some(&crate_label) = s.front() {
            result.push(crate_label);
        }
    }

    return result;
}

fn part_1(rearrangement: &str) -> Option<String> {
    let (crane_moves, mut stacks) = parse(rearrangement)?;

    for cm in crane_moves {
        for _ in 0..(cm.ammount) {
            let crate_label = stacks[cm.from_idx].pop_front()?;
            stacks[cm.to_idx].push_front(crate_label);
        }
    }

    return Some(stacks_to_string(&stacks));
}

fn part_2(rearrangement: &str) -> Option<String> {
    let (crane_moves, mut stacks) = parse(rearrangement)?;

    for cm in crane_moves {
        let mut tmp: VecDeque<char> = VecDeque::new();
        for _ in 0..(cm.ammount) {
            tmp.push_back(stacks[cm.from_idx].pop_front()?);
        }
        for _ in 0..(cm.ammount) {
            stacks[cm.to_idx].push_front(tmp.pop_back()?);
        }
    }

    return Some(stacks_to_string(&stacks));
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), Some(String::from("CMZ")));
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), Some(String::from("MCD")));
}

fn main() {
    println!("part 1: {}", part_1(INPUT).unwrap_or("None".to_string()));
    println!("part 2: {}", part_2(INPUT).unwrap_or("None".to_string()));
    println!("done.");
}
