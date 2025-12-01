#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_a() {
        assert_eq!(part_1(TEST_INPUT), Some(3));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_2(TEST_INPUT), Some(6));
    }

    #[test]
    fn test_c() {
        let mut dial_pos = 50;
        let zeroes = p2_right(&mut dial_pos, 20);
        assert_eq!(dial_pos, 70);
        assert_eq!(zeroes, 0);
    }
    #[test]
    fn test_d() {
        let mut dial_pos = 50;
        let zeroes = p2_right(&mut dial_pos, 50);
        assert_eq!(dial_pos, 0);
        assert_eq!(zeroes, 1);
    }
    #[test]
    fn test_e() {
        let mut dial_pos = 0;
        let zeroes = p2_right(&mut dial_pos, 50);
        assert_eq!(dial_pos, 50);
        assert_eq!(zeroes, 0);
    }
    #[test]
    fn test_f() {
        let mut dial_pos = 0;
        let zeroes = p2_right(&mut dial_pos, 100);
        assert_eq!(dial_pos, 0);
        assert_eq!(zeroes, 1);
    }
    #[test]
    fn test_g() {
        let mut dial_pos = 99;
        let zeroes = p2_right(&mut dial_pos, 1);
        assert_eq!(dial_pos, 0);
        assert_eq!(zeroes, 1);
    }
    #[test]
    fn test_h() {
        let mut dial_pos = 50;
        let zeroes = p2_right(&mut dial_pos, 49);
        assert_eq!(dial_pos, 99);
        assert_eq!(zeroes, 0);
    }
    #[test]
    fn test_i() {
        let mut dial_pos = 50;
        let zeroes = p2_right(&mut dial_pos, 51);
        assert_eq!(dial_pos, 1);
        assert_eq!(zeroes, 1);
    }
    #[test]
    fn test_j() {
        let mut dial_pos = 0;
        let zeroes = p2_right(&mut dial_pos, 99);
        assert_eq!(dial_pos, 99);
        assert_eq!(zeroes, 0);
    }
    #[test]
    fn test_k() {
        let mut dial_pos = 0;
        let zeroes = p2_right(&mut dial_pos, 99);
        assert_eq!(dial_pos, 99);
        assert_eq!(zeroes, 0);
    }
    #[test]
    fn test_l() {
        let mut dial_pos = 0;
        let zeroes = p2_right(&mut dial_pos, 100);
        assert_eq!(dial_pos, 0);
        assert_eq!(zeroes, 1);
    }
    #[test]
    fn test_m() {
        let mut dial_pos = 0;
        let zeroes = p2_right(&mut dial_pos, 123);
        assert_eq!(dial_pos, 23);
        assert_eq!(zeroes, 1);
    }
    #[test]
    fn test_n() {
        let mut dial_pos = 50;
        let zeroes = p2_right(&mut dial_pos, 149);
        assert_eq!(dial_pos, 99);
        assert_eq!(zeroes, 1);
    }
    #[test]
    fn test_o() {
        let mut dial_pos = 50;
        let zeroes = p2_right(&mut dial_pos, 150);
        assert_eq!(dial_pos, 0);
        assert_eq!(zeroes, 2);
    }
    #[test]
    fn test_p() {
        let mut dial_pos = 50;
        let zeroes = p2_right(&mut dial_pos, 151);
        assert_eq!(dial_pos, 1);
        assert_eq!(zeroes, 2);
    }
    #[test]
    fn test_q() {
        let mut dial_pos = 50;
        let zeroes = p2_right(&mut dial_pos, 249);
        assert_eq!(dial_pos, 99);
        assert_eq!(zeroes, 2);
    }
    #[test]
    fn test_r() {
        let mut dial_pos = 50;
        let zeroes = p2_right(&mut dial_pos, 250);
        assert_eq!(dial_pos, 0);
        assert_eq!(zeroes, 3);
    }
    #[test]
    fn test_s() {
        let mut dial_pos = 50;
        let zeroes = p2_right(&mut dial_pos, 251);
        assert_eq!(dial_pos, 1);
        assert_eq!(zeroes, 3);
    }

    #[test]
    fn test_la() {
        let mut dial_pos = 50;
        let zeroes = p2_left(&mut dial_pos, 49);
        assert_eq!(dial_pos, 1);
        assert_eq!(zeroes, 0);
    }
    #[test]
    fn test_lb() {
        let mut dial_pos = 50;
        let zeroes = p2_left(&mut dial_pos, 50);
        assert_eq!(dial_pos, 0);
        assert_eq!(zeroes, 1);
    }
    #[test]
    fn test_lc() {
        let mut dial_pos = 50;
        let zeroes = p2_left(&mut dial_pos, 51);
        assert_eq!(dial_pos, 99);
        assert_eq!(zeroes, 1);
    }
    #[test]
    fn test_ld() {
        let mut dial_pos = 0;
        let zeroes = p2_left(&mut dial_pos, 1);
        assert_eq!(dial_pos, 99);
        assert_eq!(zeroes, 0);
    }
    #[test]
    fn test_le() {
        let mut dial_pos = 1;
        let zeroes = p2_left(&mut dial_pos, 1);
        assert_eq!(dial_pos, 0);
        assert_eq!(zeroes, 1);
    }
    #[test]
    fn test_lf() {
        let mut dial_pos = 1;
        let zeroes = p2_left(&mut dial_pos, 2);
        assert_eq!(dial_pos, 99);
        assert_eq!(zeroes, 1);
    }
    #[test]
    fn test_lg() {
        let mut dial_pos = 50;
        let zeroes = p2_left(&mut dial_pos, 100);
        assert_eq!(dial_pos, 50);
        assert_eq!(zeroes, 1);
    }
    #[test]
    fn test_lh() {
        let mut dial_pos = 50;
        let zeroes = p2_left(&mut dial_pos, 149);
        assert_eq!(dial_pos, 1);
        assert_eq!(zeroes, 1);
    }
    #[test]
    fn test_lj() {
        let mut dial_pos = 50;
        let zeroes = p2_left(&mut dial_pos, 150);
        assert_eq!(dial_pos, 0);
        assert_eq!(zeroes, 2);
    }
    #[test]
    fn test_lk() {
        let mut dial_pos = 50;
        let zeroes = p2_left(&mut dial_pos, 151);
        assert_eq!(dial_pos, 99);
        assert_eq!(zeroes, 2);
    }
    #[test]
    fn test_ll() {
        let mut dial_pos = 50;
        let zeroes = p2_left(&mut dial_pos, 251);
        assert_eq!(dial_pos, 99);
        assert_eq!(zeroes, 3);
    }
}

