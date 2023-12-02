const INPUT: &str = include_str!("../input.txt");

fn calibration_value_p1(trebuchet_adjustment_line: &str) -> Option<i32> {
    let mut first_digit: i32 = -1;
    let mut last_digit: i32 = -1;

    let mut found_digit = |digit: i32| {
        if first_digit == -1 {
            first_digit = digit;
        }
        last_digit = digit;
    };
    for char in trebuchet_adjustment_line.chars() {
        match char {
            '0' => found_digit(0),
            '1' => found_digit(1),
            '2' => found_digit(2),
            '3' => found_digit(3),
            '4' => found_digit(4),
            '5' => found_digit(5),
            '6' => found_digit(6),
            '7' => found_digit(7),
            '8' => found_digit(8),
            '9' => found_digit(9),
            _ => { /* nothing */ }
        }
    }

    if first_digit == -1 || last_digit == -1 {
        return None;
    }

    return Some(10 * first_digit + last_digit);
}

struct CalibrationValues {
    first_digit: i32,
    last_digit: i32,
}

impl CalibrationValues {
    fn new() -> Self {
        Self {
            first_digit: -1,
            last_digit: -1,
        }
    }

    fn get(&self) -> Option<i32> {
        if self.first_digit == -1 || self.last_digit == -1 {
            return None;
        }

        return Some(10 * self.first_digit + self.last_digit);
    }

    fn found_digit(&mut self, d: i32) {
        if self.first_digit == -1 {
            self.first_digit = d;
        }
        self.last_digit = d;
    }
}

fn calibration_value_p2(trebuchet_adjustment_line: &str) -> Option<i32> {
    const O: u8 = 111;
    const N: u8 = 110;
    const E: u8 = 101;
    const T: u8 = 116;
    const W: u8 = 119;
    const H: u8 = 104;
    const R: u8 = 114;
    const F: u8 = 102;
    const U: u8 = 117;
    const I: u8 = 105;
    const V: u8 = 118;
    const S: u8 = 115;
    const X: u8 = 120;
    const G: u8 = 103;

    let mut res = CalibrationValues::new();
    let mut idx: usize = 0;
    let line = trebuchet_adjustment_line.as_bytes();
    let len = line.len();

    fn handle_maybe_number_word(
        needle: &[u8],
        value: i32,
        haystack: &[u8],
        idx: usize,
        res: &mut CalibrationValues,
    ) {
        let end = idx + needle.len();
        if end > haystack.len() {
            return;
        }
        if &haystack[idx..end] == needle {
            res.found_digit(value);
        }
    }

    while idx < len {
        match char::from(line[idx]) {
            '0' => {
                res.found_digit(0);
            }
            '1' => {
                res.found_digit(1);
            }
            '2' => {
                res.found_digit(2);
            }
            '3' => {
                res.found_digit(3);
            }
            '4' => {
                res.found_digit(4);
            }
            '5' => {
                res.found_digit(5);
            }
            '6' => {
                res.found_digit(6);
            }
            '7' => {
                res.found_digit(7);
            }
            '8' => {
                res.found_digit(8);
            }
            '9' => {
                res.found_digit(9);
            }
            'o' => {
                handle_maybe_number_word(&[O, N, E], 1, line, idx, &mut res);
            }
            't' => {
                handle_maybe_number_word(&[T, W, O], 2, line, idx, &mut res);
                handle_maybe_number_word(&[T, H, R, E, E], 3, line, idx, &mut res);
            }
            'f' => {
                handle_maybe_number_word(&[F, O, U, R], 4, line, idx, &mut res);
                handle_maybe_number_word(&[F, I, V, E], 5, line, idx, &mut res);
            }
            's' => {
                handle_maybe_number_word(&[S, I, X], 6, line, idx, &mut res);
                handle_maybe_number_word(&[S, E, V, E, N], 7, line, idx, &mut res);
            }
            'e' => {
                handle_maybe_number_word(&[E, I, G, H, T], 8, line, idx, &mut res);
            }
            'n' => {
                handle_maybe_number_word(&[N, I, N, E], 9, line, idx, &mut res);
            }
            _ => { /* nothing */ }
        }
        idx += 1;
    }

    return res.get();
}

fn part_1(trebuchet_adjustment_instructions: &str) -> Option<i32> {
    let mut res: i32 = 0;
    for line in trebuchet_adjustment_instructions.lines() {
        res += calibration_value_p1(line)?;
    }
    return Some(res);
}

fn part_2(trebuchet_adjustment_instructions: &str) -> Option<i32> {
    let mut res: i32 = 0;

    for line in trebuchet_adjustment_instructions.lines() {
        let v = calibration_value_p2(line)?;
        res += v;
    }
    return Some(res);
}

#[test]
fn test_aa() {
    assert_eq!(calibration_value_p1("1abc2"), Some(12));
}

#[test]
fn test_ab() {
    assert_eq!(calibration_value_p1("pqr3stu8vwx"), Some(38));
}

#[test]
fn test_ac() {
    assert_eq!(calibration_value_p1("a1b2c3d4e5f"), Some(15));
}

#[test]
fn test_ad() {
    assert_eq!(calibration_value_p1("treb7uchet"), Some(77));
}

#[test]
fn test_ae() {
    assert_eq!(
        part_1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"),
        Some(142)
    );
}

#[test]
fn test_af() {
    assert_eq!(calibration_value_p2("1abc2"), Some(12));
}

#[test]
fn test_ag() {
    assert_eq!(calibration_value_p2("pqr3stu8vwx"), Some(38));
}

#[test]
fn test_ah() {
    assert_eq!(calibration_value_p2("a1b2c3d4e5f"), Some(15));
}

#[test]
fn test_ai() {
    assert_eq!(calibration_value_p2("treb7uchet"), Some(77));
}

#[test]
fn test_aj() {
    assert_eq!(
        part_1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"),
        Some(142)
    );
}

#[test]
fn test_ak() {
    assert_eq!(calibration_value_p2("two1nine"), Some(29));
}

#[test]
fn test_al() {
    assert_eq!(calibration_value_p2("eightwothree"), Some(83));
}

#[test]
fn test_am() {
    assert_eq!(calibration_value_p2("abcone2threexyz"), Some(13));
}

#[test]
fn test_an() {
    assert_eq!(calibration_value_p2("xtwone3four"), Some(24));
}

#[test]
fn test_ao() {
    assert_eq!(calibration_value_p2("4nineeightseven2"), Some(42));
}

#[test]
fn test_ap() {
    assert_eq!(calibration_value_p2("zoneight234"), Some(14));
}

#[test]
fn test_aq() {
    assert_eq!(calibration_value_p2("7pqrstsixteen"), Some(76));
}

#[test]
fn test_ar() {
    assert_eq!(calibration_value_p2("xxtwonexx"), Some(21));
}

#[test]
fn test_as() {
    assert_eq!(
        part_2("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"),
        Some(281)
    );
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

    println!("Done.");
}
