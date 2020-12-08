// --- Day 8: Handheld Halting ---
//
// Your flight to the major airline hub reaches cruising altitude without incident. While you consider checking the in-flight menu for one of those drinks that come with a little umbrella, you are interrupted by the kid sitting next to you.
//
// Their handheld game console won't turn on! They ask if you can take a look.
//
// You narrow the problem down to a strange infinite loop in the boot code (your puzzle input) of the device. You should be able to fix it, but first you need to be able to run the code in isolation.
//
// The boot code is represented as a text file with one instruction per line of text. Each instruction consists of an operation (acc, jmp, or nop) and an argument (a signed number like +4 or -20).
//
//     acc increases or decreases a single global value called the accumulator by the value given in the argument. For example, acc +7 would increase the accumulator by 7. The accumulator starts at 0. After an acc instruction, the instruction immediately below it is executed next.
//     jmp jumps to a new instruction relative to itself. The next instruction to execute is found using the argument as an offset from the jmp instruction; for example, jmp +2 would skip the next instruction, jmp +1 would continue to the instruction immediately below it, and jmp -20 would cause the instruction 20 lines above to be executed next.
//     nop stands for No OPeration - it does nothing. The instruction immediately below it is executed next.
//
// For example, consider the following program:
//
// nop +0
// acc +1
// jmp +4
// acc +3
// jmp -3
// acc -99
// acc +1
// jmp -4
// acc +6
//
// These instructions are visited in this order:
//
// nop +0  | 1
// acc +1  | 2, 8(!)
// jmp +4  | 3
// acc +3  | 6
// jmp -3  | 7
// acc -99 |
// acc +1  | 4
// jmp -4  | 5
// acc +6  |
//
// First, the nop +0 does nothing. Then, the accumulator is increased from 0 to 1 (acc +1) and jmp +4 sets the next instruction to the other acc +1 near the bottom. After it increases the accumulator from 1 to 2, jmp -4 executes, setting the next instruction to the only acc +3. It sets the accumulator to 5, and jmp -3 causes the program to continue back at the first acc +1.
//
// This is an infinite loop: with this sequence of jumps, the program will run forever. The moment the program tries to run any instruction a second time, you know it will never terminate.
//
// Immediately before the program would run an instruction a second time, the value in the accumulator is 5.
//
// Run your copy of the boot code. Immediately before any instruction is executed a second time, what value is in the accumulator?
//
// To begin, get your puzzle input.
//
// --- Part Two ---
//
// After some careful analysis, you believe that exactly one instruction is corrupted.
//
// Somewhere in the program, either a jmp is supposed to be a nop, or a nop is supposed to be a jmp. (No acc instructions were harmed in the corruption of this boot code.)
//
// The program is supposed to terminate by attempting to execute an instruction immediately after the last instruction in the file. By changing exactly one jmp or nop, you can repair the boot code and make it terminate correctly.
//
// For example, consider the same program from above:
//
// nop +0
// acc +1
// jmp +4
// acc +3
// jmp -3
// acc -99
// acc +1
// jmp -4
// acc +6
//
// If you change the first instruction from nop +0 to jmp +0, it would create a single-instruction infinite loop, never leaving that instruction. If you change almost any of the jmp instructions, the program will still eventually find another jmp instruction and loop forever.
//
// However, if you change the second-to-last instruction (from jmp -4 to nop -4), the program terminates! The instructions are visited in this order:
//
// nop +0  | 1
// acc +1  | 2
// jmp +4  | 3
// acc +3  |
// jmp -3  |
// acc -99 |
// acc +1  | 4
// nop -4  | 5
// acc +6  | 6
//
// After the last instruction (acc +6), the program terminates by attempting to run the instruction below the last instruction in the file. With this change, after the program terminates, the accumulator contains the value 8 (acc +1, acc +1, acc +6).
//
// Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to jmp). What is the value of the accumulator after the program terminates?

use regex::Regex;
#[allow(unused)]
use std::error::Error;
use std::str::FromStr;