const INPUT: &str = include_str!("../input.txt");

enum DialOp {
    L(i32),
    R(i32),
}

fn parse_1_2(input: &str) -> Option<Vec<DialOp>> {
    let mut res: Vec<DialOp> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let first = line.chars().nth(0)?;
        let rot: i32 = line[1..].parse().ok()?;

        if first == 'L' {
            res.push(DialOp::L(rot));
        } else if first == 'R' {
            res.push(DialOp::R(rot));
        } else {
            return None;
        }
    }

    return Some(res);
}

fn part_1(input: &str) -> Option<i32> {
    let mut res: i32 = 0;
    let mut dial_pos: i32 = 50;

    for op in parse_1_2(input)? {
        match op {
            DialOp::L(rot) => {
                dial_pos = dial_pos.wrapping_add(100 - (rot % 100)) % 100;
            }
            DialOp::R(rot) => {
                dial_pos = dial_pos.wrapping_add(rot % 100) % 100;
            }
        }
        if dial_pos == 0 {
            res += 1;
        }
    }
    return Some(res);
}

fn p2_right(dial_pos: &mut i32, r: i32) -> i32 {
    let mut d = *dial_pos;
    d += r;
    let zeropasses: i32 = d / 100;
    d = d % 100;
    *dial_pos = d;

    return zeropasses;
}

fn p2_left(dial_pos: &mut i32, l: i32) -> i32 {
    let mut d = *dial_pos;
    let mut l = l;

    let mut zeropasses: i32 = l / 100;
    l = l % 100;

    d = d - l;
    if d < 0 {
        if *dial_pos != 0 {
            zeropasses += 1;
        }
        d += 100;
    }

    if d == 0 {
        zeropasses += 1;
    }

    *dial_pos = d;
    return zeropasses;
}

fn part_2(input: &str) -> Option<i32> {
    let mut res: i32 = 0;
    let mut dial_pos: i32 = 50;

    for op in parse_1_2(input)? {
        match op {
            DialOp::L(rot) => {
                res += p2_left(&mut dial_pos, rot);
            }
            DialOp::R(rot) => {
                res += p2_right(&mut dial_pos, rot);
            }
        }
    }
    return Some(res);
}

fn main() {
    match part_1(INPUT) {
        Some(answer) => {
            println!("part 1: {0}", answer);
        }
        None => {
            println!("part 1 failed");
        }
    }
    match part_2(INPUT) {
        Some(answer) => {
            println!("part 2: {0}", answer);
        }
        None => {
            println!("part 2 failed");
        }
    }
    println!("Done.");
}
