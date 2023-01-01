use regex::Regex;
use std::cmp::{max, min, Ordering};
use std::collections::HashSet;

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coord2D {
    x: i32,
    y: i32,
}

struct Scanner {
    s: Coord2D,
    b: Coord2D,
}

impl Scanner {
    fn parse(s: &str) -> Option<Self> {
        let re = Regex::new(r"Sensor\s+at\s+x=(-?\d+),\s+y=(-?\d+):\s+closest\s+beacon\s+is\s+at\s+x=(-?\d+),\s+y=(-?\d+)").ok()?;
        let caps = re.captures(s)?;
        return Some(Self {
            s: Coord2D {
                x: caps[1].parse().ok()?,
                y: caps[2].parse().ok()?,
            },
            b: Coord2D {
                x: caps[3].parse().ok()?,
                y: caps[4].parse().ok()?,
            },
        });
    }
}

fn manhattan_dist(a: &Coord2D, b: &Coord2D) -> i32 {
    return i32::abs(a.x - b.x) + i32::abs(a.y - b.y);
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct RangeStartEnd {
    s: i32, // start is inclusive
    e: i32, // end is exclusive
}

#[derive(Debug, Eq, PartialEq)]
enum RangeSplitResult {
    Unchanged,
    Vanished,
    Smaller(RangeStartEnd),
    Split(RangeStartEnd, RangeStartEnd),
}

impl RangeStartEnd {
    fn new(start: i32, end: i32) -> Self {
        if end <= start {
            panic!();
        }
        Self { s: start, e: end }
    }

    fn join_if_touching(&self, other: &Self) -> Option<Self> {
        if other.e < self.s {
            return None;
        }
        if other.s > self.e {
            return None;
        }
        if other.s < self.s {
            return other.join_if_touching(self);
        }
        return Some(RangeStartEnd::new(self.s, max(self.e, other.e)));
    }

    fn split_if_touching(&self, other: &Self) -> RangeSplitResult {
        if other.e <= self.s {
            return RangeSplitResult::Unchanged;
        }
        if other.s >= self.e {
            return RangeSplitResult::Unchanged;
        }
        if other.s <= self.s {
            if other.e >= self.e {
                return RangeSplitResult::Vanished;
            }
            return RangeSplitResult::Smaller(RangeStartEnd::new(other.e, self.e));
        }

        if other.e >= self.e {
            return RangeSplitResult::Smaller(RangeStartEnd::new(self.s, other.s));
        }

        return RangeSplitResult::Split(
            RangeStartEnd::new(self.s, other.s),
            RangeStartEnd::new(other.e, self.e),
        );
    }
}

#[derive(PartialEq, Eq)]
struct Ranges1D {
    ranges: Vec<RangeStartEnd>,
}

impl Ranges1D {
    fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn create_prefilled(prefill_range_end: i32) -> Self {
        let mut res = Self::new();
        res.add(0, prefill_range_end);
        return res;
    }

    fn binary_search(&self, start: i32) -> usize {
        // assumes self.ranges is not empty
        // returns 0 if start < self.ranges.first().unwrap().s
        // returns the smallest index for which start >= self.ranges[idx].s

        let mut idx_from: usize = 0;
        let mut idx_to = self.ranges.len();
        while idx_from + 1 < idx_to {
            let mid = idx_from + (idx_to - idx_from) / 2;
            match start.cmp(&self.ranges[mid].s) {
                Ordering::Less => {
                    idx_to = mid;
                }
                Ordering::Equal => {
                    idx_from = mid;
                    idx_to = mid;
                }
                Ordering::Greater => {
                    idx_from = mid;
                }
            }
        }

        return idx_from;
    }

    fn add(&mut self, start: i32, end: i32) {
        let to_add = RangeStartEnd::new(start, end);
        if self.ranges.is_empty() {
            self.ranges.push(to_add);
            return;
        }

        let mut idx = self.binary_search(start);

        let old_first_start = self.ranges.first().unwrap().s;
        if idx == 0 && start < old_first_start {
            match to_add.join_if_touching(self.ranges.first().unwrap()) {
                Some(_) => {
                    // nothing
                }
                None => {
                    self.ranges.insert(0, to_add);
                    return;
                }
            }
        }

        match to_add.join_if_touching(&self.ranges[idx]) {
            Some(joined) => {
                self.ranges[idx] = joined;
            }
            None => {
                idx += 1;
                self.ranges.insert(idx, to_add);
            }
        }
        let mut did_change = true;
        while did_change && idx + 1 < self.ranges.len() {
            did_change = false;

            let maybe_join_this = &self.ranges[idx];
            match maybe_join_this.join_if_touching(&self.ranges[idx + 1]) {
                Some(joined) => {
                    self.ranges[idx] = joined;
                    self.ranges.remove(idx + 1);
                    did_change = true;
                }
                None => {
                    // nothing
                }
            }
        }
    }

    fn remove(&mut self, start: i32, end: i32) {
        if self.ranges.is_empty() {
            return;
        }

        let mut idx = self.binary_search(start);

        let to_remove = RangeStartEnd::new(start, end);

        let mut keep_going = true;
        let mut idx_change: usize;
        while keep_going {
            match &self.ranges[idx].split_if_touching(&to_remove) {
                RangeSplitResult::Unchanged => {
                    idx_change = 1;
                }
                RangeSplitResult::Vanished => {
                    self.ranges.remove(idx);
                    idx_change = 0;
                }
                RangeSplitResult::Smaller(s) => {
                    self.ranges[idx] = s.clone();
                    idx_change = 1;
                }
                RangeSplitResult::Split(a, b) => {
                    self.ranges[idx] = a.clone();
                    self.ranges.insert(idx + 1, b.clone());
                    idx_change = 2;
                }
            }

            idx += idx_change;
            if idx < self.ranges.len() && to_remove.e > self.ranges[idx].s {
                keep_going = true;
            } else {
                keep_going = false;
            }
        }
    }

    fn is_lenth_1(&self) -> bool {
        if self.ranges.len() != 1 {
            return false;
        }

        let r = self.ranges.first().unwrap();
        return r.s + 1 == r.e;
    }
}

fn part_1(cave_scan: &str, y_selector: i32) -> Option<usize> {
    let mut beacons_in_row: HashSet<Coord2D> = HashSet::new();

    let scanners: Vec<Scanner> = cave_scan
        .lines()
        .map(|l| {
            let res = Scanner::parse(l);
            if let Some(s) = &res {
                if s.b.y == y_selector {
                    beacons_in_row.insert(s.b.clone());
                }
            }

            return res;
        })
        .collect::<Option<Vec<Scanner>>>()?;

    let mut cant_be_beacon = Ranges1D::new();

    for scanner in scanners {
        let scanner_range = manhattan_dist(&scanner.s, &scanner.b);
        let scanner_vertical_crossing_selected_row = Coord2D {
            x: scanner.s.x,
            y: y_selector,
        };
        let dist_to_y =
            scanner_range - manhattan_dist(&scanner.s, &scanner_vertical_crossing_selected_row);
        if dist_to_y >= 0 {
            let start = scanner_vertical_crossing_selected_row.x - dist_to_y;
            let end = scanner_vertical_crossing_selected_row.x + dist_to_y + 1;
            cant_be_beacon.add(start, end);
        }
    }

    let mut result: usize = 0;
    for range in &cant_be_beacon.ranges {
        let mut num_blocked: usize = (range.e - range.s) as usize;
        for bir in &beacons_in_row {
            if bir.x >= range.s && bir.x < range.e {
                num_blocked -= 1;
            }
        }
        result += num_blocked;
    }

    return Some(result);
}

fn part_2(cave_scan: &str, max_coord: usize) -> Option<usize> {
    let scanners: Vec<Scanner> = cave_scan
        .lines()
        .map(Scanner::parse)
        .collect::<Option<Vec<Scanner>>>()?;
    let list_size = max_coord + 1;
    let mut ranges: Vec<Ranges1D> = Vec::new();
    for _ in 0..list_size {
        ranges.push(Ranges1D::create_prefilled(list_size as i32));
    }
    let mut y_coords_with_1_possible_beacon: HashSet<usize> = HashSet::new();

    for scanner in scanners {
        let scanner_range = manhattan_dist(&scanner.s, &scanner.b);
        let y_from: usize = max(0, scanner.s.y - scanner_range) as usize;
        let y_to: usize = min(max_coord, (scanner.s.y + scanner_range) as usize);
        for y in y_from..=y_to {
            let width = scanner_range
                - manhattan_dist(
                    &scanner.s,
                    &Coord2D {
                        x: scanner.s.x,
                        y: y as i32,
                    },
                );
            let start = scanner.s.x - width;
            let end = scanner.s.x + width + 1;
            let before = ranges[y].is_lenth_1();
            ranges[y].remove(start, end);
            let after = ranges[y].is_lenth_1();
            if before && !after {
                y_coords_with_1_possible_beacon.remove(&y);
            } else if !before && after {
                y_coords_with_1_possible_beacon.insert(y);
            }
        }
    }

    if y_coords_with_1_possible_beacon.len() != 1 {
        return None;
    }

    let y = *y_coords_with_1_possible_beacon.iter().next().unwrap();
    let x: usize = ranges[y].ranges.first().unwrap().s as usize;

    return Some(x * 4000000 + y);
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT, 10), Some(26));
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT, 20), Some(56000011));
}

