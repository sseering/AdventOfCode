#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_a() {
        assert_eq!(part_1_simple(TEST_INPUT), Some(1227775554));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_1_better(TEST_INPUT), Some(1227775554));
    }

    #[test]
    fn test_c() {
        assert_eq!(part_2(TEST_INPUT), Some(4174379265));
    }

    #[test]
    fn test_aa() {
        assert_eq!(part_1_is_invalid_id(55, 2), true);
    }
    #[test]
    fn test_ab() {
        assert_eq!(part_1_is_invalid_id(6464, 4), true);
    }
    #[test]
    fn test_ac() {
        assert_eq!(part_1_is_invalid_id(123123, 6), true);
    }
    #[test]
    fn test_ad() {
        assert_eq!(part_1_is_invalid_id(101, 3), false);
    }
    #[test]
    fn test_ae() {
        assert_eq!(part_1_is_invalid_id(11, 2), true);
    }
    #[test]
    fn test_af() {
        assert_eq!(part_1_is_invalid_id(22, 2), true);
    }
    #[test]
    fn test_ag() {
        assert_eq!(part_1_is_invalid_id(95, 2), false);
    }
    #[test]
    fn test_ah() {
        assert_eq!(part_1_is_invalid_id(99, 2), true);
    }
    #[test]
    fn test_ai() {
        assert_eq!(part_1_is_invalid_id(115, 3), false);
    }
    #[test]
    fn test_aj() {
        assert_eq!(part_1_is_invalid_id(998, 3), false);
    }
    #[test]
    fn test_ak() {
        assert_eq!(part_1_is_invalid_id(1010, 4), true);
    }
    #[test]
    fn test_al() {
        assert_eq!(part_1_is_invalid_id(1012, 4), false);
    }
    #[test]
    fn test_am() {
        assert_eq!(part_1_is_invalid_id(1188511880, 10), false);
    }
    #[test]
    fn test_an() {
        assert_eq!(part_1_is_invalid_id(1188511885, 10), true);
    }
    #[test]
    fn test_ao() {
        assert_eq!(part_1_is_invalid_id(1188511890, 10), false);
    }
    #[test]
    fn test_ap() {
        assert_eq!(part_1_is_invalid_id(222220, 6), false);
    }
    #[test]
    fn test_aq() {
        assert_eq!(part_1_is_invalid_id(222222, 6), true);
    }
    #[test]
    fn test_ar() {
        assert_eq!(part_1_is_invalid_id(222224, 6), false);
    }
    #[test]
    fn test_as() {
        assert_eq!(part_1_is_invalid_id(1698522, 7), false);
    }
    #[test]
    fn test_at() {
        assert_eq!(part_1_is_invalid_id(1698528, 7), false);
    }
    #[test]
    fn test_au() {
        assert_eq!(part_1_is_invalid_id(446443, 6), false);
    }
    #[test]
    fn test_av() {
        assert_eq!(part_1_is_invalid_id(446446, 6), true);
    }
    #[test]
    fn test_aw() {
        assert_eq!(part_1_is_invalid_id(446449, 6), false);
    }
    #[test]
    fn test_ax() {
        assert_eq!(part_1_is_invalid_id(38593856, 8), false);
    }
    #[test]
    fn test_ay() {
        assert_eq!(part_1_is_invalid_id(38593859, 8), true);
    }
    #[test]
    fn test_az() {
        assert_eq!(part_1_is_invalid_id(38593862, 8), false);
    }

    #[test]
    fn test_ba() {
        assert_eq!(next_bigger_num_of_even_digit_length(1), 11);
    }
    #[test]
    fn test_bb() {
        assert_eq!(next_bigger_num_of_even_digit_length(1), 11);
    }
    #[test]
    fn test_bc() {
        assert_eq!(next_bigger_num_of_even_digit_length(1), 11);
    }
    #[test]
    fn test_bd() {
        assert_eq!(next_bigger_num_of_even_digit_length(1), 11);
    }
    #[test]
    fn test_be() {
        assert_eq!(next_bigger_num_of_even_digit_length(3), 1010);
    }
    #[test]
    fn test_bf() {
        assert_eq!(next_bigger_num_of_even_digit_length(3), 1010);
    }
    #[test]
    fn test_bg() {
        assert_eq!(next_bigger_num_of_even_digit_length(3), 1010);
    }
    #[test]
    fn test_bh() {
        assert_eq!(next_bigger_num_of_even_digit_length(3), 1010);
    }
    #[test]
    fn test_bi() {
        assert_eq!(next_bigger_num_of_even_digit_length(3), 1010);
    }
    #[test]
    fn test_bj() {
        assert_eq!(next_bigger_num_of_even_digit_length(3), 1010);
    }
    #[test]
    fn test_bk() {
        assert_eq!(next_bigger_num_of_even_digit_length(3), 1010);
    }
    #[test]
    fn test_bl() {
        assert_eq!(next_bigger_num_of_even_digit_length(3), 1010);
    }
    #[test]
    fn test_bm() {
        assert_eq!(next_bigger_num_of_even_digit_length(3), 1010);
    }
    #[test]
    fn test_bn() {
        assert_eq!(next_bigger_num_of_even_digit_length(3), 1010);
    }
    #[test]
    fn test_bo() {
        assert_eq!(next_bigger_num_of_even_digit_length(3), 1010);
    }
    #[test]
    fn test_bp() {
        assert_eq!(next_bigger_num_of_even_digit_length(3), 1010);
    }
    #[test]
    fn test_bq() {
        assert_eq!(next_bigger_num_of_even_digit_length(5), 100100);
    }
    #[test]
    fn test_br() {
        assert_eq!(next_bigger_num_of_even_digit_length(5), 100100);
    }
    #[test]
    fn test_bs() {
        assert_eq!(next_bigger_num_of_even_digit_length(5), 100100);
    }
    #[test]
    fn test_bt() {
        assert_eq!(next_bigger_num_of_even_digit_length(5), 100100);
    }
    #[test]
    fn test_bu() {
        assert_eq!(next_bigger_num_of_even_digit_length(5), 100100);
    }
    #[test]
    fn test_bv() {
        assert_eq!(next_bigger_num_of_even_digit_length(5), 100100);
    }

    #[test]
    fn test_ca() {
        assert_eq!(part_2_is_invalid_id(12341234, 8), true);
    }
    #[test]
    fn test_cb() {
        assert_eq!(part_2_is_invalid_id(123123123, 9), true);
    }
    #[test]
    fn test_cd() {
        assert_eq!(part_2_is_invalid_id(1212121212, 10), true);
    }
    #[test]
    fn test_ce() {
        assert_eq!(part_2_is_invalid_id(1111111, 7), true);
    }
    #[test]
    fn test_cf() {
        assert_eq!(part_2_is_invalid_id(11, 2), true);
    }
    #[test]
    fn test_cg() {
        assert_eq!(part_2_is_invalid_id(22, 2), true);
    }
    #[test]
    fn test_ch() {
        assert_eq!(part_2_is_invalid_id(95, 2), false);
    }
    #[test]
    fn test_ci() {
        assert_eq!(part_2_is_invalid_id(99, 2), true);
    }
    #[test]
    fn test_cj() {
        assert_eq!(part_2_is_invalid_id(111, 3), true);
    }
    #[test]
    fn test_ck() {
        assert_eq!(part_2_is_invalid_id(115, 3), false);
    }
    #[test]
    fn test_cl() {
        assert_eq!(part_2_is_invalid_id(998, 3), false);
    }
    #[test]
    fn test_cm() {
        assert_eq!(part_2_is_invalid_id(999, 3), true);
    }
    #[test]
    fn test_cn() {
        assert_eq!(part_2_is_invalid_id(1010, 4), true);
    }
    #[test]
    fn test_co() {
        assert_eq!(part_2_is_invalid_id(1012, 4), false);
    }
    #[test]
    fn test_cp() {
        assert_eq!(part_2_is_invalid_id(1188511880, 10), false);
    }
    #[test]
    fn test_cq() {
        assert_eq!(part_2_is_invalid_id(1188511885, 10), true);
    }
    #[test]
    fn test_cr() {
        assert_eq!(part_2_is_invalid_id(1188511890, 10), false);
    }
    #[test]
    fn test_cs() {
        assert_eq!(part_2_is_invalid_id(222220, 6), false);
    }
    #[test]
    fn test_ct() {
        assert_eq!(part_2_is_invalid_id(222222, 6), true);
    }
    #[test]
    fn test_cu() {
        assert_eq!(part_2_is_invalid_id(222224, 6), false);
    }
    #[test]
    fn test_cv() {
        assert_eq!(part_2_is_invalid_id(1698522, 7), false);
    }
    #[test]
    fn test_cw() {
        assert_eq!(part_2_is_invalid_id(1698528, 7), false);
    }
    #[test]
    fn test_cx() {
        assert_eq!(part_2_is_invalid_id(446443, 6), false);
    }
    #[test]
    fn test_cy() {
        assert_eq!(part_2_is_invalid_id(446446, 6), true);
    }
    #[test]
    fn test_cz() {
        assert_eq!(part_2_is_invalid_id(446449, 6), false);
    }
    #[test]
    fn test_da() {
        assert_eq!(part_2_is_invalid_id(38593856, 8), false);
    }
    #[test]
    fn test_db() {
        assert_eq!(part_2_is_invalid_id(38593859, 8), true);
    }
    #[test]
    fn test_dc() {
        assert_eq!(part_2_is_invalid_id(38593862, 8), false);
    }
    #[test]
    fn test_dd() {
        assert_eq!(part_2_is_invalid_id(565653, 6), false);
    }
    #[test]
    fn test_de() {
        assert_eq!(part_2_is_invalid_id(565656, 6), true);
    }
    #[test]
    fn test_df() {
        assert_eq!(part_2_is_invalid_id(565659, 6), false);
    }
    #[test]
    fn test_dg() {
        assert_eq!(part_2_is_invalid_id(824824821, 9), false);
    }
    #[test]
    fn test_dh() {
        assert_eq!(part_2_is_invalid_id(824824824, 9), true);
    }
    #[test]
    fn test_di() {
        assert_eq!(part_2_is_invalid_id(824824827, 9), false);
    }
    #[test]
    fn test_dj() {
        assert_eq!(part_2_is_invalid_id(2121212118, 10), false);
    }
    #[test]
    fn test_dk() {
        assert_eq!(part_2_is_invalid_id(2121212121, 10), true);
    }
    #[test]
    fn test_dl() {
        assert_eq!(part_2_is_invalid_id(2121212124, 10), false);
    }
}

