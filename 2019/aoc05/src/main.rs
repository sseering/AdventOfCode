// --- Day 5: Sunny with a Chance of Asteroids ---
//
// You're starting to sweat as the ship makes its way toward Mercury. The Elves suggest that you get the air conditioner working by upgrading your ship computer to support the Thermal Environment Supervision Terminal.
//
// The Thermal Environment Supervision Terminal (TEST) starts by running a diagnostic program (your puzzle input). The TEST diagnostic program will run on your existing Intcode computer after a few modifications:
//
// First, you'll need to add two new instructions:
//
//     Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. For example, the instruction 3,50 would take an input value and store it at address 50.
//     Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50.
//
// Programs that use these instructions will come with documentation that explains what should be connected to the input and output. The program 3,0,4,0,99 outputs whatever it gets as input, then halts.
//
// Second, you'll need to add support for parameter modes:
//
// Each parameter of an instruction is handled based on its parameter mode. Right now, your ship computer already understands parameter mode 0, position mode, which causes the parameter to be interpreted as a position - if the parameter is 50, its value is the value stored at address 50 in memory. Until now, all parameters have been in position mode.
//
// Now, your ship computer will also need to handle parameters in mode 1, immediate mode. In immediate mode, a parameter is interpreted as a value - if the parameter is 50, its value is simply 50.
//
// Parameter modes are stored in the same value as the instruction's opcode. The opcode is a two-digit number based only on the ones and tens digit of the value, that is, the opcode is the rightmost two digits of the first value in an instruction. Parameter modes are single digits, one per parameter, read right-to-left from the opcode: the first parameter's mode is in the hundreds digit, the second parameter's mode is in the thousands digit, the third parameter's mode is in the ten-thousands digit, and so on. Any missing modes are 0.
//
// For example, consider the program 1002,4,3,4,33.
//
// The first instruction, 1002,4,3,4, is a multiply instruction - the rightmost two digits of the first value, 02, indicate opcode 2, multiplication. Then, going right to left, the parameter modes are 0 (hundreds digit), 1 (thousands digit), and 0 (ten-thousands digit, not present and therefore zero):
//
// ABCDE
//  1002
//
// DE - two-digit opcode,      02 == opcode 2
//  C - mode of 1st parameter,  0 == position mode
//  B - mode of 2nd parameter,  1 == immediate mode
//  A - mode of 3rd parameter,  0 == position mode,
//                                   omitted due to being a leading zero
//
// This instruction multiplies its first two parameters. The first parameter, 4 in position mode, works like it did before - its value is the value stored at address 4 (33). The second parameter, 3 in immediate mode, simply has value 3. The result of this operation, 33 * 3 = 99, is written according to the third parameter, 4 in position mode, which also works like it did before - 99 is written to address 4.
//
// Parameters that an instruction writes to will never be in immediate mode.
//
// Finally, some notes:
//
//     It is important to remember that the instruction pointer should increase by the number of values in the instruction after the instruction finishes. Because of the new instructions, this amount is no longer always 4.
//     Integers can be negative: 1101,100,-1,4,0 is a valid program (find 100 + -1, store the result in position 4).
//
// The TEST diagnostic program will start by requesting from the user the ID of the system to test by running an input instruction - provide it 1, the ID for the ship's air conditioner unit.
//
// It will then perform a series of diagnostic tests confirming that various parts of the Intcode computer, like parameter modes, function correctly. For each test, it will run an output instruction indicating how far the result of the test was from the expected value, where 0 means the test was successful. Non-zero outputs mean that a function is not working correctly; check the instructions that were run before the output instruction to see which one failed.
//
// Finally, the program will output a diagnostic code and immediately halt. This final output isn't an error; an output followed immediately by a halt means the program finished. If all outputs were zero except the diagnostic code, the diagnostic program ran successfully.
//
// After providing 1 to the only input instruction and passing all the tests, what diagnostic code does the program produce?
//
// To begin, get your puzzle input.
//
// Your puzzle answer was 16489636.
//
// The first half of this puzzle is complete! It provides one gold star: *
// --- Part Two ---
//
// The air conditioner comes online! Its cold air feels good for a while, but then the TEST alarms start to go off. Since the air conditioner can't vent its heat anywhere but back into the spacecraft, it's actually making the air inside the ship warmer.
//
// Instead, you'll need to use the TEST to extend the thermal radiators. Fortunately, the diagnostic program (your puzzle input) is already equipped for this. Unfortunately, your Intcode computer is not.
//
// Your computer is only missing a few opcodes:
//
//     Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
//     Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
//     Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
//     Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
//
// Like all instructions, these instructions need to support parameter modes as described above.
//
// Normally, after an instruction is finished, the instruction pointer increases by the number of values in that instruction. However, if the instruction modifies the instruction pointer, that value is used and the instruction pointer is not automatically increased.
//
// For example, here are several programs that take one input, compare it to the value 8, and then produce one output:
//
//     3,9,8,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
//     3,9,7,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
//     3,3,1108,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
//     3,3,1107,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
//
// Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
//
//     3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9 (using position mode)
//     3,3,1105,-1,9,1101,0,0,12,4,12,99,1 (using immediate mode)
//
// Here's a larger example:
//
// 3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
// 1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
// 999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
//
// The above example program uses an input instruction to ask for a single number. The program will then output 999 if the input value is below 8, output 1000 if the input value is equal to 8, or output 1001 if the input value is greater than 8.
//
// This time, when the TEST diagnostic program runs its input instruction to get the ID of the system to test, provide it 5, the ID for the ship's thermal radiator controller. This diagnostic test suite only outputs one number, the diagnostic code.
//
// What is the diagnostic code for system ID 5?
//
// Although it hasn't changed, you can still get your puzzle input.

