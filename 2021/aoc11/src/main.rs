use std::collections::VecDeque;

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

fn neighbors(y: usize, x: usize) -> Vec<(usize, usize)> {
    // yield is still a experimental feature in Rust so we don't use it though this would be better
    // with yield
    let mut result: Vec<(usize, usize)> = Vec::new();

    for dy in -1_i32..=1_i32 {
        for dx in -1_i32..=1_i32 {
            if dy == -1 && y == 0 {
                continue;
            }
            if dx == -1 && x == 0 {
                continue;
            }
            if dy == 1 && y == 9 {
                continue;
            }
            if dx == 1 && x == 9 {
                continue;
            }
            if dy == 0 && dx == 0 {
                continue;
            }
            result.push((((y as i32) + dy) as usize, ((x as i32) + dx) as usize));
        }
    }

    return result;
}

fn parse_input(energy_level_strs: &str, energy_levels: &mut [[u32; 10]; 10]) {
    for (row, line) in energy_level_strs.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            energy_levels[row][col] = c.to_digit(10).unwrap();
        }
    }
}

fn step(energy_levels: &mut [[u32; 10]; 10]) -> u32 {
    let mut num_flashes = 0;
    let mut flashes: VecDeque<(usize, usize)> = VecDeque::new();

    for y in 0_usize..10_usize {
        for x in 0_usize..10_usize {
            let new = energy_levels[y][x] + 1;
            energy_levels[y][x] = new;

            if new == 10 {
                num_flashes += 1;
                flashes.push_back((y, x));
            }
        }
    }

    while let Some((y, x)) = flashes.pop_front() {
        for (ny, nx) in neighbors(y, x) {
            let new = energy_levels[ny][nx] + 1;
            energy_levels[ny][nx] = new;

            if new == 10 {
                num_flashes += 1;
                flashes.push_back((ny, nx));
            }
        }
    }

    for y in 0_usize..10_usize {
        for x in 0_usize..10_usize {
            if energy_levels[y][x] > 9 {
                energy_levels[y][x] = 0;
            }
        }
    }

    return num_flashes;
}

fn part_1(energy_level_strs: &str, steps: u32) -> u32 {
    let mut energy_levels: [[u32; 10]; 10] = [[0; 10]; 10];

    parse_input(energy_level_strs, &mut energy_levels);

    let mut num_flashes = 0;

    for _ in 0..steps {
        num_flashes += step(&mut energy_levels);
    }

    return num_flashes;
}

fn part_2(energy_level_strs: &str) -> u32 {
    let mut energy_levels: [[u32; 10]; 10] = [[0; 10]; 10];

    parse_input(energy_level_strs, &mut energy_levels);

    let mut steps = 0;
    loop {
        steps += 1;

        let num_flashes = step(&mut energy_levels);
        if num_flashes == 100 {
            return steps;
        }
    }
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT, 10), 204);
}

#[test]
fn test_b() {
    assert_eq!(part_1(TEST_INPUT, 100), 1656);
}

#[test]
fn test_c() {
    assert_eq!(part_2(TEST_INPUT), 195);
}

fn main() {
    println!("part 1: {0}", part_1(INPUT, 100));
    println!("part 2: {0}", part_2(INPUT));
    println!("Done.");
}
