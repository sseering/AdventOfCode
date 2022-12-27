use std::cmp::{max, min, Ordering};
// use std::collections::HashMap;
use std::ops::{Add, Index, IndexMut};

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point2D {
    x: usize,
    y: usize,
}

impl Add<Point2D> for Point2D {
    type Output = Point2D;

    fn add(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&Point2D> for &Point2D {
    type Output = Point2D;

    fn add(self, other: &Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

type CaveWalls = Vec<Vec<Point2D>>;

struct ParseResult {
    cave_walls: CaveWalls,
    min_x: usize,
    max_x: usize,
    // min_y is always 0
    max_y: usize,
}

impl ParseResult {
    fn new(cave_walls: CaveWalls, min_x: usize, max_x: usize, max_y: usize) -> Self {
        Self {
            cave_walls,
            min_x,
            max_x,
            max_y,
        }
    }
}

fn parse(cave_scan: &str) -> Option<ParseResult> {
    let mut min_x: usize = 500;
    let mut max_x: usize = 500;
    // min_y is always 0
    let mut max_y: usize = 0;

    let cave_walls: CaveWalls = cave_scan
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|coord_str| -> Option<Point2D> {
                    let mut coord_split = coord_str.split(',');
                    let x: usize = coord_split.next()?.parse().ok()?;
                    let y: usize = coord_split.next()?.parse().ok()?;
                    if coord_split.next().is_some() {
                        return None;
                    }
                    min_x = min(min_x, x);
                    max_x = max(max_x, x);
                    max_y = max(max_y, y);

                    return Some(Point2D { x, y });
                })
                .collect::<Option<Vec<Point2D>>>()
        })
        .collect::<Option<CaveWalls>>()?;

    // only allow horizontal and vertical walls
    for cw in &cave_walls {
        for w in cw.windows(2) {
            let a = w[0];
            let b = w[1];
            if a.x != b.x && a.y != b.y {
                return None;
            }
        }
    }

    return Some(ParseResult::new(cave_walls, min_x, max_x, max_y));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Rock,
    Sand,
}

struct CaveSystem {
    map: Vec<Vec<Cell>>,
    x_offset: isize,
    // y_offset is always 0
    width: usize,
    height: usize,
    part_2: bool,
    y_infinite_plane: usize,
}

impl CaveSystem {
    fn new(pr: &ParseResult, part_2: bool) -> Self {
        let width = pr.max_x - pr.min_x + 1;
        let height = pr.max_y + 3;
        let mut res = Self {
            map: vec![vec![Cell::Empty; width]; height],
            x_offset: (pr.min_x as isize) * -1,
            width,
            height,
            part_2,
            y_infinite_plane: pr.max_y + 2,
        };

        for cw in &pr.cave_walls {
            for w in cw.windows(2) {
                let a = w[0];
                let b = w[1];
                let dx: isize = match a.x.cmp(&b.x) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                };
                let dy: isize = match a.y.cmp(&b.y) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                };

                let mut a = a;

                while a != b {
                    res[a] = Cell::Rock;
                    a.x = ((a.x as isize) + dx) as usize;
                    a.y = ((a.y as isize) + dy) as usize;
                }
                res[b] = Cell::Rock;
            }
        }
        return res;
    }

    fn grow(&mut self) {
        let grow_by: usize = 20;
        let mut new: Vec<Vec<Cell>> = Vec::new();

        for old in self.map.iter_mut() {
            let mut newr: Vec<Cell> = vec![Cell::Empty; grow_by];
            newr.append(old);
            newr.append(&mut vec![Cell::Empty; grow_by]);
            new.push(newr);
        }

        self.x_offset += grow_by as isize;
        self.width += grow_by * 2;
        self.map = new;
    }
}

impl Index<Point2D> for CaveSystem {
    type Output = Cell;

    fn index(&self, p: Point2D) -> &Self::Output {
        if self.part_2 && p.y == self.y_infinite_plane {
            return &Cell::Rock;
        }
        let xs = (p.x as isize) + self.x_offset;
        if xs < 0 {
            return &Cell::Empty;
        }
        let x = xs as usize;
        if x >= self.width {
            return &Cell::Empty;
        }
        if p.y >= self.height {
            return &Cell::Empty;
        }
        &self.map[p.y][x]
    }
}