const INPUT: &str = include_str!("../input.txt");

fn part_1_is_invalid_id(n: u64, num_digits: u32) -> bool {
    if num_digits < 1 || num_digits % 2 != 0 {
        return false;
    }

    let nd2 = num_digits / 2;

    let mut d_right: u64 = 1;
    let mut d_left: u64 = 10_u64.pow(nd2);

    for _ in 0..nd2 {
        let r = (n / d_right) % 10;
        let l = (n / d_left) % 10;

        if l != r {
            return false;
        }

        d_right *= 10;
        d_left *= 10;
    }

    return true;
}

fn part_2_is_invalid_id(n: u64, num_digits: u32) -> bool {
    if n < 10 {
        return false;
    }
    let nd2 = num_digits / 2;

    for check_len in 1..=nd2 {
        if num_digits % check_len != 0 {
            continue;
        }

        let modulo = 10_u64.pow(check_len);
        let lowest = n % modulo;

        let num_checks = (num_digits / check_len) - 1;
        let mut to_check = n;
        let mut all_matching = true;

        for _ in 0..num_checks {
            to_check = to_check / modulo;
            let cmp_val = to_check % modulo;
            if cmp_val != lowest {
                all_matching = false;
            }
        }

        if all_matching {
            return true;
        }
    }

    return false;
}

