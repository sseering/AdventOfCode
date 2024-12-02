#[allow(unused)]
const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

const INPUT: &str = include_str!("../input.txt");

fn parse_1_2(reports: &str) -> Option<Vec<Vec<u32>>> {
    let mut res: Vec<Vec<u32>> = Vec::new();
    for report in reports.lines() {
        let report_vec: Vec<u32> = report
            .split_whitespace()
            .map(|s| {
                return s.parse::<u32>().ok();
            })
            .collect::<Option<Vec<u32>>>()?;
        res.push(report_vec);
    }

    return Some(res);
}

fn check_report(report: &Vec<u32>) -> Option<(bool, bool)> {
    let mut levels = report.iter();
    let mut prev_level: u32 = *levels.next()?;
    let mut asc = true;
    let mut desc = true;
    for &level in levels {
        if level > prev_level {
            desc = false;
            let diff = prev_level.abs_diff(level);
            if diff > 3 {
                asc = false;
            }
        } else if level < prev_level {
            asc = false;
            let diff = prev_level.abs_diff(level);
            if diff > 3 {
                desc = false;
            }
        } else {
            asc = false;
            desc = false;
        }
        prev_level = level;
    }
    return Some((asc, desc));
}

fn part_1(reports: &str) -> Option<u32> {
    let reports = parse_1_2(reports)?;
    let mut res = 0;
    for report in reports {
        let (asc, desc) = check_report(&report)?;
        if asc || desc {
            res += 1;
        }
    }

    return Some(res);
}

fn part_2_check_report(
    report: &Vec<u32>,
    mut skip_idx: usize,
) -> Option<(bool, usize, bool, usize)> {
    let mut levels = report.iter();
    let mut prev_level: u32 = *levels.next()?;
    if skip_idx == 0 {
        prev_level = *levels.next()?;
        skip_idx = usize::MAX;
    }
    let mut asc = true;
    let mut desc = true;
    let mut asc_fail_idx: usize = usize::MAX;
    let mut desc_fail_idx: usize = usize::MAX;
    let mut set_asc_to_false = |idx: usize| {
        asc = false;
        if asc_fail_idx == usize::MAX {
            asc_fail_idx = idx - 1;
        }
    };
    let mut set_desc_to_false = |idx: usize| {
        desc = false;
        if desc_fail_idx == usize::MAX {
            desc_fail_idx = idx - 1;
        }
    };

    let mut idx = 1;
    for &level in levels {
        if idx == skip_idx {
            idx += 1;
            continue;
        }
        if level > prev_level {
            set_desc_to_false(idx);
            let diff = prev_level.abs_diff(level);
            if diff > 3 {
                set_asc_to_false(idx);
            }
        } else if level < prev_level {
            set_asc_to_false(idx);
            let diff = prev_level.abs_diff(level);
            if diff > 3 {
                set_desc_to_false(idx);
            }
        } else {
            set_asc_to_false(idx);
            set_desc_to_false(idx);
        }
        prev_level = level;
        idx += 1;
    }

    return Some((asc, asc_fail_idx, desc, desc_fail_idx));
}

fn part_2_simple(reports: &str) -> Option<u32> {
    let reports = parse_1_2(reports)?;
    let mut res = 0;
    for report in reports {
        let (asc, desc) = check_report(&report)?;
        if asc || desc {
            res += 1;
            continue;
        }
        for idx in 0..report.len() {
            let mut with_dampener = report.clone();
            with_dampener.remove(idx);
            let (asc, desc) = check_report(&with_dampener)?;
            if asc || desc {
                res += 1;
                break;
            }
        }
    }
    return Some(res);
}

fn part_2(reports: &str) -> Option<u32> {
    // this implementation is incorrect

    let reports = parse_1_2(reports)?;
    let mut res = 0;
    for report in reports {
        let (asc, asc_fail_idx, desc, desc_fail_idx) = part_2_check_report(&report, usize::MAX)?;
        if asc || desc {
            res += 1;
            continue;
        }
        let (asc, _, _, _) = part_2_check_report(&report, asc_fail_idx)?;
        if asc {
            res += 1;
            continue;
        }
        let (asc, _, _, _) = part_2_check_report(&report, asc_fail_idx + 1)?;
        if asc {
            res += 1;
            continue;
        }
        let (_, _, desc, _) = part_2_check_report(&report, desc_fail_idx)?;
        if desc {
            res += 1;
            continue;
        }
        let (_, _, desc, _) = part_2_check_report(&report, desc_fail_idx + 1)?;
        if desc {
            res += 1;
            continue;
        }
    }
    return Some(res);
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), Some(2));
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), Some(4));
}

#[test]
fn test_c() {
    assert_eq!(part_2_simple(TEST_INPUT), Some(4));
}

#[test]
fn test_d() {
    assert_eq!(part_2("46 50 49 51 52 54 57 58"), Some(1));
}

#[test]
fn test_e() {
    assert_eq!(part_2_simple("46 50 49 51 52 54 57 58"), Some(1));
}

#[test]
fn test_f() {
    assert_eq!(part_2("69 70 67 63 66"), Some(0));
}

#[test]
fn test_g() {
    assert_eq!(part_2_simple("69 70 67 63 66"), Some(0));
}

#[test]
fn test_h() {
    assert_eq!(part_2("21 23 19 17 16 13 10"), Some(1));
}

#[test]
fn test_i() {
    assert_eq!(part_2_simple("21 23 19 17 16 13 10"), Some(1));
}

#[test]
fn test_j() {
    assert_eq!(part_2("83 84 80 77 74 72 71"), Some(1));
}

#[test]
fn test_k() {
    assert_eq!(part_2_simple("83 84 80 77 74 72 71"), Some(1));
}

#[test]
fn test_l() {
    assert_eq!(part_2("84 82 83 84 85 88 90"), Some(1));
}

#[test]
fn test_m() {
    assert_eq!(part_2_simple("84 82 83 84 85 88 90"), Some(1));
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
    match part_2_simple(INPUT) {
        Some(cv) => {
            println!("Part 2 simple: {0}.", cv);
        }
        None => {
            println!("Part 2 simple failed.");
        }
    }
    println!("Done.");
}
