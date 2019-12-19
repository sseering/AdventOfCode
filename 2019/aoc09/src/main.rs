// --- Day 9: Sensor Boost ---
//
// You've just said goodbye to the rebooted rover and left Mars when you receive a faint distress signal coming from the asteroid belt. It must be the Ceres monitoring station!
//
// In order to lock on to the signal, you'll need to boost your sensors. The Elves send up the latest BOOST program - Basic Operation Of System Test.
//
// While BOOST (your puzzle input) is capable of boosting your sensors, for tenuous safety reasons, it refuses to do so until the computer it runs on passes some checks to demonstrate it is a complete Intcode computer.
//
// Your existing Intcode computer is missing one key feature: it needs support for parameters in relative mode.
//
// Parameters in mode 2, relative mode, behave very similarly to parameters in position mode: the parameter is interpreted as a position. Like position mode, parameters in relative mode can be read from or written to.
//
// The important difference is that relative mode parameters don't count from address 0. Instead, they count from a value called the relative base. The relative base starts at 0.
//
// The address a relative mode parameter refers to is itself plus the current relative base. When the relative base is 0, relative mode parameters and position mode parameters with the same value refer to the same address.
//
// For example, given a relative base of 50, a relative mode parameter of -7 refers to memory address 50 + -7 = 43.
//
// The relative base is modified with the relative base offset instruction:
//
//     Opcode 9 adjusts the relative base by the value of its only parameter. The relative base increases (or decreases, if the value is negative) by the value of the parameter.
//
// For example, if the relative base is 2000, then after the instruction 109,19, the relative base would be 2019. If the next instruction were 204,-34, then the value at address 1985 would be output.
//
// Your Intcode computer will also need a few other capabilities:
//
//     The computer's available memory should be much larger than the initial program. Memory beyond the initial program starts with the value 0 and can be read or written like any other memory. (It is invalid to try to access memory at a negative address, though.)
//     The computer should have support for large numbers. Some instructions near the beginning of the BOOST program will verify this capability.
//
// Here are some example programs that use these features:
//
//     109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99 takes no input and produces a copy of itself as output.
//     1102,34915192,34915192,7,4,7,99,0 should output a 16-digit number.
//     104,1125899906842624,99 should output the large number in the middle.
//
// The BOOST program will ask for a single input; run it in test mode by providing it the value 1. It will perform a series of checks on each opcode, output any opcodes (and the associated parameter modes) that seem to be functioning incorrectly, and finally output a BOOST keycode.
//
// Once your Intcode computer is fully functional, the BOOST program should report no malfunctioning opcodes when run in test mode; it should only output a single value, the BOOST keycode. What BOOST keycode does it produce?
//
// To begin, get your puzzle input.
//
// --- Part Two ---
//
// You now have a complete Intcode computer.
//
// Finally, you can lock on to the Ceres distress signal! You just need to boost your sensors using the BOOST program.
//
// The program runs in sensor boost mode by providing the input instruction the value 2. Once run, it will boost the sensors automatically, but it might take a few seconds to complete the operation on slower hardware. In sensor boost mode, the program will output a single value: the coordinates of the distress signal.
//
// Run the BOOST program in sensor boost mode. What are the coordinates of the distress signal?
//
// Although it hasn't changed, you can still get your puzzle input.

extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
use std::collections::{HashMap, VecDeque};