use std::collections::VecDeque;

#[allow(unused)]
const INPUT: & str = "3,225,1,225,6,6,1100,1,238,225,104,0,1102,91,92,225,1102,85,13,225,1,47,17,224,101,-176,224,224,4,224,1002,223,8,223,1001,224,7,224,1,223,224,223,1102,79,43,225,1102,91,79,225,1101,94,61,225,1002,99,42,224,1001,224,-1890,224,4,224,1002,223,8,223,1001,224,6,224,1,224,223,223,102,77,52,224,1001,224,-4697,224,4,224,102,8,223,223,1001,224,7,224,1,224,223,223,1101,45,47,225,1001,43,93,224,1001,224,-172,224,4,224,102,8,223,223,1001,224,1,224,1,224,223,223,1102,53,88,225,1101,64,75,225,2,14,129,224,101,-5888,224,224,4,224,102,8,223,223,101,6,224,224,1,223,224,223,101,60,126,224,101,-148,224,224,4,224,1002,223,8,223,1001,224,2,224,1,224,223,223,1102,82,56,224,1001,224,-4592,224,4,224,1002,223,8,223,101,4,224,224,1,224,223,223,1101,22,82,224,1001,224,-104,224,4,224,1002,223,8,223,101,4,224,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,8,226,677,224,102,2,223,223,1005,224,329,1001,223,1,223,1007,226,226,224,1002,223,2,223,1006,224,344,101,1,223,223,108,226,226,224,1002,223,2,223,1006,224,359,1001,223,1,223,107,226,677,224,102,2,223,223,1006,224,374,101,1,223,223,8,677,677,224,102,2,223,223,1006,224,389,1001,223,1,223,1008,226,677,224,1002,223,2,223,1006,224,404,101,1,223,223,7,677,677,224,1002,223,2,223,1005,224,419,101,1,223,223,1108,226,677,224,1002,223,2,223,1005,224,434,101,1,223,223,1108,226,226,224,102,2,223,223,1005,224,449,1001,223,1,223,107,226,226,224,102,2,223,223,1005,224,464,101,1,223,223,1007,677,677,224,102,2,223,223,1006,224,479,101,1,223,223,1007,226,677,224,102,2,223,223,1005,224,494,1001,223,1,223,1008,226,226,224,1002,223,2,223,1005,224,509,1001,223,1,223,1108,677,226,224,1002,223,2,223,1006,224,524,1001,223,1,223,108,677,677,224,1002,223,2,223,1005,224,539,101,1,223,223,108,226,677,224,1002,223,2,223,1005,224,554,101,1,223,223,1008,677,677,224,1002,223,2,223,1006,224,569,1001,223,1,223,1107,677,677,224,102,2,223,223,1005,224,584,1001,223,1,223,7,677,226,224,102,2,223,223,1005,224,599,1001,223,1,223,8,677,226,224,1002,223,2,223,1005,224,614,1001,223,1,223,7,226,677,224,1002,223,2,223,1006,224,629,101,1,223,223,1107,677,226,224,1002,223,2,223,1005,224,644,1001,223,1,223,1107,226,677,224,102,2,223,223,1006,224,659,1001,223,1,223,107,677,677,224,1002,223,2,223,1005,224,674,101,1,223,223,4,223,99,226";

#[allow(unused)]
const TEST_INPUT_A: & str = "3,0,4,0,99";

#[allow(unused)]
const TEST_INPUT_B: & str = "1002,4,3,4,33";

#[allow(unused)]
const TEST_INPUT_C: & str = "1101,100,-1,4,0";

#[allow(unused)]
const TEST_INPUT_D: & str = "3,9,8,9,10,9,4,9,99,-1,8";

#[allow(unused)]
const TEST_INPUT_E: & str = "3,9,7,9,10,9,4,9,99,-1,8";

