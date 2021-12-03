#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

fn part_1(diagnostics_report: &str) -> i32 {
    let mut lines = diagnostics_report.lines().peekable();
    let num_didgts = lines.peek().unwrap().len();
    let mut ones: Vec<i32> = vec![0; num_didgts];
    let mut zeroes: Vec<i32> = vec![0; num_didgts];
    for line in lines {
        for (idx, char) in line.chars().enumerate() {
            match char {
                '0' => {
                    zeroes[idx] += 1;
                }
                '1' => {
                    ones[idx] += 1;
                }
                _ => unimplemented!(),
            }
        }
    }

    // I don´t use from_str_radix() or similar here on purpose because I want to implement the Horner scheme.
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for idx in 0..num_didgts {
        if ones[idx] > zeroes[idx] {
            gamma_rate = gamma_rate * 2 + 1;
            epsilon_rate = epsilon_rate * 2;
        } else if ones[idx] < zeroes[idx] {
            gamma_rate = gamma_rate * 2;
            epsilon_rate = epsilon_rate * 2 + 1;
        } else {
            unimplemented!()
        }
    }

    let power_consumption = gamma_rate * epsilon_rate;

    return power_consumption;
}

fn find_rating(diagnostics_report: &Vec<&str>, for_oxygen_generator: bool) -> i32 {
    return _find_rating(diagnostics_report, for_oxygen_generator, 0);
}

fn _find_rating(diagnostics_report: &Vec<&str>, for_oxygen_generator: bool, idx: usize) -> i32 {
    let mut ones = 0;
    let mut zeroes = 0;
    for line in diagnostics_report {
        match line.chars().nth(idx).unwrap() {
            '0' => {
                zeroes += 1;
            }
            '1' => {
                ones += 1;
            }
            _ => unimplemented!(),
        }
    }

    let criteria = if for_oxygen_generator {
        if ones >= zeroes {
            '1'
        } else {
            '0'
        }
    } else {
        if zeroes <= ones {
            '0'
        } else {
            '1'
        }
    };
    let selected = diagnostics_report
        .iter()
        .filter(|&v| v.chars().nth(idx).unwrap() == criteria)
        .map(|&s| s.clone())
        .collect::<Vec<&str>>();

    if selected.len() == 0 {
        panic!()
    } else if selected.len() == 1 {
        return i32::from_str_radix(selected[0], 2).unwrap();
    } else {
        return _find_rating(&selected, for_oxygen_generator, idx + 1);
    }
}

fn part_2(diagnostics_report: &str) -> i32 {
    let diagnostics_report = diagnostics_report.lines().collect::<Vec<&str>>();

    let oxygen_generator_rating = find_rating(&diagnostics_report, true);
    let co2_scrubber_rating = find_rating(&diagnostics_report, false);

    let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;

    return life_support_rating;
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), 198);
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), 230);
}

fn main() {
    println!("part 1: {0}", part_1(INPUT));
    println!("part 2: {0}", part_2(INPUT));
    println!("done");
}
