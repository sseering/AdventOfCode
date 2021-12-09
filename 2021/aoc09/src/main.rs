use std::cmp::Ordering;
use std::collections::VecDeque;

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

fn parse_input(heightmap_str: &str) -> (Vec<Vec<u32>>, i32, i32) {
    let mut heightmap: Vec<Vec<u32>> = Vec::new();
    let mut width: i32 = 0;
    for line in heightmap_str.lines() {
        let height_line: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        width = height_line.len() as i32;
        heightmap.push(height_line);
    }
    let height: i32 = heightmap.len() as i32;

    return (heightmap, width, height);
}

#[derive(Eq)]
struct LowPoint {
    x: i32,
    y: i32,
    risk_level: u32,
}

impl LowPoint {
    fn new(x: i32, y: i32, risk_level: u32) -> Self {
        LowPoint { x, y, risk_level }
    }
}

impl Ord for LowPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.risk_level.cmp(&other.risk_level);
    }
}

impl PartialOrd for LowPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.risk_level.cmp(&other.risk_level));
    }
}

impl PartialEq for LowPoint {
    fn eq(&self, other: &Self) -> bool {
        return self.risk_level == other.risk_level;
    }
}

fn low_points(heightmap: &Vec<Vec<u32>>, width: i32, height: i32) -> Vec<LowPoint> {
    let mut result: Vec<LowPoint> = Vec::new();

    let get = |x: i32, y: i32| -> u32 {
        if x < 0 || y < 0 || x >= width || y >= height {
            return u32::MAX;
        }
        return heightmap[y as usize][x as usize];
    };

    for y in 0..height {
        for x in 0..width {
            let local = get(x, y);
            if local < get(x - 1, y)
                && local < get(x + 1, y)
                && local < get(x, y - 1)
                && local < get(x, y + 1)
            {
                result.push(LowPoint::new(x, y, local + 1));
            }
        }
    }

    return result;
}

fn part_1(heightmap_str: &str) -> u32 {
    let (heightmap, width, height) = parse_input(heightmap_str);

    let lps = low_points(&heightmap, width, height);

    return lps.iter().map(|lp| lp.risk_level).sum();
}

fn part_2(heightmap_str: &str) -> u32 {
    let (heightmap, width, height) = parse_input(heightmap_str);

    let mut lps = low_points(&heightmap, width, height);

    lps.sort_unstable();

    let mut done: Vec<Vec<bool>> = Vec::new();
    for _ in 0..height {
        done.push(vec![false; width as usize]);
    }

    let add_to_process =
        |add_here: &mut VecDeque<(usize, usize)>, done: &Vec<Vec<bool>>, x: usize, y: usize| {
            if x > 0 && !done[y][x - 1] && heightmap[y][x - 1] < 9 {
                add_here.push_back((x - 1, y));
            }
            if y > 0 && !done[y - 1][x] && heightmap[y - 1][x] < 9 {
                add_here.push_back((x, y - 1));
            }
            if x < (width - 1) as usize && !done[y][x + 1] && heightmap[y][x + 1] < 9 {
                add_here.push_back((x + 1, y));
            }
            if y < (height - 1) as usize && !done[y + 1][x] && heightmap[y + 1][x] < 9 {
                add_here.push_back((x, y + 1));
            }
        };

    let mut basin_sizes: Vec<u32> = Vec::new();
    for lp in &lps {
        let mut basin_size: u32 = 1;
        done[lp.y as usize][lp.x as usize] = true;
        let mut to_process: VecDeque<(usize, usize)> = VecDeque::new();
        add_to_process(&mut to_process, &done, lp.x as usize, lp.y as usize);

        while let Some((x, y)) = to_process.pop_front() {
            if done[y][x] {
                continue;
            }
            basin_size += 1;
            done[y][x] = true;
            add_to_process(&mut to_process, &done, x, y);
        }

        basin_sizes.push(basin_size);
    }

    basin_sizes.sort_by(|a, b| b.cmp(a));

    return basin_sizes[0] * basin_sizes[1] * basin_sizes[2];
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), 15);
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), 1134);
}

fn main() {
    println!("part 1: {0}", part_1(INPUT));
    println!("part 2: {0}", part_2(INPUT));
    println!("Done.");
}
