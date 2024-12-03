use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn part_1(computer_memory: &str) -> Option<u32> {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").ok()?;
    let mut res = 0;

    for cap in mul_re.captures_iter(computer_memory) {
        let l: u32 = cap.get(1)?.as_str().parse().ok()?;
        let r: u32 = cap.get(2)?.as_str().parse().ok()?;

        res += l * r;
    }
    return Some(res);
}

fn part_2(computer_memory: &str) -> Option<u32> {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").ok()?;
    let mut res = 0;

    let mut do_mul = true;
    for cap in mul_re.captures_iter(computer_memory) {
        let cmd = cap.get(0)?.as_str();
        if cmd == "do()" {
            do_mul = true;
        } else if cmd == "don't()" {
            do_mul = false;
        } else {
            if do_mul {
                let l: u32 = cap.get(1)?.as_str().parse().ok()?;
                let r: u32 = cap.get(2)?.as_str().parse().ok()?;

                res += l * r;
            }
        }
    }
    return Some(res);
}

#[test]
fn test_a() {
    assert_eq!(part_1("mul(44,46)"), Some(2024));
}

#[test]
fn test_b() {
    assert_eq!(part_1("mul(123,4)"), Some(123 * 4));
}

#[test]
fn test_c() {
    assert_eq!(part_1("mul(4*"), Some(0));
}

#[test]
fn test_d() {
    assert_eq!(part_1("mul(6,9!"), Some(0));
}

#[test]
fn test_e() {
    assert_eq!(part_1("?(12,34)"), Some(0));
}

#[test]
fn test_f() {
    assert_eq!(part_1("mul ( 2 , 4 )"), Some(0));
}

#[test]
fn test_g() {
    assert_eq!(
        part_1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
        Some(161)
    );
}

#[test]
fn test_h() {
    assert_eq!(
        part_2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
        Some(48)
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
