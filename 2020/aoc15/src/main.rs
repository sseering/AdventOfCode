// --- Day 15: Rambunctious Recitation ---
//
// You catch the airport shuttle and try to book a new flight to your vacation island. Due to the storm, all direct flights have been cancelled, but a route is available to get around the storm. You take it.
//
// While you wait for your flight, you decide to check in with the Elves back at the North Pole. They're playing a memory game and are ever so excited to explain the rules!
//
// In this game, the players take turns saying numbers. They begin by taking turns reading from a list of starting numbers (your puzzle input). Then, each turn consists of considering the most recently spoken number:
//
//     If that was the first time the number has been spoken, the current player says 0.
//     Otherwise, the number had been spoken before; the current player announces how many turns apart the number is from when it was previously spoken.
//
// So, after the starting numbers, each turn results in that player speaking aloud either 0 (if the last number is new) or an age (if the last number is a repeat).
//
// For example, suppose the starting numbers are 0,3,6:
//
//     Turn 1: The 1st number spoken is a starting number, 0.
//     Turn 2: The 2nd number spoken is a starting number, 3.
//     Turn 3: The 3rd number spoken is a starting number, 6.
//     Turn 4: Now, consider the last number spoken, 6. Since that was the first time the number had been spoken, the 4th number spoken is 0.
//     Turn 5: Next, again consider the last number spoken, 0. Since it had been spoken before, the next number to speak is the difference between the turn number when it was last spoken (the previous turn, 4) and the turn number of the time it was most recently spoken before then (turn 1). Thus, the 5th number spoken is 4 - 1, 3.
//     Turn 6: The last number spoken, 3 had also been spoken before, most recently on turns 5 and 2. So, the 6th number spoken is 5 - 2, 3.
//     Turn 7: Since 3 was just spoken twice in a row, and the last two turns are 1 turn apart, the 7th number spoken is 1.
//     Turn 8: Since 1 is new, the 8th number spoken is 0.
//     Turn 9: 0 was last spoken on turns 8 and 4, so the 9th number spoken is the difference between them, 4.
//     Turn 10: 4 is new, so the 10th number spoken is 0.
//
// (The game ends when the Elves get sick of playing or dinner is ready, whichever comes first.)
//
// Their question for you is: what will be the 2020th number spoken? In the example above, the 2020th number spoken will be 436.
//
// Here are a few more examples:
//
//     Given the starting numbers 1,3,2, the 2020th number spoken is 1.
//     Given the starting numbers 2,1,3, the 2020th number spoken is 10.
//     Given the starting numbers 1,2,3, the 2020th number spoken is 27.
//     Given the starting numbers 2,3,1, the 2020th number spoken is 78.
//     Given the starting numbers 3,2,1, the 2020th number spoken is 438.
//     Given the starting numbers 3,1,2, the 2020th number spoken is 1836.
//
// Given your starting numbers, what will be the 2020th number spoken?
//
// Your puzzle input is 11,18,0,20,1,7,16.
//
// The first half of this puzzle is complete! It provides one gold star: *
//
// --- Part Two ---
//
// Impressed, the Elves issue you a challenge: determine the 30000000th number spoken. For example, given the same starting numbers as above:
//
//     Given 0,3,6, the 30000000th number spoken is 175594.
//     Given 1,3,2, the 30000000th number spoken is 2578.
//     Given 2,1,3, the 30000000th number spoken is 3544142.
//     Given 1,2,3, the 30000000th number spoken is 261214.
//     Given 2,3,1, the 30000000th number spoken is 6895259.
//     Given 3,2,1, the 30000000th number spoken is 18.
//     Given 3,1,2, the 30000000th number spoken is 362.
//
// Given your starting numbers, what will be the 30000000th number spoken?

use std::collections::HashMap;

#[allow(unused)]
const TEST_INPUT_A: [u64; 3] = [0, 3, 6];

#[allow(unused)]
const TEST_INPUT_B: [u64; 3] = [1, 3, 2];

#[allow(unused)]
const TEST_INPUT_C: [u64; 3] = [2, 1, 3];

#[allow(unused)]
const TEST_INPUT_D: [u64; 3] = [1, 2, 3];

#[allow(unused)]
const TEST_INPUT_E: [u64; 3] = [2, 3, 1];

#[allow(unused)]
const TEST_INPUT_F: [u64; 3] = [3, 2, 1];

#[allow(unused)]
const TEST_INPUT_G: [u64; 3] = [3, 1, 2];

#[allow(unused)]
const INPUT: [u64; 7] = [11, 18, 0, 20, 1, 7, 16];

#[allow(unused)]
fn part_1(starting_numbers: &[u64]) -> u64 {
    return part_1_2(starting_numbers, 2020);
}

#[allow(unused)]
fn part_2(starting_numbers: &[u64]) -> u64 {
    return part_1_2(starting_numbers, 30000000);
}

fn part_1_2(starting_numbers: &[u64], turn_limit: u64) -> u64 {
    let mut last_seen: HashMap<u64, u64> = HashMap::new();

    let mut turn: u64 = 1;
    for &n in starting_numbers {
        last_seen.insert(n, turn);
        turn += 1;
    }

    let mut last_spoken: u64 = 0;

    while turn < turn_limit {
        let lsg = last_seen.get(&last_spoken);
        let to_speak: u64 = match lsg {
            Some(&last_turn) => turn - last_turn,
            None => 0,
        };
        last_seen.insert(last_spoken, turn);
        turn += 1;
        last_spoken = to_speak;
    }

    return last_spoken;
}

#[test]
fn test_a() {
    assert_eq!(part_1(&TEST_INPUT_A), 436);
}

#[test]
fn test_b() {
    assert_eq!(part_1(&TEST_INPUT_B), 1);
}

#[test]
fn test_c() {
    assert_eq!(part_1(&TEST_INPUT_C), 10);
}

#[test]
fn test_d() {
    assert_eq!(part_1(&TEST_INPUT_D), 27);
}

#[test]
fn test_e() {
    assert_eq!(part_1(&TEST_INPUT_E), 78);
}

#[test]
fn test_f() {
    assert_eq!(part_1(&TEST_INPUT_F), 438);
}

#[test]
fn test_g() {
    assert_eq!(part_1(&TEST_INPUT_G), 1836);
}

#[test]
fn test_aa() {
    assert_eq!(part_2(&TEST_INPUT_A), 175594);
}

#[test]
fn test_bb() {
    assert_eq!(part_2(&TEST_INPUT_B), 2578);
}

#[test]
fn test_cc() {
    assert_eq!(part_2(&TEST_INPUT_C), 3544142);
}

#[test]
fn test_dd() {
    assert_eq!(part_2(&TEST_INPUT_D), 261214);
}

#[test]
fn test_ee() {
    assert_eq!(part_2(&TEST_INPUT_E), 6895259);
}

#[test]
fn test_ff() {
    assert_eq!(part_2(&TEST_INPUT_F), 18);
}

#[test]
fn test_gg() {
    assert_eq!(part_2(&TEST_INPUT_G), 362);
}

fn main() {
    println!("Part 1: {}", part_1(&INPUT));
    println!("Part 2: {}", part_2(&INPUT));
    println!("done.");
}
