// --- Day 6: Custom Customs ---
//
// As your flight approaches the regional airport where you'll switch to a much larger plane, customs declaration forms are distributed to the passengers.
//
// The form asks a series of 26 yes-or-no questions marked a through z. All you need to do is identify the questions for which anyone in your group answers "yes". Since your group is just you, this doesn't take very long.
//
// However, the person sitting next to you seems to be experiencing a language barrier and asks if you can help. For each of the people in their group, you write down the questions for which they answer "yes", one per line. For example:
//
// abcx
// abcy
// abcz
//
// In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z. (Duplicate answers to the same question don't count extra; each question counts at most once.)
//
// Another group asks for your help, then another, and eventually you've collected answers from every group on the plane (your puzzle input). Each group's answers are separated by a blank line, and within each group, each person's answers are on a single line. For example:
//
// abc
//
// a
// b
// c
//
// ab
// ac
//
// a
// a
// a
// a
//
// b
//
// This list represents answers from five groups:
//
//     The first group contains one person who answered "yes" to 3 questions: a, b, and c.
//     The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
//     The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
//     The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
//     The last group contains one person who answered "yes" to only 1 question, b.
//
// In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.
//
// For each group, count the number of questions to which anyone answered "yes". What is the sum of those counts?
//
// To begin, get your puzzle input.
//
// The first half of this puzzle is complete! It provides one gold star: *
// --- Part Two ---
//
// As you finish the last group's customs declaration, you notice that you misread one word in the instructions:
//
// You don't need to identify the questions to which anyone answered "yes"; you need to identify the questions to which everyone answered "yes"!
//
// Using the same example as above:
//
// abc
//
// a
// b
// c
//
// ab
// ac
//
// a
// a
// a
// a
//
// b
//
// This list represents answers from five groups:
//
//     In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
//     In the second group, there is no question to which everyone answered "yes".
//     In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
//     In the fourth group, everyone answered yes to only 1 question, a.
//     In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.
//
// In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.
//
// For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?

use std::str::FromStr;

#[allow(unused)]
const TEST_INPUT_A: &str = "abcx
abcy
abcz";

#[allow(unused)]
const TEST_INPUT_B: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

#[allow(unused)]
const INPUT: &str = include_str!("input");

fn char_diff(bigger: char, smaller: char) -> usize {
    return (u32::from(bigger) - u32::from(smaller)) as usize;
}

struct CustomDeclaration {
    // one could use a bitset or hashset here, but maybe we need it again for a later days question
    answers_seen: [u32; 26],
    people_on_form: u32,
}

impl CustomDeclaration {
    fn new() -> Self {
        Self {
            answers_seen: [0; 26],
            people_on_form: 0,
        }
    }

    fn part_1_score(&self) -> u32 {
        return self
            .answers_seen
            .iter()
            .map(|&c| if c > 0 { 1 } else { 0 })
            .sum();
    }

    fn part_2_score(&self) -> u32 {
        return self
            .answers_seen
            .iter()
            .map(|&c| if c == self.people_on_form { 1 } else { 0 })
            .sum();
    }
}

impl FromStr for CustomDeclaration {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut res = Self::new();

        for line in input.lines() {
            let line = line.trim();
            if line.len() <= 0 {
                continue;
            }

            res.people_on_form += 1;
            for c in line.chars() {
                if !('a'..='z').contains(&c) {
                    return Err(());
                }
                res.answers_seen[char_diff('z', c)] += 1;
            }
        }

        return Ok(res);
    }
}

fn split_declaration_forms_on_newlines(form_list: &str) -> impl Iterator<Item = String> {
    let mut form_strs: Vec<String> = Vec::new();
    let mut current_form = String::new();

    for line in form_list.lines() {
        let line = line.trim();
        if line.len() > 0 {
            current_form.push_str(line);
            current_form.push('\n');
        } else {
            current_form = current_form.trim().to_string();
            if current_form.len() > 0 {
                form_strs.push(current_form);
            }
            current_form = String::new();
        }
    }

    current_form = current_form.trim().to_string();
    if current_form.len() > 0 {
        form_strs.push(current_form);
    }

    return form_strs.into_iter();
}

fn part_1_declaration_forms(form_list: &str) -> impl Iterator<Item = u32> {
    return split_declaration_forms_on_newlines(form_list)
        .filter_map(|s| CustomDeclaration::from_str(s.as_str()).ok())
        .map(|c| c.part_1_score());
}

#[allow(unused)]
fn part_1(form_list: &str) -> u32 {
    return part_1_declaration_forms(form_list).sum();
}

fn part_2_declaration_forms(form_list: &str) -> impl Iterator<Item = u32> {
    return split_declaration_forms_on_newlines(form_list)
        .filter_map(|s| CustomDeclaration::from_str(s.as_str()).ok())
        .map(|c| c.part_2_score());
}

#[allow(unused)]
fn part_2(form_list: &str) -> u32 {
    return part_2_declaration_forms(form_list).sum();
}

#[test]
fn test_a() {
    let mut i = part_1_declaration_forms(TEST_INPUT_A);
    assert_eq!(i.next(), Some(6));
    assert_eq!(i.next(), None);
}

#[test]
fn test_b() {
    assert_eq!(part_1(TEST_INPUT_A), 6);
}

#[test]
fn test_c() {
    let mut i = part_1_declaration_forms(TEST_INPUT_B);
    assert_eq!(i.next(), Some(3));
    assert_eq!(i.next(), Some(3));
    assert_eq!(i.next(), Some(3));
    assert_eq!(i.next(), Some(1));
    assert_eq!(i.next(), Some(1));
    assert_eq!(i.next(), None);
}

#[test]
fn test_d() {
    assert_eq!(part_1(TEST_INPUT_B), 11);
}

#[test]
fn test_e() {
    let mut i = part_2_declaration_forms(TEST_INPUT_B);
    assert_eq!(i.next(), Some(3));
    assert_eq!(i.next(), Some(0));
    assert_eq!(i.next(), Some(1));
    assert_eq!(i.next(), Some(1));
    assert_eq!(i.next(), Some(1));
    assert_eq!(i.next(), None);
}

#[test]
fn test_f() {
    assert_eq!(part_2(TEST_INPUT_B), 6);
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
    println!("done.");
}
