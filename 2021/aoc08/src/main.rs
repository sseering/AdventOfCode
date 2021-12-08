use std::collections::HashSet;

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

fn parse(signal_patterns: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    let mut result = Vec::new();
    for line in signal_patterns.lines() {
        if let [unique_signal_patterns, four_digit_output_value] =
            line.split('|').collect::<Vec<&str>>()[..]
        {
            result.push((
                unique_signal_patterns
                    .split_whitespace()
                    .collect::<Vec<&str>>(),
                four_digit_output_value
                    .split_whitespace()
                    .collect::<Vec<&str>>(),
            ));
        } else {
            panic!();
        }
    }
    return result;
}

fn part_1(signal_patterns: &str) -> u32 {
    let mut num_easy_sigits = 0;
    let lines = parse(signal_patterns);
    for line in lines {
        for four_digit_output_value in line.1 {
            let l = four_digit_output_value.len();
            if l == 2 || l == 3 || l == 4 || l == 7 {
                num_easy_sigits += 1;
            }
        }
    }
    return num_easy_sigits;
}

fn part_2(signal_patterns: &str) -> u32 {
    let mut sum_of_decoded = 0;
    let lines = parse(signal_patterns);
    for line in lines {
        let mut segments_in_digit_1: HashSet<char> = HashSet::new();
        let mut segments_in_digit_4: HashSet<char> = HashSet::new();
        let mut segments_in_digit_7: HashSet<char> = HashSet::new();
        let mut segments_in_digit_8: HashSet<char> = HashSet::new();

        for unique_signal_pattern in &line.0 {
            let l = unique_signal_pattern.len();
            if l == 2 {
                for c in unique_signal_pattern.chars() {
                    segments_in_digit_1.insert(c);
                }
            } else if l == 3 {
                for c in unique_signal_pattern.chars() {
                    segments_in_digit_7.insert(c);
                }
            }
            if l == 4 {
                for c in unique_signal_pattern.chars() {
                    segments_in_digit_4.insert(c);
                }
            }
            if l == 7 {
                for c in unique_signal_pattern.chars() {
                    segments_in_digit_8.insert(c);
                }
            }
        }

        if segments_in_digit_1.len() == 0
            || segments_in_digit_4.len() == 0
            || segments_in_digit_7.len() == 0
            || segments_in_digit_8.len() == 0
        {
            panic!("not enough information");
        }

        let top_segement_char: char = *segments_in_digit_1
            .symmetric_difference(&segments_in_digit_7)
            .next()
            .unwrap();

        let mut top_right_segment_char = ' ';
        'top_right_search: for unique_signal_pattern in &line.0 {
            // digit 6 or 0 or 9
            if unique_signal_pattern.len() == 6 {
                for &c in &segments_in_digit_1 {
                    if !unique_signal_pattern.contains(c) {
                        top_right_segment_char = c;
                        break 'top_right_search;
                    }
                }
            }
        }
        if top_right_segment_char == ' ' {
            panic!("not enough information tr");
        }

        let mut bottom_right_segment_char = ' ';
        for &c in &segments_in_digit_1 {
            if c != top_right_segment_char {
                bottom_right_segment_char = c;
                break;
            }
        }
        if bottom_right_segment_char == ' ' {
            panic!("not enough information br");
        }

        let mut middle_segment_char: char = ' ';
        let in_4_not_in_7: HashSet<char> = segments_in_digit_7
            .symmetric_difference(&segments_in_digit_4)
            .map(|&c| c)
            .collect();
        'middle_search: for unique_signal_pattern in &line.0 {
            // digit 2 or 3 or 5
            if unique_signal_pattern.len() == 5 {
                let mut not_in_7: HashSet<char> = HashSet::new();
                for c in unique_signal_pattern.chars() {
                    if !segments_in_digit_7.contains(&c) {
                        not_in_7.insert(c);
                    }
                }
                // is unique_signal_pattern the digit 3?
                if not_in_7.len() == 2 {
                    middle_segment_char = *not_in_7.intersection(&in_4_not_in_7).next().unwrap();
                    break 'middle_search;
                }
            }
        }
        if middle_segment_char == ' ' {
            panic!("not enough information m");
        }

        let top_left_segment_char: char = *segments_in_digit_4
            .iter()
            .filter(|&&c| {
                c != top_segement_char
                    && c != top_right_segment_char
                    && c != bottom_right_segment_char
                    && c != middle_segment_char
            })
            .next()
            .unwrap();

        let mut bottom_segment_char: char = ' ';
        'bottom_left_search: for unique_signal_pattern in &line.0 {
            // digit 0 or 5 or 9
            if unique_signal_pattern.len() == 6 {
                let yet_unknow: Vec<char> = unique_signal_pattern
                    .chars()
                    .filter(|&c| {
                        c != top_segement_char
                            && c != top_right_segment_char
                            && c != bottom_right_segment_char
                            && c != middle_segment_char
                            && c != top_left_segment_char
                    })
                    .collect();
                if yet_unknow.len() == 1 {
                    bottom_segment_char = yet_unknow[0];
                    break 'bottom_left_search;
                }
            }
        }
        if bottom_segment_char == ' ' {
            panic!("not enough information b");
        }

        // Verify we have no duplications till now.
        let should_be_6: HashSet<char> = HashSet::from([
            top_segement_char,
            top_right_segment_char,
            bottom_right_segment_char,
            middle_segment_char,
            top_left_segment_char,
            bottom_segment_char,
        ]);
        if should_be_6.len() != 6 {
            panic!("not 6");
        }

        let mut bottom_left_segment_char: char = ' ';
        for c in ['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
            if !should_be_6.contains(&c) {
                bottom_left_segment_char = c;
                break;
            }
        }
        if bottom_left_segment_char == ' ' {
            panic!("not enough information bl");
        }

        let segments_in_digit_0: HashSet<char> = HashSet::from([
            top_segement_char,
            top_right_segment_char,
            bottom_right_segment_char,
            top_left_segment_char,
            bottom_left_segment_char,
            bottom_segment_char,
        ]);
        let segments_in_digit_2: HashSet<char> = HashSet::from([
            top_segement_char,
            top_right_segment_char,
            middle_segment_char,
            bottom_left_segment_char,
            bottom_segment_char,
        ]);
        let segments_in_digit_3: HashSet<char> = HashSet::from([
            top_segement_char,
            top_right_segment_char,
            bottom_right_segment_char,
            middle_segment_char,
            bottom_segment_char,
        ]);
        let segments_in_digit_5: HashSet<char> = HashSet::from([
            top_segement_char,
            bottom_right_segment_char,
            middle_segment_char,
            top_left_segment_char,
            bottom_segment_char,
        ]);
        let segments_in_digit_6: HashSet<char> = HashSet::from([
            top_segement_char,
            bottom_right_segment_char,
            middle_segment_char,
            top_left_segment_char,
            bottom_left_segment_char,
            bottom_segment_char,
        ]);
        let segments_in_digit_9: HashSet<char> = HashSet::from([
            top_segement_char,
            top_right_segment_char,
            bottom_right_segment_char,
            middle_segment_char,
            top_left_segment_char,
            bottom_segment_char,
        ]);

        let str2digit = |s: &str| -> u32 {
            let l = s.len();
            if l == 2 {
                return 1;
            }
            if l == 3 {
                return 7;
            }
            if l == 4 {
                return 4;
            }
            if l == 7 {
                return 8;
            }

            let as_set: HashSet<char> = HashSet::from_iter(s.chars());
            if as_set == segments_in_digit_0 {
                return 0;
            }
            if as_set == segments_in_digit_2 {
                return 2;
            }
            if as_set == segments_in_digit_3 {
                return 3;
            }
            if as_set == segments_in_digit_5 {
                return 5;
            }
            if as_set == segments_in_digit_6 {
                return 6;
            }
            if as_set == segments_in_digit_9 {
                return 9;
            }
            panic!("{}", s);
        };

        let four_digit_output_values = line.1;
        let decoded: Vec<u32> = four_digit_output_values
            .iter()
            .map(|&s| str2digit(s))
            .collect();

        sum_of_decoded += 1000 * decoded[0] + 100 * decoded[1] + 10 * decoded[2] + decoded[3];
    }
    return sum_of_decoded;
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), 26);
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), 61229);
}

fn main() {
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
    println!("Done.");
}
