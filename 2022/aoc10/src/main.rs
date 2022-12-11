use std::str::FromStr;

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT_A: &str = include_str!("../test-input-a.txt");

#[allow(unused)]
const TEST_INPUT_B: &str = include_str!("../test-input-b.txt");

enum Instr {
    Noop,
    Addx(i32),
}

impl Instr {
    fn duration(&self) -> i32 {
        match self {
            Instr::Noop => 1,
            Instr::Addx(_) => 2,
        }
    }

    fn reg_change(&self) -> i32 {
        match self {
            Instr::Noop => 0,
            Instr::Addx(x) => *x,
        }
    }
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input == "noop" {
            return Ok(Instr::Noop);
        }
        let mut split = input.split_whitespace();
        let a = split.next().ok_or(())?;
        let b = split.next().ok_or(())?;
        if a != "addx" {
            return Err(());
        }
        let num: i32 = b.parse().map_err(|_| ())?;
        return Ok(Self::Addx(num));
    }
}

fn parse_instructions(instructions: &str) -> Option<Vec<Instr>> {
    return instructions
        .lines()
        .map(|line| {
            return Instr::from_str(line).ok();
        })
        .collect::<Option<Vec<Instr>>>();
}

fn part_1(instructions: &str, measured_cycles: Vec<i32>) -> Option<i32> {
    let instructions = parse_instructions(instructions)?;

    let instructions_len = instructions.len();
    let measured_cycles_len = measured_cycles.len();
    let mut signal_strength: i32 = 0;
    let mut reg_x: i32 = 1;
    let mut cycle_begun: i32 = 1;
    let mut next_measurement_idx: usize = 0;
    let mut instruction_pointer: usize = 0;

    while next_measurement_idx < measured_cycles_len {
        let instr: &Instr = if instruction_pointer < instructions_len {
            &instructions[instruction_pointer]
        } else {
            &Instr::Noop
        };
        instruction_pointer += 1;

        let cycle_after = cycle_begun + instr.duration();

        while next_measurement_idx < measured_cycles_len {
            let cycle_to_measure = measured_cycles[next_measurement_idx];
            if cycle_to_measure >= cycle_begun && cycle_to_measure < cycle_after {
                signal_strength += cycle_to_measure * reg_x;
                next_measurement_idx += 1;
            } else {
                break;
            }
        }

        reg_x += instr.reg_change();

        cycle_begun = cycle_after;
    }

    return Some(signal_strength);
}

const DARK: bool = false;
const LIGHT: bool = true;
const COLS: usize = 40;
const ROWS: usize = 6;

fn part_2(instructions: &str) -> Option<[[bool; COLS]; ROWS]> {
    let instructions = parse_instructions(instructions)?;

    let mut result = [[DARK; COLS]; ROWS];
    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut reg_x: i32 = 1;

    for instr in instructions {
        for _ in 0..instr.duration() {
            let from = reg_x - 1;
            let from: usize = if from >= 0 { from as usize } else { 0 };
            let to = reg_x + 2;
            let to: usize = if to >= 0 { to as usize } else { 0 };

            if col >= from && col < to {
                result[row][col] = LIGHT;
            }

            col += 1;
            if col >= COLS {
                col = 0;
                row += 1;
                if row >= ROWS {
                    row = 0;
                }
            }
        }

        reg_x += instr.reg_change();
    }

    return Some(result);
}

fn crt(pixels: &[[bool; COLS]; ROWS]) {
    for row in pixels {
        for &pixel in row {
            print!("{}", if pixel { '#' } else { ' ' });
        }
        println!("");
    }
}

#[test]
fn test_aa() {
    let cycle: i32 = 1;
    assert_eq!(part_1(TEST_INPUT_A, vec![cycle]), Some(cycle * 1));
}

#[test]
fn test_ab() {
    let cycle: i32 = 2;
    assert_eq!(part_1(TEST_INPUT_A, vec![cycle]), Some(cycle * 1));
}

#[test]
fn test_ac() {
    let cycle: i32 = 3;
    assert_eq!(part_1(TEST_INPUT_A, vec![cycle]), Some(cycle * 1));
}

#[test]
fn test_ad() {
    let cycle: i32 = 4;
    assert_eq!(part_1(TEST_INPUT_A, vec![cycle]), Some(cycle * 4));
}