#[allow(unused)]
const INPUT: & str = "1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1101,3,0,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1101,37,0,1005,1101,30,0,1013,1102,1,33,1019,1102,1,25,1003,1102,1,28,1018,1101,26,0,1006,1102,1,866,1029,1101,760,0,1023,1102,39,1,1012,1102,23,1,1009,1101,281,0,1026,1102,1,20,1011,1102,1,34,1008,1101,0,36,1017,1101,38,0,1000,1102,0,1,1020,1102,278,1,1027,1101,21,0,1010,1102,875,1,1028,1101,0,212,1025,1102,1,1,1021,1102,1,24,1014,1102,763,1,1022,1101,0,31,1007,1102,1,221,1024,1101,0,32,1002,1102,1,29,1004,1102,1,35,1016,1102,22,1,1015,1101,0,27,1001,109,9,1207,-6,26,63,1005,63,199,4,187,1105,1,203,1001,64,1,64,1002,64,2,64,109,19,2105,1,-4,4,209,1001,64,1,64,1106,0,221,1002,64,2,64,109,-33,1207,5,37,63,1005,63,241,1001,64,1,64,1106,0,243,4,227,1002,64,2,64,109,16,2102,1,-2,63,1008,63,23,63,1005,63,269,4,249,1001,64,1,64,1106,0,269,1002,64,2,64,109,16,2106,0,0,1106,0,287,4,275,1001,64,1,64,1002,64,2,64,109,-11,21101,40,0,0,1008,1016,38,63,1005,63,311,1001,64,1,64,1105,1,313,4,293,1002,64,2,64,109,4,21107,41,40,-9,1005,1011,329,1105,1,335,4,319,1001,64,1,64,1002,64,2,64,109,-14,21108,42,42,5,1005,1011,353,4,341,1106,0,357,1001,64,1,64,1002,64,2,64,109,2,2107,33,0,63,1005,63,379,4,363,1001,64,1,64,1105,1,379,1002,64,2,64,109,-7,1201,2,0,63,1008,63,25,63,1005,63,401,4,385,1105,1,405,1001,64,1,64,1002,64,2,64,109,11,1201,-8,0,63,1008,63,28,63,1005,63,429,1001,64,1,64,1106,0,431,4,411,1002,64,2,64,109,-7,2108,26,1,63,1005,63,449,4,437,1105,1,453,1001,64,1,64,1002,64,2,64,109,9,1206,7,465,1105,1,471,4,459,1001,64,1,64,1002,64,2,64,109,4,21102,43,1,-3,1008,1015,42,63,1005,63,491,1106,0,497,4,477,1001,64,1,64,1002,64,2,64,109,7,21108,44,43,-7,1005,1018,517,1001,64,1,64,1105,1,519,4,503,1002,64,2,64,109,-28,2101,0,7,63,1008,63,29,63,1005,63,545,4,525,1001,64,1,64,1105,1,545,1002,64,2,64,109,11,2107,28,-7,63,1005,63,561,1105,1,567,4,551,1001,64,1,64,1002,64,2,64,109,-4,2101,0,-1,63,1008,63,26,63,1005,63,587,1105,1,593,4,573,1001,64,1,64,1002,64,2,64,109,9,1206,7,607,4,599,1105,1,611,1001,64,1,64,1002,64,2,64,109,-10,1208,1,27,63,1005,63,627,1106,0,633,4,617,1001,64,1,64,1002,64,2,64,109,26,1205,-9,649,1001,64,1,64,1106,0,651,4,639,1002,64,2,64,109,-20,1208,0,23,63,1005,63,669,4,657,1105,1,673,1001,64,1,64,1002,64,2,64,109,-7,2102,1,1,63,1008,63,28,63,1005,63,693,1105,1,699,4,679,1001,64,1,64,1002,64,2,64,109,18,21102,45,1,-6,1008,1014,45,63,1005,63,725,4,705,1001,64,1,64,1106,0,725,1002,64,2,64,109,-23,1202,6,1,63,1008,63,25,63,1005,63,751,4,731,1001,64,1,64,1106,0,751,1002,64,2,64,109,20,2105,1,6,1106,0,769,4,757,1001,64,1,64,1002,64,2,64,109,-22,2108,39,10,63,1005,63,789,1001,64,1,64,1106,0,791,4,775,1002,64,2,64,109,3,1202,6,1,63,1008,63,32,63,1005,63,815,1001,64,1,64,1105,1,817,4,797,1002,64,2,64,109,23,21107,46,47,-9,1005,1012,835,4,823,1106,0,839,1001,64,1,64,1002,64,2,64,109,1,1205,-1,853,4,845,1105,1,857,1001,64,1,64,1002,64,2,64,109,-2,2106,0,8,4,863,1001,64,1,64,1105,1,875,1002,64,2,64,109,-8,21101,47,0,-2,1008,1010,47,63,1005,63,897,4,881,1106,0,901,1001,64,1,64,4,64,99,21102,27,1,1,21101,0,915,0,1105,1,922,21201,1,27810,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21102,1,942,0,1106,0,922,22101,0,1,-1,21201,-2,-3,1,21101,957,0,0,1106,0,922,22201,1,-1,-2,1106,0,968,22101,0,-2,-2,109,-3,2106,0,0";

#[allow(unused)]
const TEST_QUINE: &str = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";

#[allow(unused)]
const TEST_16DIGIT_NUM: &str = "1102,34915192,34915192,7,4,7,99,0";