fn parse_1_2(input: &str) -> Option<Vec<(u64, u64)>> {
    let mut res: Vec<(u64, u64)> = Vec::new();

    for pair_str in input.trim().split(',') {
        let num_strs: Vec<&str> = pair_str.split("-").collect();
        if num_strs.len() != 2 {
            return None;
        }
        let start: u64 = num_strs.get(0)?.parse().ok()?;
        let end: u64 = num_strs.get(1)?.parse().ok()?;

        res.push((start, end));
    }

    return Some(res);
}

fn part_1_simple(input: &str) -> Option<u64> {
    // approximately 6 times slower than part_1_better()
    let mut res = 0;

    for (start, end) in parse_1_2(input)? {
        let mut val = start;
        while val <= end {
            let len = val.ilog10() + 1;
            if part_1_is_invalid_id(val, len) {
                res += val;
            }
            val += 1;
        }
    }

    return Some(res);
}

fn part_1_better(input: &str) -> Option<u64> {
    let mut res = 0;

    for (start, end) in parse_1_2(input)? {
        let mut val = start;
        while val <= end {
            let len = val.ilog10() + 1;
            if len % 2 != 0 {
                val = next_bigger_num_of_even_digit_length(len);
                continue;
            }

            if part_1_is_invalid_id(val, len) {
                res += val;
                let offset = 1 + 10_u64.pow(len / 2);
                val += offset;
            } else {
                val += 1;
            }
        }
    }

    return Some(res);
}

fn next_bigger_num_of_even_digit_length(num_digits: u32) -> u64 {
    let nd2 = num_digits / 2;
    let bigger = 10_u64.pow(num_digits) + 10_u64.pow(nd2);
    return bigger;
}

fn part_2(input: &str) -> Option<u64> {
    let mut res = 0;

    for (start, end) in parse_1_2(input)? {
        let mut val = start;
        while val <= end {
            let len = val.ilog10() + 1;
            if part_2_is_invalid_id(val, len) {
                res += val;
            }
            val += 1;
        }
    }

    return Some(res);
}

fn main() {
    match part_1_simple(INPUT) {
        Some(answer) => {
            println!("part 1 simple: {0}", answer);
        }
        None => {
            println!("part 1 simple failed");
        }
    }
    match part_1_better(INPUT) {
        Some(answer) => {
            println!("part 1 better: {0}", answer);
        }
        None => {
            println!("part 1 better failed");
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
