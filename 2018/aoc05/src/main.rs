// --- Day 5: Alchemical Reduction ---
// 
// You've managed to sneak in to the prototype suit manufacturing lab. The Elves are making decent progress, but are still struggling with the suit's size reduction capabilities.
// 
// While the very latest in 1518 alchemical technology might have solved their problem eventually, you can do better. You scan the chemical composition of the suit's material and discover that it is formed by extremely long polymers (one of which is available as your puzzle input).
// 
// The polymer is formed by smaller units which, when triggered, react with each other such that two adjacent units of the same type and opposite polarity are destroyed. Units' types are represented by letters; units' polarity is represented by capitalization. For instance, r and R are units with the same type but opposite polarity, whereas r and s are entirely different types and do not react.
// 
// For example:
// 
//     In aA, a and A react, leaving nothing behind.
//     In abBA, bB destroys itself, leaving aA. As above, this then destroys itself, leaving nothing.
//     In abAB, no two adjacent units are of the same type, and so nothing happens.
//     In aabAAB, even though aa and AA are of the same type, their polarities match, and so nothing happens.
// 
// Now, consider a larger example, dabAcCaCBAcCcaDA:
//                                 0123456789
//                                           012345
// 
// dabAcCaCBAcCcaDA  The first 'cC' is removed.
// dabAaCBAcCcaDA    This creates 'Aa', which is removed.
// dabCBAcCcaDA      Either 'cC' or 'Cc' are removed (the result is the same).
// dabCBAcaDA        No further actions can be taken.
// 
// After all possible reactions, the resulting polymer contains 10 units.
// 
// How many units remain after fully reacting the polymer you scanned? (Note: in this puzzle and others, the input is large; if you copy/paste your input, make sure you get the whole thing.)
// 
// To begin, get your puzzle input.
// 
// Your puzzle answer was 10450.
// 
// The first half of this puzzle is complete! It provides one gold star: *
// --- Part Two ---
// 
// Time to improve the polymer.
// 
// One of the unit types is causing problems; it's preventing the polymer from collapsing as much as it should. Your goal is to figure out which unit type is causing the most problems, remove all instances of it (regardless of polarity), fully react the remaining polymer, and measure its length.
// 
// For example, again using the polymer dabAcCaCBAcCcaDA from above:
// 
//     Removing all A/a units produces dbcCCBcCcD. Fully reacting this polymer produces dbCBcD, which has length 6.
//     Removing all B/b units produces daAcCaCAcCcaDA. Fully reacting this polymer produces daCAcaDA, which has length 8.
//     Removing all C/c units produces dabAaBAaDA. Fully reacting this polymer produces daDA, which has length 4.
//     Removing all D/d units produces abAcCaCBAcCcaA. Fully reacting this polymer produces abCBAc, which has length 6.
// 
// In this example, removing all C/c units was best, producing the answer 4.
// 
// What is the length of the shortest polymer you can produce by removing all units of exactly one type and fully reacting the result?
// 
// Although it hasn't changed, you can still get your puzzle input.

use std::fs;
use std::collections::HashSet;

fn read() -> String {
    return fs::read_to_string("input.txt").expect("bad file read").trim().to_string();
    // return String::from("dabAcCaCBAcCcaDA");
}

fn part1destroys(a: char, b: char) -> bool {
    a != b && a.to_uppercase().to_string() == b.to_uppercase().to_string()
}

fn part1(input: &String) -> usize {
    let mut did_change = true;
    let mut reacted = input.clone();
    // println!("{}", reacted);
    while did_change {
        did_change = false;

        let mut reactor = String::new();
        let mut skip = false;
        for (a, b) in reacted.chars().zip(reacted.chars().skip(1)) {
            if skip {
                skip = false;
                continue;
            }
            if part1destroys(a, b) {
                did_change = true;
                skip = true;
                // println!("skipping {} {}", a, b);
            } else {
                reactor.push(a);
            }
        }
        if !skip {
            reactor.push(reacted.chars().last().unwrap());
        }
        reacted = reactor;
        // println!("{}", reacted);
    }

    return reacted.len();
}

fn part1b(input: &String) -> usize {
    let mut reactor: Vec<char> = input.chars().collect();
    let mut idx = 0;
    while idx < reactor.len()-1 {
        let a = reactor[idx];
        let b = reactor[idx+1];
        if part1destroys(a, b) {
            // println!("removing at {}", idx);
            reactor.remove(idx);
            reactor.remove(idx);
            // let s = reactor.iter().fold(String::new(), |a, b| { let mut x = a.clone(); x.push(*b); return x; });
            // println!("{}", s);
            if idx > 0 {
                idx -= 1;
            }
        } else {
            idx += 1;
        }
    }

    return reactor.len();
}

fn part2(input: &String) -> usize {
    let mut chars = HashSet::new();

    for c in input.chars() {
        chars.insert(c.to_uppercase().to_string());
    }

    let mut best = 50_000;
    for c in chars {
        let s: String = input.chars().filter(|e| e.to_uppercase().to_string() != c).collect();
        let len = part1b(&s);
        if len < best {
            best = len;
        }
    }

    return best;
}

fn main() {
    let input = read();
    println!("len {}", input.len());
    println!("part1 {}", part1(&input));
    println!("part1b {}", part1b(&input));
    println!("part2 {}", part2(&input));
    println!("done");
}