#[test]
fn test_ba() {
    assert_eq!(
        RangeStartEnd::new(400, 600).split_if_touching(&RangeStartEnd::new(100, 200)),
        RangeSplitResult::Unchanged
    );
}

#[test]
fn test_bb() {
    assert_eq!(
        RangeStartEnd::new(400, 600).split_if_touching(&RangeStartEnd::new(800, 900)),
        RangeSplitResult::Unchanged
    );
}

#[test]
fn test_bc() {
    assert_eq!(
        RangeStartEnd::new(400, 600).split_if_touching(&RangeStartEnd::new(300, 400)),
        RangeSplitResult::Unchanged
    );
}

#[test]
fn test_bd() {
    assert_eq!(
        RangeStartEnd::new(400, 600).split_if_touching(&RangeStartEnd::new(600, 900)),
        RangeSplitResult::Unchanged
    );
}

#[test]
fn test_be() {
    assert_eq!(
        RangeStartEnd::new(400, 600).split_if_touching(&RangeStartEnd::new(200, 900)),
        RangeSplitResult::Vanished
    );
}

#[test]
fn test_bf() {
    assert_eq!(
        RangeStartEnd::new(400, 600).split_if_touching(&RangeStartEnd::new(400, 600)),
        RangeSplitResult::Vanished
    );
}