#[allow(unused)]
const TEST_INPUT_F: & str = "3,3,1108,-1,8,3,4,3,99";

#[allow(unused)]
const TEST_INPUT_G: & str = "3,3,1107,-1,8,3,4,3,99";

#[allow(unused)]
const TEST_INPUT_H: & str = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";

#[allow(unused)]
const TEST_INPUT_I: & str = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";

#[allow(unused)]
const TEST_INPUT_J: & str = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

fn parse(input: & str) -> Vec<i32> {
    input.split(",").map(|s| -> i32 { s.parse().unwrap() }).collect()
}

#[derive(PartialEq)]
enum Op {
    Calculation(fn(i32, i32) -> i32, ParamMode, ParamMode),
    Input,
    Output(ParamMode),
    ContitionalJump(fn(i32) -> bool, ParamMode, ParamMode),
    End,
}

impl Op {
    fn new(opcode: i32) -> Op {
        const ADD: i32 = 1;
        const MUL: i32 = 2;
        const INP: i32 = 3;
        const OUT: i32 = 4;
        const JMPT: i32 = 5;
        const JMPF: i32 = 6;
        const LT: i32 = 7;
        const EQ: i32 = 8;
        const END: i32 = 99;

        let low_opcode = opcode % 100;
        let opcode = opcode / 100;

        let (p1, p2) = if low_opcode == ADD || low_opcode == MUL || low_opcode == LT || low_opcode == EQ {
            let p1 = ParamMode::from_int(opcode % 10);

            let opcode = opcode / 10;
            let p2 = ParamMode::from_int(opcode % 10);

            let opcode = opcode / 10;
            let p3 = ParamMode::from_int(opcode % 10);

            if p3 != ParamMode::Position {
                panic!();
            }

            (p1, p2)
        } else if low_opcode == INP {
            let p1 = ParamMode::from_int(opcode % 10);
            if p1 != ParamMode::Position {
                panic!();
            }

            (ParamMode::Immediate, ParamMode::Immediate)
        } else if low_opcode == OUT {
            let p1 = ParamMode::from_int(opcode % 10);

            (p1, ParamMode::Immediate)
        } else if low_opcode == JMPT || low_opcode == JMPF {
            let p1 = ParamMode::from_int(opcode % 10);

            let opcode = opcode / 10;
            let p2 = ParamMode::from_int(opcode % 10);

            (p1, p2)
        } else {
            (ParamMode::Immediate, ParamMode::Immediate)
        };

        return match low_opcode {
            ADD => Op::Calculation(|a, b| { return a + b }, p1, p2),
            MUL => Op::Calculation(|a, b| { return a * b }, p1, p2),
            INP => Op::Input,
            OUT => Op::Output(p1),
            JMPT => Op::ContitionalJump(|a| { return a != 0 }, p1, p2),
            JMPF => Op::ContitionalJump(|a| { return a == 0 }, p1, p2),
            LT => Op::Calculation(|a, b| { return if a < b { 1 } else { 0 } }, p1, p2),
            EQ => Op::Calculation(|a, b| { return if a == b { 1 } else { 0 } }, p1, p2),
            END => Op::End,
            _ => panic!("{}", low_opcode),
        };
    }

    fn operate(&self, prog: & mut Vec<i32>, op_idx: usize, input: & mut VecDeque<i32>) -> (usize, Option<i32>) {
        match self {
            Op::End => { panic!(); },
            Op::Input => {
                let addr = ParamMode::Immediate.get_param(prog, op_idx + 1) as usize;
                prog[addr] = input.pop_front().unwrap();
                return (op_idx + 2, None);
            }
            Op::Output(pm) => {
                return (op_idx + 2, Some(pm.get_param(prog, op_idx + 1)));
            }
            Op::Calculation(fun, pm1, pm2) => {
                let val_a = pm1.get_param(prog, op_idx + 1);
                let val_b = pm2.get_param(prog, op_idx + 2);
                let addr = ParamMode::Immediate.get_param(prog, op_idx + 3) as usize;
                // println!("[{}] = {} =  {} x {} ", addr, fun(val_a, val_b), val_a, val_b);
                prog[addr] = fun(val_a, val_b);
                return (op_idx + 4, None);
            },
            Op::ContitionalJump(cond, pm1, pm2) => {
                let cond_val = pm1.get_param(prog, op_idx + 1);
                let jump_dst = pm2.get_param(prog, op_idx + 2) as usize;
                return (if cond(cond_val) { jump_dst } else { op_idx + 3}, None);
            }
        }
    }
}

#[derive(PartialEq)]
enum ParamMode {
    Immediate,
    Position,
}

impl ParamMode {
    fn from_int(i: i32) -> ParamMode {
        match i {
            1 => ParamMode::Immediate,
            0 => ParamMode::Position,
            _ => panic!(),
        }
    }

