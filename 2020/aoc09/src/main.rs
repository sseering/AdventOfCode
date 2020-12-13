// --- Day 9: Encoding Error ---
//
// With your neighbor happily enjoying their video game, you turn your attention to an open data port on the little screen in the seat in front of you.
//
// Though the port is non-standard, you manage to connect it to your computer through the clever use of several paperclips. Upon connection, the port outputs a series of numbers (your puzzle input).
//
// The data appears to be encrypted with the eXchange-Masking Addition System (XMAS) which, conveniently for you, is an old cypher with an important weakness.
//
// XMAS starts by transmitting a preamble of 25 numbers. After that, each number you receive should be the sum of any two of the 25 immediately previous numbers. The two numbers will have different values, and there might be more than one such pair.
//
// For example, suppose your preamble consists of the numbers 1 through 25 in a random order. To be valid, the next number must be the sum of two of those numbers:
//
//     26 would be a valid next number, as it could be 1 plus 25 (or many other pairs, like 2 and 24).
//     49 would be a valid next number, as it is the sum of 24 and 25.
//     100 would not be valid; no two of the previous 25 numbers sum to 100.
//     50 would also not be valid; although 25 appears in the previous 25 numbers, the two numbers in the pair must be different.
//
// Suppose the 26th number is 45, and the first number (no longer an option, as it is more than 25 numbers ago) was 20. Now, for the next number to be valid, there needs to be some pair of numbers among 1-19, 21-25, or 45 that add up to it:
//
//     26 would still be a valid next number, as 1 and 25 are still within the previous 25 numbers.
//     65 would not be valid, as no two of the available numbers sum to it.
//     64 and 66 would both be valid, as they are the result of 19+45 and 21+45 respectively.
//
// Here is a larger example which only considers the previous 5 numbers (and has a preamble of length 5):
//
// 35
// 20
// 15
// 25
// 47
// 40
// 62
// 55
// 65
// 95
// 102
// 117
// 150
// 182
// 127
// 219
// 299
// 277
// 309
// 576
//
// In this example, after the 5-number preamble, almost every number is the sum of two of the previous 5 numbers; the only number that does not follow this rule is 127.
//
// The first step of attacking the weakness in the XMAS data is to find the first number in the list (after the preamble) which is not the sum of two of the 25 numbers before it. What is the first number that does not have this property?
//
// The first half of this puzzle is complete! It provides one gold star: *
// --- Part Two ---
//
// The final step in breaking the XMAS encryption relies on the invalid number you just found: you must find a contiguous set of at least two numbers in your list which sum to the invalid number from step 1.
//
// Again consider the above example:
//
// 35
// 20
// 15
// 25
// 47
// 40
// 62
// 55
// 65
// 95
// 102
// 117
// 150
// 182
// 127
// 219
// 299
// 277
// 309
// 576
//
// In this list, adding up all of the numbers from 15 through 40 produces the invalid number from step 1, 127. (Of course, the contiguous set of numbers in your actual list might be much longer.)
//
// To find the encryption weakness, add together the smallest and largest number in this contiguous range; in this example, these are 15 and 47, producing 62.
//
// What is the encryption weakness in your XMAS-encrypted list of numbers?

use std::collections::VecDeque;

#[allow(unused)]
const TEST_INPUT_A: &str =
    "20\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n1\n21\n22\n23\n24\n25
26";

#[allow(unused)]
const TEST_INPUT_B: &str =
    "20\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n1\n21\n22\n23\n24\n25
49";

#[allow(unused)]
const TEST_INPUT_C: &str =
    "20\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n1\n21\n22\n23\n24\n25
100";

#[allow(unused)]
const TEST_INPUT_D: &str =
    "20\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n1\n21\n22\n23\n24\n25
50";

#[allow(unused)]
const TEST_INPUT_E: &str =
    "20\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n1\n21\n22\n23\n24\n25
45
26";

#[allow(unused)]
const TEST_INPUT_F: &str =
    "20\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n1\n21\n22\n23\n24\n25
45
65";

#[allow(unused)]
const TEST_INPUT_G: &str =
    "20\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n1\n21\n22\n23\n24\n25
45
64";

#[allow(unused)]
const TEST_INPUT_H: &str =
    "20\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n1\n21\n22\n23\n24\n25
45
66";

#[allow(unused)]
const TEST_INPUT_I: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