#[test]
fn test_bg() {
    assert_eq!(
        RangeStartEnd::new(400, 600).split_if_touching(&RangeStartEnd::new(100, 500)),
        RangeSplitResult::Smaller(RangeStartEnd::new(500, 600))
    );
}

#[test]
fn test_bh() {
    assert_eq!(
        RangeStartEnd::new(400, 600).split_if_touching(&RangeStartEnd::new(500, 700)),
        RangeSplitResult::Smaller(RangeStartEnd::new(400, 500))
    );
}

#[test]
fn test_bi() {
    assert_eq!(
        RangeStartEnd::new(400, 600).split_if_touching(&RangeStartEnd::new(400, 500)),
        RangeSplitResult::Smaller(RangeStartEnd::new(500, 600))
    );
}

#[test]
fn test_bj() {
    assert_eq!(
        RangeStartEnd::new(400, 600).split_if_touching(&RangeStartEnd::new(500, 600)),
        RangeSplitResult::Smaller(RangeStartEnd::new(400, 500))
    );
}

#[test]
fn test_bk() {
    assert_eq!(
        RangeStartEnd::new(400, 600).split_if_touching(&RangeStartEnd::new(450, 550)),
        RangeSplitResult::Split(RangeStartEnd::new(400, 450), RangeStartEnd::new(550, 600))
    );
}

#[test]
fn test_ca() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000);
    r1d.remove(0, 100);
    assert_eq!(r1d.ranges, vec![RangeStartEnd::new(1000, 4000)]);
}

#[test]
fn test_cb() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000);
    r1d.remove(5000, 6100);
    assert_eq!(r1d.ranges, vec![RangeStartEnd::new(1000, 4000)]);
}

#[test]
fn test_cc() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000);
    r1d.remove(0, 1000);
    assert_eq!(r1d.ranges, vec![RangeStartEnd::new(1000, 4000)]);
}