#[test]
fn test_ae() {
    let cycle: i32 = 5;
    assert_eq!(part_1(TEST_INPUT_A, vec![cycle]), Some(cycle * 4));
}

#[test]
fn test_af() {
    let cycle: i32 = 6;
    assert_eq!(part_1(TEST_INPUT_A, vec![cycle]), Some(cycle * -1));
}

#[test]
fn test_ba() {
    let cycle: i32 = 20;
    assert_eq!(part_1(TEST_INPUT_B, vec![cycle]), Some(cycle * 21));
}

#[test]
fn test_bb() {
    let cycle: i32 = 60;
    assert_eq!(part_1(TEST_INPUT_B, vec![cycle]), Some(cycle * 19));
}

#[test]
fn test_bc() {
    let cycle: i32 = 100;
    assert_eq!(part_1(TEST_INPUT_B, vec![cycle]), Some(cycle * 18));
}

#[test]
fn test_bd() {
    let cycle: i32 = 140;
    assert_eq!(part_1(TEST_INPUT_B, vec![cycle]), Some(cycle * 21));
}

#[test]
fn test_be() {
    let cycle: i32 = 180;
    assert_eq!(part_1(TEST_INPUT_B, vec![cycle]), Some(cycle * 16));
}

#[test]
fn test_bf() {
    let cycle: i32 = 220;
    assert_eq!(part_1(TEST_INPUT_B, vec![cycle]), Some(cycle * 18));
}

#[test]
fn test_c() {
    let expected = [
        [
            LIGHT, LIGHT, DARK, DARK, LIGHT, LIGHT, DARK, DARK, LIGHT, LIGHT, DARK, DARK, LIGHT,
            LIGHT, DARK, DARK, LIGHT, LIGHT, DARK, DARK, LIGHT, LIGHT, DARK, DARK, LIGHT, LIGHT,
            DARK, DARK, LIGHT, LIGHT, DARK, DARK, LIGHT, LIGHT, DARK, DARK, LIGHT, LIGHT, DARK,
            DARK,
        ],
        [
            LIGHT, LIGHT, LIGHT, DARK, DARK, DARK, LIGHT, LIGHT, LIGHT, DARK, DARK, DARK, LIGHT,
            LIGHT, LIGHT, DARK, DARK, DARK, LIGHT, LIGHT, LIGHT, DARK, DARK, DARK, LIGHT, LIGHT,
            LIGHT, DARK, DARK, DARK, LIGHT, LIGHT, LIGHT, DARK, DARK, DARK, LIGHT, LIGHT, LIGHT,
            DARK,
        ],
        [
            LIGHT, LIGHT, LIGHT, LIGHT, DARK, DARK, DARK, DARK, LIGHT, LIGHT, LIGHT, LIGHT, DARK,
            DARK, DARK, DARK, LIGHT, LIGHT, LIGHT, LIGHT, DARK, DARK, DARK, DARK, LIGHT, LIGHT,
            LIGHT, LIGHT, DARK, DARK, DARK, DARK, LIGHT, LIGHT, LIGHT, LIGHT, DARK, DARK, DARK,
            DARK,
        ],
        [
            LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, DARK, DARK, DARK, DARK, DARK, LIGHT, LIGHT, LIGHT,
            LIGHT, LIGHT, DARK, DARK, DARK, DARK, DARK, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, DARK,
            DARK, DARK, DARK, DARK, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, DARK, DARK, DARK, DARK,
            DARK,
        ],
        [
            LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, DARK, DARK, DARK, DARK, DARK, DARK, LIGHT,
            LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, DARK, DARK, DARK, DARK, DARK, DARK, LIGHT, LIGHT,
            LIGHT, LIGHT, LIGHT, LIGHT, DARK, DARK, DARK, DARK, DARK, DARK, LIGHT, LIGHT, LIGHT,
            LIGHT,
        ],
        [
            LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, DARK, DARK, DARK, DARK, DARK, DARK,
            DARK, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, DARK, DARK, DARK, DARK, DARK,
            DARK, DARK, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, DARK, DARK, DARK, DARK,
            DARK,
        ],
    ];
    assert_eq!(part_2(TEST_INPUT_B), Some(expected));
}

fn main() {
    println!(
        "part 1: {}",
        part_1(INPUT, vec![20, 60, 100, 140, 180, 220]).unwrap_or(999999)
    );
    crt(&part_2(INPUT).unwrap());
    println!("done.");
}