    fn get_param(&self, prog: & Vec<i32>, idx: usize) -> i32 {
        match self {
            ParamMode::Immediate => prog[idx],
            ParamMode::Position => prog[prog[idx] as usize],
        }
    }
}

fn part1(puzzle_input: & str, input: & mut VecDeque<i32>) -> (Vec<i32>, VecDeque<i32>) {
    let mut prog = parse(puzzle_input);
    let mut ip = 0;
    let mut output: VecDeque<i32> = VecDeque::new();

    loop {
        // println!("Op::new on [ip]: {}", ip);
        let op = Op::new(prog[ip]);

        if op == Op::End {
            break;
        }

        let (new_ip, maybe_out) = op.operate(&mut prog, ip, input);
        if let Some(out) = maybe_out {
            output.push_back(out);
        }

        ip = new_ip;
    }

    return (prog, output);
}

fn part2(puzzle_input: & str, input: & mut VecDeque<i32>) -> (Vec<i32>, VecDeque<i32>) {
    return part1(puzzle_input, input);
}

fn deque_one(i: i32) -> VecDeque<i32> {
    let mut res: VecDeque<i32> = VecDeque::new();
    res.push_back(i);
    return res;
}

fn main() {
    let mut tested = 0;
    let mut good = 0;
    for i in 0..20 {
        tested += 1;
        if part1(TEST_INPUT_A, & mut deque_one(i)) == (vec![i, 0, 4, 0, 99], deque_one(i)) {
            good += 1;
        }
    }
    println!("part 1 selftest 1 tested/good/bad: {0}/{1}/{2} => {3}", tested, good, tested - good, if tested == good { "good" } else { "OMG BADD BAD BAD!" });
    let mut empty: VecDeque<i32> = VecDeque::new();
    println!("part 1 selftest 2 good: {}", part1(TEST_INPUT_B, & mut empty) == (vec![1002, 4, 3, 4, 99], empty));
    let mut empty: VecDeque<i32> = VecDeque::new();
    println!("part 1 selftest 3 good: {}", part1(TEST_INPUT_C, & mut empty) == (vec![1101, 100, -1, 4, 99], empty));
    println!("part 1: {:?}", part1(INPUT, & mut deque_one(1)).1);

    tested = 0;
    good = 0;
    for i in 0..20 {
        let i_eq_8 = if i == 8 { 1 } else { 0 };
        let i_lt_8 = if i < 8 { 1 } else { 0 };

        tested += 1;
        if part2(TEST_INPUT_D, & mut deque_one(i)) == (vec![3, 9, 8, 9, 10, 9, 4, 9, 99, i_eq_8, 8] ,deque_one(i_eq_8)) {
            good += 1;
        }
        tested += 1;
        if part2(TEST_INPUT_E, & mut deque_one(i)) == (vec![3, 9, 7, 9, 10, 9, 4 ,9 , 99, i_lt_8, 8] ,deque_one(i_lt_8)) {
            good += 1;
        }
        tested += 1;
        if part2(TEST_INPUT_F, & mut deque_one(i)) == (vec![3, 3, 1108, i_eq_8, 8, 3, 4, 3, 99], deque_one(i_eq_8)) {
            good += 1;
        }
        tested += 1;
        if part2(TEST_INPUT_G, & mut deque_one(i)) == (vec![3, 3, 1107, i_lt_8, 8, 3, 4, 3, 99], deque_one(i_lt_8)) {
            good += 1;
        }
        tested += 1;
        if part2(TEST_INPUT_H, & mut deque_one(i)) == (vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, i, if i == 0 { 0 }  else { 1 }, 1, 9], deque_one(if i == 0 { i } else { 1 })) {
            good += 1;
        }
        tested += 1;
        if part2(TEST_INPUT_I, & mut deque_one(i)) == (vec![3, 3, 1105, i, 9, 1101, 0, 0, 12, 4, 12, 99, if i != 0 { 1 } else { 0 }], deque_one(if i == 0 { 0 } else { 1 })) {
            good += 1;
        }

        let test_j_out = if i < 8 { 999 } else if i == 8 { i * 125 } else { 1000 + 1 };

        tested += 1;
        if part2(TEST_INPUT_J, & mut deque_one(i)) == (vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, if i < 8 { 0 } else if i == 8 { i*125 } else { 1000 + 1 }, i, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99], deque_one(test_j_out)) {
            good += 1;
        }
    }
    println!("part 2 selftest 1 tested/good/bad: {0}/{1}/{2} => {3}", tested, good, tested - good, if tested == good { "good" } else { "OMG BADD BAD BAD!" });
    println!("part 2: {:?}", part2(INPUT, & mut deque_one(5)).1);
}