#[allow(unused)]
const TEST_BIG_NUM: &str = "104,1125899906842624,99";

#[allow(unused)]
const TEST_INPUT_A: &str = "3,0,4,0,99";

#[allow(unused)]
const TEST_INPUT_B: &str = "1002,4,3,4,33";

#[allow(unused)]
const TEST_INPUT_C: &str = "1101,100,-1,4,0";

#[allow(unused)]
const TEST_INPUT_D: &str = "3,9,8,9,10,9,4,9,99,-1,8";

#[allow(unused)]
const TEST_INPUT_E: &str = "3,9,7,9,10,9,4,9,99,-1,8";

#[allow(unused)]
const TEST_INPUT_F: &str = "3,3,1108,-1,8,3,4,3,99";

#[allow(unused)]
const TEST_INPUT_G: &str = "3,3,1107,-1,8,3,4,3,99";

#[allow(unused)]
const TEST_INPUT_H: &str = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";

#[allow(unused)]
const TEST_INPUT_I: &str = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";

#[allow(unused)]
const TEST_INPUT_J: & str = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

fn parse(input: &str) -> Vec<BigInt> {
    input
        .split(",")
        .map(|s| -> BigInt { s.parse().unwrap() })
        .collect()
}

#[derive(Debug)]
struct SystemState {
    base_mem: Vec<BigInt>,
    base_mem_size: BigInt,
    extra_mem: HashMap<BigInt, BigInt>,
    relaive_base: BigInt,
    output: VecDeque<BigInt>,
}

impl SystemState {
    fn new(prog: Vec<BigInt>) -> SystemState {
        let size = BigInt::from(prog.len());
        SystemState {
            base_mem: prog,
            base_mem_size: size,
            extra_mem: HashMap::new(),
            relaive_base: BigInt::from(0),
            output: VecDeque::new(),
        }
    }

    #[allow(unused)]
    fn new_test(
        base_mem: Vec<BigInt>,
        extra_mem: Option<HashMap<BigInt, BigInt>>,
        relaive_base: Option<BigInt>,
        output: Option<VecDeque<BigInt>>,
    ) -> SystemState {
        let base_mem_size = BigInt::from(base_mem.len());
        let extra_mem = match extra_mem {
            Some(em) => em,
            None => HashMap::new(),
        };
        let relaive_base = match relaive_base {
            Some(rb) => rb,
            None => BigInt::from(0),
        };
        let output = match output {
            Some(o) => o,
            None => VecDeque::new(),
        };
        SystemState {
            base_mem,
            base_mem_size,
            extra_mem,
            relaive_base,
            output,
        }
    }

    fn get_mem_val(&mut self, addr: BigInt) -> &BigInt {
        if addr < self.base_mem_size {
            let addr = addr.to_usize().unwrap();
            return &self.base_mem[addr];
        }
        return self.extra_mem.entry(addr).or_insert(BigInt::from(0));
    }

    fn set_mem_val(&mut self, addr: BigInt, val: BigInt) {
        if addr < self.base_mem_size {
            let addr = addr.to_usize().unwrap();
            self.base_mem[addr] = val;
        } else {
            self.extra_mem.insert(addr, val);
        }
    }

