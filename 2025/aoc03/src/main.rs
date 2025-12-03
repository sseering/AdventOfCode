#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_a() {
        assert_eq!(part_1_simple(TEST_INPUT), Some(357));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_1_better(TEST_INPUT), Some(357));
    }

    #[test]
    fn test_c() {
        assert_eq!(part_2(TEST_INPUT), Some(3121910778619));
    }
}

const INPUT: &str = include_str!("../input.txt");

fn prarse_1_2(input: &str) -> Option<Vec<Vec<u8>>> {
    let mut res: Vec<Vec<u8>> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut battery_bank: Vec<u8> = Vec::with_capacity(line.len());

        for char in line.chars() {
            let n: u8 = match char {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => {
                    return None;
                }
            };
            battery_bank.push(n);
        }
        res.push(battery_bank);
    }

    return Some(res);
}

fn part_1_simple(input: &str) -> Option<u64> {
    let mut res = 0;

    for battery_bank in prarse_1_2(input)? {
        let mut max_found = 0;

        let l = battery_bank.len();

        for idx_a in 0..(l - 1) {
            for idx_b in (idx_a + 1)..l {
                let val = 10 * battery_bank[idx_a] + battery_bank[idx_b];
                if val > max_found {
                    max_found = val;
                }
            }
        }

        res += max_found as u64;
    }

    return Some(res);
}

fn part_1_better(input: &str) -> Option<u64> {
    let mut res: u64 = 0;

    for battery_bank in prarse_1_2(input)? {
        let l = battery_bank.len();

        let mut biggest = 0;
        let mut biggest_idx = 0;

        for (idx, joltage) in battery_bank.iter().enumerate() {
            if *joltage > biggest {
                biggest = *joltage;
                biggest_idx = idx;
            }
        }

        if biggest_idx < (l - 1) {
            // biggest joltage digit is not the last battery in the bank
            let mut _2nd_biggest = 0;
            for joltage in battery_bank[(biggest_idx + 1)..].iter() {
                if *joltage > _2nd_biggest {
                    _2nd_biggest = *joltage;
                }
            }

            res += (10 * biggest + _2nd_biggest) as u64;
        } else {
            // biggest joltage digit is the last battery in the bank
            let mut _2nd_biggest = 0;
            for joltage in battery_bank[0..biggest_idx].iter() {
                if *joltage > _2nd_biggest {
                    _2nd_biggest = *joltage;
                }
            }

            res += (10 * _2nd_biggest + biggest) as u64;
        }
    }

    return Some(res);
}

fn part_2_pick_next(
    battery_bank: &Vec<u8>,
    start_idx: usize,
    space_to_leave_to_the_right: usize,
) -> (u8, usize) {
    let mut biggest = 0;
    let mut biggest_idx = 0;
    let l = battery_bank.len() - space_to_leave_to_the_right;

    for idx in start_idx..l {
        let joltage = battery_bank[idx];
        if joltage > biggest {
            biggest = joltage;
            biggest_idx = idx;
        }
    }

    return (biggest, biggest_idx);
}

fn part_2(input: &str) -> Option<u64> {
    let mut res = 0;

    for battery_bank in prarse_1_2(input)? {
        let mut val: u64 = 0;
        let mut start_idx: usize = 0;
        let mut space_to_leave: usize = 11;

        for _ in 0..12 {
            let (big, next_idx) = part_2_pick_next(&battery_bank, start_idx, space_to_leave);
            val = 10 * val + big as u64;
            start_idx = next_idx + 1;
            space_to_leave = space_to_leave.wrapping_sub(1);
        }

        res += val;
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
