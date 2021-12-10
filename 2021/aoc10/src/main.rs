use std::collections::VecDeque;

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

fn syntax_checker_score(c: char) -> Option<u32> {
    match c {
        ')' => Some(3),
        ']' => Some(57),
        '}' => Some(1197),
        '>' => Some(25137),
        _ => None,
    }
}

fn autocomplete_score(c: char) -> Option<u32> {
    match c {
        ')' => Some(1),
        ']' => Some(2),
        '}' => Some(3),
        '>' => Some(4),
        _ => None,
    }
}

fn closing_char(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn line_syntax_error_score(line: &str) -> (u32, VecDeque<char>) {
    let mut expected: VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        if let Some(cc) = closing_char(c) {
            expected.push_front(cc);
        } else {
            if c != expected.pop_front().unwrap() {
                match syntax_checker_score(c) {
                    Some(line_score) => {
                        return (line_score, expected);
                    }
                    None => {
                        panic!("unexpected char {}", c);
                    }
                }
            }
        }
    }
    return (0, expected);
}

fn part_1(navigation_subsystem: &str) -> u32 {
    let mut syntax_error_score = 0;

    for line in navigation_subsystem.lines() {
        let (line_score, _) = line_syntax_error_score(line);
        syntax_error_score += line_score;
    }

    return syntax_error_score;
}

fn part_2(navigation_subsystem: &str) -> u64 {
    let mut autocomplete_scores: Vec<u64> = Vec::new();

    for line in navigation_subsystem.lines() {
        let (line_score, missing_closing) = line_syntax_error_score(line);

        if line_score != 0 {
            continue;
        }

        let mut line_autocomplete_score: u64 = 0;
        for c in missing_closing {
            line_autocomplete_score *= 5;
            match autocomplete_score(c) {
                Some(cs) => {
                    line_autocomplete_score += cs as u64;
                }
                None => {
                    panic!("unexpected char {}", c);
                }
            }
        }

        autocomplete_scores.push(line_autocomplete_score);
    }

    let median_idx = autocomplete_scores.len() / 2;

    autocomplete_scores.select_nth_unstable(median_idx);

    return autocomplete_scores[median_idx];
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), 26397);
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), 288957);
}

fn main() {
    println!("part 1: {0}", part_1(INPUT));
    println!("part 2: {0}", part_2(INPUT));
    println!("Done.");
}