    fn extra_mem_eq(a: &SystemState, b: &SystemState) -> bool {
        let a = &a.extra_mem;
        let b = &b.extra_mem;
        let z = BigInt::from(0);
        for (k, v) in a {
            if *v == z {
                continue;
            }
            match b.get(k) {
                Some(ov) => {
                    if v != ov {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
        return true;
    }
}

impl PartialEq for SystemState {
    fn eq(&self, other: &SystemState) -> bool {
        self.base_mem == other.base_mem
            && SystemState::extra_mem_eq(self, other)
            && SystemState::extra_mem_eq(other, self)
            && self.relaive_base == other.relaive_base
            && self.output == other.output
    }
}

impl Eq for SystemState {}

#[derive(PartialEq, Eq)]
enum Op {
    Calculation(
        fn(BigInt, BigInt) -> BigInt,
        ParamMode,
        ParamMode,
        ParamMode,
    ),
    Input(ParamMode),
    Output(ParamMode),
    ContitionalJump(fn(BigInt) -> bool, ParamMode, ParamMode),
    RelativeBaseAdjust(ParamMode),
    End,
}

impl Op {
    fn new(opcode: &BigInt) -> Op {
        let opcode = opcode.to_i32().unwrap();
        const ADD: i32 = 1;
        const MUL: i32 = 2;
        const INP: i32 = 3;
        const OUT: i32 = 4;
        const JMPT: i32 = 5;
        const JMPF: i32 = 6;
        const LT: i32 = 7;
        const EQ: i32 = 8;
        const BADJ: i32 = 9;
        const END: i32 = 99;

        let low_opcode = opcode % 100;
        let opcode = opcode / 100;

        let mut param_modes: Vec<ParamMode> = Vec::new();
        if low_opcode == ADD || low_opcode == MUL || low_opcode == LT || low_opcode == EQ {
            let p1 = ParamMode::from_int(opcode % 10);

            let opcode = opcode / 10;
            let p2 = ParamMode::from_int(opcode % 10);

            let opcode = opcode / 10;
            let p3 = ParamMode::from_int(opcode % 10);

            if p3 == ParamMode::Immediate {
                panic!();
            }

            param_modes.push(p1);
            param_modes.push(p2);
            param_modes.push(p3);
        } else if low_opcode == INP {
            let p1 = ParamMode::from_int(opcode % 10);
            if p1 == ParamMode::Immediate {
                panic!();
            }

            param_modes.push(p1);
        } else if low_opcode == OUT || low_opcode == BADJ {
            let p1 = ParamMode::from_int(opcode % 10);

            param_modes.push(p1);
        } else if low_opcode == JMPT || low_opcode == JMPF {
            let p1 = ParamMode::from_int(opcode % 10);

            let opcode = opcode / 10;
            let p2 = ParamMode::from_int(opcode % 10);

            param_modes.push(p1);
            param_modes.push(p2);
        };

        return match low_opcode {
            ADD => Op::Calculation(
                |a, b| return a + b,
                param_modes[0],
                param_modes[1],
                param_modes[2],
            ),
            MUL => Op::Calculation(
                |a, b| return a * b,
                param_modes[0],
                param_modes[1],
                param_modes[2],
            ),
            INP => Op::Input(param_modes[0]),
            OUT => Op::Output(param_modes[0]),
            JMPT => Op::ContitionalJump(
                |a| return a != BigInt::from(0),
                param_modes[0],
                param_modes[1],
            ),
            JMPF => Op::ContitionalJump(
                |a| return a == BigInt::from(0),
                param_modes[0],
                param_modes[1],
            ),
            LT => Op::Calculation(
                |a, b| {
                    return BigInt::from(if a < b { 1 } else { 0 });
                },
                param_modes[0],
                param_modes[1],
                param_modes[2],
            ),
            EQ => Op::Calculation(
                |a, b| {
                    return BigInt::from(if a == b { 1 } else { 0 });
                },
                param_modes[0],
                param_modes[1],
                param_modes[2],
            ),
            BADJ => Op::RelativeBaseAdjust(param_modes[0]),

            END => Op::End,
            _ => panic!("{}", low_opcode),
        };
    }

    fn operate(
        &self,
        sys: &mut SystemState,
        op_idx: BigInt,
        input: &mut VecDeque<BigInt>,
    ) -> BigInt {
        match self {
            Op::End => {
                panic!();
            }
            Op::Input(pm) => {
                let addr = pm.get_writable_addr(sys, &op_idx + 1);
                sys.set_mem_val(addr, input.pop_front().unwrap());
                return op_idx + 2;
            }
            Op::Output(pm) => {
                let out_val = pm.get_param(sys, &op_idx + 1);
                sys.output.push_back(out_val);
                return op_idx + 2;
            }
            Op::Calculation(fun, pm1, pm2, pm3) => {
                let val_a = pm1.get_param(sys, &op_idx + 1);
                let val_b = pm2.get_param(sys, &op_idx + 2);
                let addr = pm3.get_writable_addr(sys, &op_idx + 3);
                sys.set_mem_val(addr, fun(val_a, val_b));
                return op_idx + 4;
            }
            Op::ContitionalJump(cond, pm1, pm2) => {
                let cond_val = pm1.get_param(sys, &op_idx + 1);
                let jump_dst = pm2.get_param(sys, &op_idx + 2);
                return if cond(cond_val) {
                    jump_dst
                } else {
                    &op_idx + 3
                };
            }
            Op::RelativeBaseAdjust(pm1) => {
                let adj_val = pm1.get_param(sys, &op_idx + 1);
                sys.relaive_base += adj_val;
                return op_idx + 2;
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum ParamMode {
    Relative,
    Immediate,
    Position,
}

impl ParamMode {
    fn from_int(i: i32) -> ParamMode {
        match i {
            2 => ParamMode::Relative,
            1 => ParamMode::Immediate,
            0 => ParamMode::Position,
            _ => panic!(),
        }
    }

    fn get_writable_addr(&self, sys_state: &mut SystemState, addr: BigInt) -> BigInt {
        let addr = sys_state.get_mem_val(addr).clone();
        if *self == ParamMode::Relative {
            addr + &sys_state.relaive_base
        } else {
            addr
        }
    }

    fn get_param(&self, sys_state: &mut SystemState, addr: BigInt) -> BigInt {
        return (match self {
            ParamMode::Immediate => sys_state.get_mem_val(addr),
            ParamMode::Position => {
                let addr = sys_state.get_mem_val(addr).clone();
                sys_state.get_mem_val(addr)
            }
            ParamMode::Relative => {
                let addr = sys_state.get_mem_val(addr).clone();
                let addr = addr + &sys_state.relaive_base;
                sys_state.get_mem_val(addr)
            }
        })
        .clone();
    }
}

fn part1(puzzle_input: &str, input: Option<&mut VecDeque<BigInt>>) -> SystemState {
    let mut empty_input: VecDeque<BigInt> = VecDeque::new();
    let input = match input {
        Some(i) => i,
        None => &mut empty_input,
    };
    let mut sys = SystemState::new(parse(puzzle_input));
    let mut ip = BigInt::from(0);

    loop {
        // println!("Op::new on [ip]: {}", ip);
        let op = Op::new(sys.get_mem_val(ip.clone()));

        if op == Op::End {
            break;
        }

        let new_ip = op.operate(&mut sys, ip, input);
        ip = new_ip;
    }

    return sys;
}

fn deque_one(i: i32) -> VecDeque<BigInt> {
    // TODO make a macro?
    let mut res: VecDeque<BigInt> = VecDeque::new();
    res.push_back(BigInt::from(i));
    return res;
}

#[allow(unused)]
fn part2(puzzle_input: &str, input: Option<&mut VecDeque<BigInt>>) -> SystemState {
    return part1(puzzle_input, input);
}

trait MyToBigInt {
    fn my_to_bigint(self) -> BigInt;
}

impl MyToBigInt for i32 {
    fn my_to_bigint(self) -> BigInt {
        BigInt::from(self)
    }
}

impl MyToBigInt for &'static str {
    fn my_to_bigint(self) -> BigInt {
        self.parse().unwrap()
    }
}

#[allow(unused)]
macro_rules! big_int_vec {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_vec: Vec<BigInt> = Vec::new();
            $(
                temp_vec.push(MyToBigInt::my_to_bigint($x));
            )*
            temp_vec
        }
    };
}

#[test]
fn test_input_a() {
    for i in 0..20 {
        assert_eq!(
            part1(TEST_INPUT_A, Some(&mut deque_one(i))),
            SystemState::new_test(big_int_vec![i, 0, 4, 0, 99], None, None, Some(deque_one(i)))
        );
    }
}

#[test]
fn test_input_b() {
    assert_eq!(
        part1(TEST_INPUT_B, None),
        SystemState::new_test(big_int_vec![1002, 4, 3, 4, 99], None, None, None)
    );
}

#[test]
fn test_input_c() {
    assert_eq!(
        part1(TEST_INPUT_C, None),
        SystemState::new_test(big_int_vec![1101, 100, -1, 4, 99], None, None, None)
    );
}

#[test]
fn test_input_d() {
    for i in 0..20 {
        let i_eq_8 = if i == 8 { 1 } else { 0 };
        assert_eq!(
            part2(TEST_INPUT_D, Some(&mut deque_one(i))),
            SystemState::new_test(
                big_int_vec![3, 9, 8, 9, 10, 9, 4, 9, 99, i_eq_8, 8],
                None,
                None,
                Some(deque_one(i_eq_8)),
            )
        );
    }
}

#[test]
fn test_input_e() {
    for i in 0..20 {
        let i_lt_8 = if i < 8 { 1 } else { 0 };
        assert_eq!(
            part2(TEST_INPUT_E, Some(&mut deque_one(i))),
            SystemState::new_test(
                big_int_vec![3, 9, 7, 9, 10, 9, 4, 9, 99, i_lt_8, 8],
                None,
                None,
                Some(deque_one(i_lt_8)),
            )
        );
    }
}

#[test]
fn test_input_f() {
    for i in 0..20 {
        let i_eq_8 = if i == 8 { 1 } else { 0 };
        assert_eq!(
            part2(TEST_INPUT_F, Some(&mut deque_one(i))),
            SystemState::new_test(
                big_int_vec![3, 3, 1108, i_eq_8, 8, 3, 4, 3, 99],
                None,
                None,
                Some(deque_one(i_eq_8)),
            )
        );
    }
}

#[test]
fn test_input_g() {
    for i in 0..20 {
        let i_lt_8 = if i < 8 { 1 } else { 0 };
        assert_eq!(
            part2(TEST_INPUT_G, Some(&mut deque_one(i))),
            SystemState::new_test(
                big_int_vec![3, 3, 1107, i_lt_8, 8, 3, 4, 3, 99],
                None,
                None,
                Some(deque_one(i_lt_8)),
            )
        );
    }
}

#[test]
fn test_input_h() {
    for i in 0..20 {
        assert_eq!(
            part2(TEST_INPUT_H, Some(&mut deque_one(i))),
            SystemState::new_test(
                big_int_vec![
                    3,
                    12,
                    6,
                    12,
                    15,
                    1,
                    13,
                    14,
                    13,
                    4,
                    13,
                    99,
                    i,
                    if i == 0 { 0 } else { 1 },
                    1,
                    9
                ],
                None,
                None,
                Some(deque_one(if i == 0 { i } else { 1 })),
            )
        );
    }
}

#[test]
fn test_input_i() {
    for i in 0..20 {
        assert_eq!(
            part2(TEST_INPUT_I, Some(&mut deque_one(i))),
            SystemState::new_test(
                big_int_vec![
                    3,
                    3,
                    1105,
                    i,
                    9,
                    1101,
                    0,
                    0,
                    12,
                    4,
                    12,
                    99,
                    if i != 0 { 1 } else { 0 }
                ],
                None,
                None,
                Some(deque_one(if i == 0 { 0 } else { 1 })),
            )
        );
    }
}

#[test]
fn test_input_j() {
    for i in 0..20 {
        let test_j_out = if i < 8 {
            999
        } else if i == 8 {
            i * 125
        } else {
            1000 + 1
        };
        let test_j_mem = if i < 8 {
            0
        } else if i == 8 {
            i * 125
        } else {
            1000 + 1
        };
        assert_eq!(
            part2(TEST_INPUT_J, Some(&mut deque_one(i))),
            SystemState::new_test(
                big_int_vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, test_j_mem, i, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105,
                    1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                None,
                None,
                Some(deque_one(test_j_out)),
            )
        );
    }
}

#[test]
fn test_quine() {
    let mut expected_extra_mem: HashMap<BigInt, BigInt> = HashMap::new();
    expected_extra_mem.insert(BigInt::from(100), BigInt::from(16));
    expected_extra_mem.insert(BigInt::from(101), BigInt::from(1));
    assert_eq!(
        part1(TEST_QUINE, None),
        SystemState::new_test(
            big_int_vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
            Some(expected_extra_mem),
            Some(BigInt::from(16)),
            Some(VecDeque::from(big_int_vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99
            ])),
        )
    );
}

#[test]
fn test_16digit_num() {
    let mut expected_out: VecDeque<BigInt> = VecDeque::new();
    expected_out.push_back("1219070632396864".parse().unwrap());
    assert_eq!(
        part1(TEST_16DIGIT_NUM, None),
        SystemState::new_test(
            big_int_vec![1102, 34915192, 34915192, 7, 4, 7, 99, "1219070632396864"],
            None,
            None,
            Some(expected_out),
        )
    );
}

#[test]
fn test_big_num() {
    let mut expected_out: VecDeque<BigInt> = VecDeque::new();
    expected_out.push_back("1125899906842624".parse().unwrap());
    assert_eq!(
        part1(TEST_BIG_NUM, None),
        SystemState::new_test(
            big_int_vec![104, "1125899906842624", 99],
            None,
            None,
            Some(expected_out),
        )
    );
}

fn main() {
    println!("Part 1: {:?}", part1(INPUT, Some(&mut deque_one(1))).output);
    println!("Part 2: {:?}", part2(INPUT, Some(&mut deque_one(2))).output);
}