#[allow(unused)]
const INPUT: &str = include_str!("input");

#[allow(unused)]
fn part_1(xmas_numbers: &str, preamble_len: usize) -> Option<i64> {
    if preamble_len == 0 {
        return None;
    }

    let nums: Vec<i64> = xmas_numbers
        .lines()
        .map(|l| -> i64 {
            return l.parse().unwrap();
        })
        .collect();

    let mut sums: Vec<VecDeque<i64>> = Vec::new();
    for _ in 0..preamble_len {
        sums.push(VecDeque::new());
    }

    // build database of sums from preamble
    for pair_span in 1..preamble_len {
        for low_idx in 0..(preamble_len - pair_span) {
            let high_idx: usize = low_idx + pair_span;
            sums[pair_span - 1].push_back(nums[low_idx] + nums[high_idx]);
        }
    }

    for idx in preamble_len..nums.len() {
        let n = nums[idx];

        // check if n is a sum we have in out database already
        let mut found = false;
        'num_is_present_check: for sums_sublist in sums.iter() {
            for &sum in sums_sublist.iter() {
                if sum == n {
                    found = true;
                    break 'num_is_present_check;
                }
            }
        }

        if !found {
            return Some(n);
        }

        // remove old values from database and add new ones containing the new number
        for pair_span in 1..preamble_len {
            sums[pair_span - 1].pop_front();

            let high_idx = idx;
            let low_idx = high_idx - pair_span;
            sums[pair_span - 1].push_back(nums[low_idx] + nums[high_idx]);
        }
    }

    return None;
}

#[allow(unused)]
fn part_2(xmas_numbers: &str, preamble_len: usize) -> Option<i64> {
    let target = part_1(xmas_numbers, preamble_len)?;

    let nums: Vec<i64> = xmas_numbers
        .lines()
        .map(|l| -> i64 {
            return l.parse().unwrap();
        })
        .collect();
    let nums_len = nums.len();

    let mut begin_idx: usize = 0;
    let mut end_idx: usize = 0;
    let mut cummulative_sum: i64 = nums[0];

    if cummulative_sum == target {
        return None;
    }

    let mut endless_loop_check = 0;
    while endless_loop_check < 0xfff0 {
        endless_loop_check += 1;

        if cummulative_sum < target {
            end_idx += 1;
            if end_idx >= nums_len {
                return None;
            }
            cummulative_sum += nums[end_idx];
        } else {
            cummulative_sum -= nums[begin_idx];

            begin_idx += 1;
            if begin_idx > end_idx {
                // this should never happen?
                return None;
            }
        }

        if cummulative_sum == target {
            let mut min: i64 = std::i64::MAX;
            let mut max: i64 = std::i64::MIN;
            for &n in nums[begin_idx..=end_idx].iter() {
                min = std::cmp::min(min, n);
                max = std::cmp::max(max, n);
            }
            return Some(min + max);
        }
    }

    return None;
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT_A, 25), None);
}

#[test]
fn test_b() {
    assert_eq!(part_1(TEST_INPUT_B, 25), None);
}

#[test]
fn test_c() {
    assert_eq!(part_1(TEST_INPUT_C, 25), Some(100));
}

#[test]
fn test_d() {
    assert_eq!(part_1(TEST_INPUT_D, 25), Some(50));
}

#[test]
fn test_e() {
    assert_eq!(part_1(TEST_INPUT_E, 25), None);
}

#[test]
fn test_f() {
    assert_eq!(part_1(TEST_INPUT_F, 25), Some(65));
}

#[test]
fn test_g() {
    assert_eq!(part_1(TEST_INPUT_G, 25), None);
}

#[test]
fn test_h() {
    assert_eq!(part_1(TEST_INPUT_H, 25), None);
}

#[test]
fn test_i() {
    assert_eq!(part_1(TEST_INPUT_I, 5), Some(127));
}

#[test]
fn test_j() {
    assert_eq!(part_2(TEST_INPUT_I, 5), Some(62));
}

fn main() {
    match part_1(INPUT, 25) {
        None => {
            println!("Part 1: every number was in set of sums.");
        }
        Some(p) => {
            println!("Part 1: {}", p);
        }
    }
    match part_2(INPUT, 25) {
        None => {
            println!("Part 2: every number was in set of sums or no contigous subset was found.");
        }
        Some(p) => {
            println!("Part 2: {}", p);
        }
    }
    println!("done.");
}