#[test]
fn test_cd() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000);
    r1d.remove(4000, 44100);
    assert_eq!(r1d.ranges, vec![RangeStartEnd::new(1000, 4000)]);
}

#[test]
fn test_ce() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000);
    r1d.remove(1200, 3100);
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1200),
            RangeStartEnd::new(3100, 4000)
        ]
    );
}

#[test]
fn test_cf() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000);
    r1d.remove(1000, 3400);
    assert_eq!(r1d.ranges, vec![RangeStartEnd::new(3400, 4000)]);
}

#[test]
fn test_cg() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000);
    r1d.remove(3300, 4000);
    assert_eq!(r1d.ranges, vec![RangeStartEnd::new(1000, 3300)]);
}

#[test]
fn test_ch() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000);
    r1d.remove(100, 3400);
    assert_eq!(r1d.ranges, vec![RangeStartEnd::new(3400, 4000)]);
}

#[test]
fn test_ci() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000);
    r1d.remove(3300, 40400);
    assert_eq!(r1d.ranges, vec![RangeStartEnd::new(1000, 3300)]);
}

#[test]
fn test_cj() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500     2000 4000
    r1d.remove(2500, 3000); // 1000 1500     2000 2500       3000 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1500),
            RangeStartEnd::new(2000, 2500),
            RangeStartEnd::new(3000, 4000)
        ]
    );
}

#[test]
fn test_ck() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(0, 2000); // 2000 4000
    r1d.remove(2500, 3000); // 2000 2500         3000 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(2000, 2500),
            RangeStartEnd::new(3000, 4000)
        ]
    );
}

#[test]
fn test_cl() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1000, 2000); // 2000 4000
    r1d.remove(2500, 3000); // 2000 2500       3000 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(2000, 2500),
            RangeStartEnd::new(3000, 4000)
        ]
    );
}

#[test]
fn test_cm() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500      2000 4000
    r1d.remove(2500, 33000); // 1000 1500      2000 2500
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1500),
            RangeStartEnd::new(2000, 2500),
        ]
    );
}

#[test]
fn test_cn() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500        2000 4000
    r1d.remove(2500, 4000); // 1000 1500        2000 2500
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1500),
            RangeStartEnd::new(2000, 2500),
        ]
    );
}

#[test]
fn test_co() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500       2000 4000
    r1d.remove(2500, 3000); // 1000 1500       2000 2500              3000 4000
    r1d.remove(2000, 2500); // 1000 1500       3000 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1500),
            RangeStartEnd::new(3000, 4000)
        ]
    );
}

#[test]
fn test_cp() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500          2000 4000
    r1d.remove(2500, 3000); // 1000 1500          2000 2500           3000 4000
    r1d.remove(0, 1500); // 2000 2500           3000 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(2000, 2500),
            RangeStartEnd::new(3000, 4000)
        ]
    );
}

#[test]
fn test_cq() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500         2000 4000
    r1d.remove(2500, 3000); // 1000 1500         2000 2500         3000 4000
    r1d.remove(2555, 33000); // 1000 1500         2000 2500
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1500),
            RangeStartEnd::new(2000, 2500),
        ]
    );
}

#[test]
fn test_cr() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500          2000 4000
    r1d.remove(2500, 3000); // 1000 1500          2000 2500               3000 4000
    r1d.remove(500, 1234); // 1234 1500          2000 2500               3000 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1234, 1500),
            RangeStartEnd::new(2000, 2500),
            RangeStartEnd::new(3000, 4000)
        ]
    );
}

#[test]
fn test_cs() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500           2000 4000
    r1d.remove(2500, 3000); // 1000 1500           2000 2500             3000 4000
    r1d.remove(1900, 2222); // 1000 1500           2222 2500             3000 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1500),
            RangeStartEnd::new(2222, 2500),
            RangeStartEnd::new(3000, 4000)
        ]
    );
}

#[test]
fn test_ct() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500             2000 4000
    r1d.remove(2500, 3000); // 1000 1500             2000 2500           3000 4000
    r1d.remove(2500, 3333); // 1000 1500             2000 2500           3333 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1500),
            RangeStartEnd::new(2000, 2500),
            RangeStartEnd::new(3333, 4000)
        ]
    );
}

