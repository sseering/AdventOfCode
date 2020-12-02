// --- Day 2: Password Philosophy ---
//
// Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.
//
// The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.
//
// Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.
//
// To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.
//
// For example, suppose you have the following list:
//
// 1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc
//
// Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
//
// In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.
//
// How many passwords are valid according to their policies?
//
// To begin, get your puzzle input.
//
// The first half of this puzzle is complete! It provides one gold star: *
// --- Part Two ---
//
// While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.
//
// The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.
//
// Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
//
// Given the same example list from above:
//
//     1-3 a: abcde is valid: position 1 contains a and position 3 does not.
//     1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
//     2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
//
// How many passwords are valid according to the new interpretation of the policies?

#[allow(unused)]
const TEST_INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

#[allow(unused)]
const INPUT: &str = include_str!("input");

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), Some(2));
}

#[test]
fn test_b() {
    assert_eq!(part_1_more_iterators(TEST_INPUT), Some(2));
}

#[test]
fn test_c() {
    assert_eq!(part_1(TEST_INPUT), part_1_more_iterators(TEST_INPUT));
}

struct SplitLine {
    low: usize,
    high: usize,
    criteria: char,
    pass: String,
}

fn split_line(line: &str) -> Option<SplitLine> {
    let mut line_split = line.split_whitespace();
    let range = line_split.next()?;
    let criteria = line_split.next()?;
    let pass = line_split.next()?;

    let criteria = criteria.chars().next()?;
    let mut range = range.split("-");
    let low: usize = range.next()?.parse().ok()?;
    let high: usize = range.next()?.parse().ok()?;

    return Some(SplitLine {
        low: low,
        high: high,
        criteria: criteria,
        pass: pass.to_string(),
    });
}

#[allow(unused)]
fn part_1(password_list: &str) -> Option<usize> {
    // return None on input parse errors

    let mut num_good_passwords = 0;
    for password_line in password_list.lines() {
        let split = split_line(password_line)?;
        let num_matches = split.pass.chars().filter(|&c| c == split.criteria).count();

        if num_matches >= split.low && num_matches <= split.high {
            num_good_passwords += 1;
        }
    }

    return Some(num_good_passwords);
}

#[allow(unused)]
fn part_1_more_iterators(password_list: &str) -> Option<usize> {
    return Some(
        password_list
            .lines()
            .filter(|password_line| -> bool {
                let split = split_line(password_line).unwrap();

                let num_matches = split.pass.chars().filter(|&c| c == split.criteria).count();

                return num_matches >= split.low && num_matches <= split.high;
            })
            .count(),
    );
}

#[allow(unused)]
fn part_2(password_list: &str) -> usize {
    return password_list
        .lines()
        .filter(|password_line| -> bool {
            let split = split_line(password_line).unwrap();

            let chars: Vec<char> = split.pass.chars().collect();

            return (chars[split.low - 1] == split.criteria)
                != (chars[split.high - 1] == split.criteria);
        })
        .count();
}

#[test]
fn test_d() {
    assert_eq!(part_2(TEST_INPUT), 1);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p1 = part_1_more_iterators(INPUT).ok_or("whoops")?;
    let p2 = part_2(INPUT);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("done");

    return Ok(());
}
