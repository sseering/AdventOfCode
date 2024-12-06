use std::collections::HashSet;
use std::ops::{Add, Index};

#[allow(unused)]
const TEST_INPUT: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord2D {
    row: i32,
    col: i32,
}

impl Coord2D {
    fn rotate_right(&mut self) {
        let r = self.col;
        let c = self.row * -1;
        self.row = r;
        self.col = c;
    }
}

impl Add<&Coord2D> for &Coord2D {
    type Output = Coord2D;

    fn add(self, other: &Coord2D) -> Coord2D {
        Coord2D {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

struct Laboratory {
    f: bool,
    width: usize,
    height: usize,
    positions: Vec<Vec<bool>>,
}

impl Index<&Coord2D> for Laboratory {
    type Output = bool;

    fn index(&self, coord: &Coord2D) -> &Self::Output {
        if coord.row < 0 || coord.col < 0 {
            return &self.f;
        }
        let r = coord.row as usize;
        let c = coord.col as usize;
        if r >= self.height || c >= self.width {
            return &self.f;
        }
        return &self.positions[r][c];
    }
}

impl Laboratory {
    fn contains(&self, c: &Coord2D) -> bool {
        return c.row >= 0
            && c.col >= 0
            && (c.row as usize) < self.height
            && (c.col as usize) < self.width;
    }
}

fn parse_map_str(map: &str) -> Option<(Laboratory, Coord2D, Coord2D)> {
    let mut positions: Vec<Vec<bool>> = Vec::new();

    let mut width: usize = 0;
    let mut pos = Coord2D { row: -1, col: -1 };
    let mut direction = Coord2D { row: -1, col: -1 };
    for (row, line) in map.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        if width == 0 {
            width = line.len();
        } else {
            if width != line.len() {
                eprintln!("width contradiction");
                return None;
            }
        }

        let mut v: Vec<bool> = Vec::with_capacity(width);
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    v.push(true);
                }
                '.' => {
                    v.push(false);
                }
                '^' => {
                    if pos.col != -1 {
                        eprintln!("pos up contradiction");
                        return None;
                    }
                    pos = Coord2D {
                        row: row as i32,
                        col: col as i32,
                    };
                    direction = Coord2D { row: -1, col: 0 };
                    v.push(false);
                }
                '<' => {
                    if pos.col != -1 {
                        eprintln!("pos left contradiction");
                        return None;
                    }
                    pos = Coord2D {
                        row: row as i32,
                        col: col as i32,
                    };
                    direction = Coord2D { row: 0, col: -1 };
                    v.push(false);
                }
                '>' => {
                    if pos.col != -1 {
                        eprintln!("pos right contradiction");
                        return None;
                    }
                    pos = Coord2D {
                        row: row as i32,
                        col: col as i32,
                    };
                    direction = Coord2D { row: 0, col: 1 };
                    v.push(false);
                }
                'v' => {
                    if pos.col != -1 {
                        eprintln!("pos right contradiction");
                        return None;
                    }
                    pos = Coord2D {
                        row: row as i32,
                        col: col as i32,
                    };
                    direction = Coord2D { row: 1, col: 0 };
                    v.push(false);
                }
                x => {
                    eprintln!("unknown char |{0}|", x);
                    return None;
                }
            }
        }
        if v.len() != width {
            eprintln!("later width contradiction {0} {1}", v.len(), width);
            return None;
        }
        positions.push(v);
    }

    return Some((
        Laboratory {
            f: false,
            width,
            height: positions.len(),
            positions,
        },
        pos,
        direction,
    ));
}

fn part_1_simple(map: &str) -> Option<usize> {
    let (lab, mut pos, mut direction) = parse_map_str(map)?;
    let mut stepped_on: HashSet<Coord2D> = HashSet::new();

    while lab.contains(&pos) {
        stepped_on.insert(pos.clone());

        let mut next_pos = &pos + &direction;
        let mut infinite_loop_check = 0;
        while lab[&next_pos] {
            infinite_loop_check += 1;
            if infinite_loop_check > 8 {
                eprintln!("infinite loop");
                return None;
            }
            direction.rotate_right();
            next_pos = &pos + &direction
        }

        pos = next_pos;
    }

    return Some(stepped_on.len());
}

fn part_1(map: &str) -> Option<usize> {
    None
}

fn part_2(map: &str) -> Option<usize> {
    None
}

#[test]
fn test_a() {
    assert_eq!(part_1_simple(TEST_INPUT), Some(41));
}

#[test]
fn test_b() {
    assert_eq!(part_1(TEST_INPUT), Some(41));
}

fn main() {
    match part_1_simple(INPUT) {
        Some(cv) => {
            println!("Part 1 simple: {0}.", cv);
        }
        None => {
            println!("Part 1 simple failed.");
        }
    }
    match part_1(INPUT) {
        Some(cv) => {
            println!("Part 1: {0}.", cv);
        }
        None => {
            println!("Part 1 failed.");
        }
    }
    match part_2(INPUT) {
        Some(cv) => {
            println!("Part 2: {0}.", cv);
        }
        None => {
            println!("Part 2 failed.");
        }
    }
    println!("Done.");
}