#[test]
fn test_cu() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500          2000 4000
    r1d.remove(2500, 3000); // 1000 1500          2000 2500               3000 4000
    r1d.remove(1400, 1999); // 1000 1400          2000 2500               3000 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1400),
            RangeStartEnd::new(2000, 2500),
            RangeStartEnd::new(3000, 4000)
        ]
    );
}

#[test]
fn test_cv() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500           2000 4000
    r1d.remove(2500, 3000); // 1000 1500           2000 2500             3000 4000
    r1d.remove(2222, 2600); // 1000 1500           2000 2222             3000 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1500),
            RangeStartEnd::new(2000, 2222),
            RangeStartEnd::new(3000, 4000)
        ]
    );
}

#[test]
fn test_cw() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500             2000 4000
    r1d.remove(2500, 3000); // 1000 1500             2000 2500           3000 4000
    r1d.remove(3333, 4444); // 1000 1500             2000 2500           3000 3333
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1500),
            RangeStartEnd::new(2000, 2500),
            RangeStartEnd::new(3000, 3333)
        ]
    );
}

#[test]
fn test_caa() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500          2000 4000
    r1d.remove(2500, 3000); // 1000 1500          2000 2500               3000 4000
    r1d.remove(1000, 1234); // 1234 1500          2000 2500               3000 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1234, 1500),
            RangeStartEnd::new(2000, 2500),
            RangeStartEnd::new(3000, 4000)
        ]
    );
}

#[test]
fn test_cab() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500           2000 4000
    r1d.remove(2500, 3000); // 1000 1500           2000 2500             3000 4000
    r1d.remove(2000, 2222); // 1000 1500           2222 2500             3000 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1500),
            RangeStartEnd::new(2222, 2500),
            RangeStartEnd::new(3000, 4000)
        ]
    );
}

#[test]
fn test_cac() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500             2000 4000
    r1d.remove(2500, 3000); // 1000 1500             2000 2500           3000 4000
    r1d.remove(3000, 3333); // 1000 1500             2000 2500           3333 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1500),
            RangeStartEnd::new(2000, 2500),
            RangeStartEnd::new(3333, 4000)
        ]
    );
}

#[test]
fn test_cad() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500          2000 4000
    r1d.remove(2500, 3000); // 1000 1500          2000 2500               3000 4000
    r1d.remove(1400, 1500); // 1000 1400          2000 2500               3000 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1400),
            RangeStartEnd::new(2000, 2500),
            RangeStartEnd::new(3000, 4000)
        ]
    );
}

#[test]
fn test_cae() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500           2000 4000
    r1d.remove(2500, 3000); // 1000 1500           2000 2500             3000 4000
    r1d.remove(2222, 2500); // 1000 1500           2000 2222             3000 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1500),
            RangeStartEnd::new(2000, 2222),
            RangeStartEnd::new(3000, 4000)
        ]
    );
}

#[test]
fn test_caf() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500             2000 4000
    r1d.remove(2500, 3000); // 1000 1500             2000 2500           3000 4000
    r1d.remove(3333, 4000); // 1000 1500             2000 2500           3000 3333
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1500),
            RangeStartEnd::new(2000, 2500),
            RangeStartEnd::new(3000, 3333)
        ]
    );
}

#[test]
fn test_cag() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500          2000 4000
    r1d.remove(2500, 3000); // 1000 1500          2000 2500               3000 4000
    r1d.remove(500, 1234); // 1234 1500          2000 2500               3000 4000
    r1d.remove(2222, 2223); // 1234 1500          2000 2222              2223 2500               3000 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1234, 1500),
            RangeStartEnd::new(2000, 2222),
            RangeStartEnd::new(2223, 2500),
            RangeStartEnd::new(3000, 4000)
        ]
    );
}

#[test]
fn test_cah() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500           2000 4000
    r1d.remove(2500, 3000); // 1000 1500           2000 2500             3000 4000
    r1d.remove(1900, 2222); // 1000 1500           2222 2500             3000 4000
    r1d.remove(3333, 3334); // 1000 1500           2222 2500             3000 3333          3334 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1500),
            RangeStartEnd::new(2222, 2500),
            RangeStartEnd::new(3000, 3333),
            RangeStartEnd::new(3334, 4000)
        ]
    );
}