impl IndexMut<Point2D> for CaveSystem {
    fn index_mut(&mut self, p: Point2D) -> &mut Self::Output {
        let xs: isize = (p.x as isize) + self.x_offset;
        let x: usize = xs as usize;
        if xs < 0 || x >= self.width {
            self.grow();
            return self.index_mut(p);
        }
        return &mut self.map[p.y][x];
    }
}

#[allow(unused)]
fn image(cave: &CaveSystem) {
    println!("P3");
    println!("{} {}", cave.map.first().unwrap().len(), cave.map.len());
    println!("255");
    for r in &cave.map {
        for c in r {
            match c {
                Cell::Empty => {
                    print!("255 255 255 ");
                }
                Cell::Rock => {
                    print!("0 0 0 ");
                }
                Cell::Sand => {
                    print!("200 200 0 ");
                }
            }
        }
        println!("");
    }
}

fn part_1(cave_scan: &str) -> Option<usize> {
    let parse_result = parse(cave_scan)?;
    let mut cave = CaveSystem::new(&parse_result, false);
    // let mut long_drops: HashMap<Point2D, usize> = HashMap::new(); the long drop optimizer is wrong

    let mut num_resting: usize = 0;
    'infinite_drop: loop {
        let mut p = Point2D { x: 500, y: 0 };
        let mut moved = true;
        while moved {
            moved = false;
            // if let Some(to_drop) = long_drops.get(&p) {
            // let to_drop: usize = *to_drop;
            // let ldrop = to_drop - 1;
            // if ldrop < 3 {
            // long_drops.remove(&p);
            // } else {
            // long_drops.insert(p, ldrop);
            // }
            // p.y += to_drop;
            // moved = true;
            // }

            // if p.y > parse_result.max_y {
            // break 'infinite_drop;
            // }
            // let start = p;
            // let mut drops: usize = 0;
            let mut below = Point2D { x: p.x, y: p.y + 1 };
            while cave[below] == Cell::Empty {
                if below.y > parse_result.max_y {
                    break 'infinite_drop;
                }
                // drops += 1;
                p = below;
                below.y += 1;
                moved = true;
            }
            // if drops > 3 {
            // long_drops.insert(start, drops - 1);
            // }

            let bl = Point2D {
                x: p.x - 1,
                y: p.y + 1,
            };
            if cave[bl] == Cell::Empty {
                p = bl;
                moved = true;
            } else {
                let br = Point2D {
                    x: p.x + 1,
                    y: p.y + 1,
                };
                if cave[br] == Cell::Empty {
                    p = br;
                    moved = true;
                }
            }
        }
        cave[p] = Cell::Sand;
        num_resting += 1;
    }

    // image(&cave);

    return Some(num_resting);
}

fn part_2(cave_scan: &str) -> Option<usize> {
    let parse_result = parse(cave_scan)?;
    let mut cave = CaveSystem::new(&parse_result, true);

    let mut start_blocked = false;
    let mut num_resting: usize = 0;
    while !start_blocked {
        let mut p = Point2D { x: 500, y: 0 };
        let mut moved = true;
        while moved {
            moved = false;

            let mut below = Point2D { x: p.x, y: p.y + 1 };
            while cave[below] == Cell::Empty {
                p = below;
                below.y += 1;
                moved = true;
            }

            let bl = Point2D {
                x: p.x - 1,
                y: p.y + 1,
            };
            if cave[bl] == Cell::Empty {
                p = bl;
                moved = true;
            } else {
                let br = Point2D {
                    x: p.x + 1,
                    y: p.y + 1,
                };
                if cave[br] == Cell::Empty {
                    p = br;
                    moved = true;
                }
            }
        }
        cave[p] = Cell::Sand;
        num_resting += 1;
        start_blocked = p.x == 500 && p.y == 0;
    }

    // image(&cave);

    return Some(num_resting);
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), Some(24));
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), Some(93));
}

fn main() {
    match part_1(INPUT) {
        Some(r) => {
            println!("part 1: {}", r);
        }
        None => {
            println!("part 1 failed.")
        }
    }
    match part_2(INPUT) {
        Some(r) => {
            println!("part 2: {}", r);
        }
        None => {
            println!("part 2 failed.")
        }
    }

    println!("done.");
}