#[allow(unused)]
const TEST_INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

#[allow(unused)]
const INPUT: &str = include_str!("input");

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

impl FromStr for Op {
    // alternatively use the enum_derive crate

    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "nop" => Ok(Self::Nop),
            "acc" => Ok(Self::Acc),
            "jmp" => Ok(Self::Jmp),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Instr {
    op: Op,
    arg: i32,
}

impl Instr {
    fn new(op: &str, arg: &str) -> Option<Self> {
        Some(Self {
            op: Op::from_str(op).ok()?,
            arg: arg.parse().ok()?,
        })
    }
}

fn parse_program(instructions: &str) -> Option<Vec<Instr>> {
    let re = Regex::new(r"(\w+)\s+([-+]\d+)").unwrap();

    return instructions
        .lines()
        .map(|line| -> Option<Instr> {
            let captures = re.captures(line)?;
            return Some(Instr::new(
                captures.get(1)?.as_str(),
                captures.get(2)?.as_str(),
            )?);
        })
        .collect();
}

#[allow(unused)]
fn part_1(instructions: &str) -> Option<i32> {
    let mut accumulator = 0;
    let instrs = parse_program(instructions)?;
    let mut instr_seen: Vec<bool> = vec![false; instrs.len()];
    let mut ip: i32 = 0;

    while !instr_seen[ip as usize] {
        instr_seen[ip as usize] = true;
        let instr = &instrs[ip as usize];
        match instr.op {
            Op::Nop => {}
            Op::Acc => {
                accumulator += instr.arg;
            }
            Op::Jmp => {
                ip = ip + instr.arg - 1;
            }
        }
        ip += 1;
    }

    return Some(accumulator);
}

#[allow(unused)]
fn part_2(instructions: &str) -> Option<i32> {
    let mut accumulator = 0;
    let instrs = parse_program(instructions)?;
    let instrs_len: i32 = instrs.len() as i32;
    let mut instr_seen: Vec<bool> = vec![false; instrs.len()];
    let mut ip: i32 = 0;

    // first attempt to replace nop -> jmp
    while !instr_seen[ip as usize] {
        instr_seen[ip as usize] = true;
        let instr = &instrs[ip as usize];
        match instr.op {
            Op::Nop => {
                let do_replacement = ip + instr.arg == instrs_len;
                if do_replacement {
                    return Some(accumulator);
                }
            }
            Op::Acc => {
                accumulator += instr.arg;
            }
            Op::Jmp => {
                ip = ip + instr.arg - 1;
            }
        }
        ip += 1;
    }

    // attempt to replace jmp -> nop
    let mut jmp_instr_indices: Vec<i32> = instrs
        .iter()
        .enumerate()
        .filter(|(idx, instr)| instr.op == Op::Jmp)
        .map(|(idx, instr)| idx as i32)
        .collect();

    for replacement_idx in jmp_instr_indices {
        accumulator = 0;
        instr_seen = vec![false; instrs.len()];
        ip = 0;

        while ip < instrs_len && ip >= 0 && !instr_seen[ip as usize] {
            instr_seen[ip as usize] = true;
            let instr = &instrs[ip as usize];
            match instr.op {
                Op::Nop => {}
                Op::Acc => {
                    accumulator += instr.arg;
                }
                Op::Jmp => {
                    let do_replacement = ip == replacement_idx;
                    if !do_replacement {
                        ip = ip + instr.arg - 1;
                    }
                }
            }
            ip += 1;
        }
        if ip == instrs_len {
            return Some(accumulator);
        }
    }

    return None;
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), Some(5));
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), Some(8));
}

fn main() {
    match part_1(INPUT) {
        Some(acc) => {
            println!("Part 1: {}", acc);
        }
        None => {
            println!("Part 1 parse failed");
        }
    }
    match part_2(INPUT) {
        Some(acc) => {
            println!("Part 2: {}", acc);
        }
        None => {
            println!("Part 2 parse failed");
        }
    }
    println!("done.");
}