#[test]
fn test_cai() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500             2000 4000
    r1d.remove(2500, 3000); // 1000 1500             2000 2500           3000 4000
    r1d.remove(2500, 3333); // 1000 1500             2000 2500           3333 4000
    r1d.remove(1111, 1112); // 1000 1111             1112 1500           2000 2500           3333 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1111),
            RangeStartEnd::new(1112, 1500),
            RangeStartEnd::new(2000, 2500),
            RangeStartEnd::new(3333, 4000)
        ]
    );
}

#[test]
fn test_caj() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500          2000 4000
    r1d.remove(2500, 3000); // 1000 1500          2000 2500               3000 4000
    r1d.remove(500, 1234); // 1234 1500          2000 2500               3000 4000
    r1d.remove(2222, 2223); // 1234 1500          2000 2222              2223 2500               3000 4000
    r1d.remove(1999, 2600); // 1234 1500          3000 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1234, 1500),
            RangeStartEnd::new(3000, 4000)
        ]
    );
}

#[test]
fn test_cak() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500           2000 4000
    r1d.remove(2500, 3000); // 1000 1500           2000 2500             3000 4000
    r1d.remove(1900, 2222); // 1000 1500           2222 2500             3000 4000
    r1d.remove(3333, 3334); // 1000 1500           2222 2500             3000 3333          3334 4000
    r1d.remove(2999, 4444); // 1000 1500           2222 2500
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1500),
            RangeStartEnd::new(2222, 2500),
        ]
    );
}

#[test]
fn test_cal() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500             2000 4000
    r1d.remove(2500, 3000); // 1000 1500             2000 2500           3000 4000
    r1d.remove(2500, 3333); // 1000 1500             2000 2500           3333 4000
    r1d.remove(1111, 1112); // 1000 1111             1112 1500           2000 2500           3333 4000
    r1d.remove(0, 1666); // 2000 2500           3333 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(2000, 2500),
            RangeStartEnd::new(3333, 4000)
        ]
    );
}

#[test]
fn test_cam() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500          2000 4000
    r1d.remove(2500, 3000); // 1000 1500          2000 2500               3000 4000
    r1d.remove(500, 1234); // 1234 1500          2000 2500               3000 4000
    r1d.remove(2222, 2223); // 1234 1500          2000 2222              2223 2500               3000 4000
    r1d.remove(2000, 2500); // 1234 1500          3000 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1234, 1500),
            RangeStartEnd::new(3000, 4000)
        ]
    );
}

#[test]
fn test_can() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500           2000 4000
    r1d.remove(2500, 3000); // 1000 1500           2000 2500             3000 4000
    r1d.remove(1900, 2222); // 1000 1500           2222 2500             3000 4000
    r1d.remove(3333, 3334); // 1000 1500           2222 2500             3000 3333          3334 4000
    r1d.remove(3000, 4000); // 1000 1500           2222 2500
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(1000, 1500),
            RangeStartEnd::new(2222, 2500),
        ]
    );
}

#[test]
fn test_cao() {
    let mut r1d = Ranges1D::new();
    r1d.add(1000, 4000); // 1000 4000
    r1d.remove(1500, 2000); // 1000 1500             2000 4000
    r1d.remove(2500, 3000); // 1000 1500             2000 2500           3000 4000
    r1d.remove(2500, 3333); // 1000 1500             2000 2500           3333 4000
    r1d.remove(1111, 1112); // 1000 1111             1112 1500           2000 2500           3333 4000
    r1d.remove(1000, 1500); // 2000 2500           3333 4000
    assert_eq!(
        r1d.ranges,
        vec![
            RangeStartEnd::new(2000, 2500),
            RangeStartEnd::new(3333, 4000)
        ]
    );
}

fn main() {
    match part_1(INPUT, 2000000) {
        Some(r) => {
            println!("part 1: {} {}", r, r == 6425133);
        }
        None => {
            println!("part 1 failed.")
        }
    }
    match part_2(INPUT, 4000000) {
        Some(r) => {
            println!("part 2: {}", r);
        }
        None => {
            println!("part 2 failed.")
        }
    }

    println!("done.");
}
