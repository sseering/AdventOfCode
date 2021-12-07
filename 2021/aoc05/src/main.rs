use std::cmp::{max, min, Ordering};
use std::collections::HashSet;

#[derive(Debug)]
struct Linie {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
}

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

fn min4(a: i64, b: i64, c: i64, d: i64) -> i64 {
    min(min(min(a, b), c), d)
}

fn max4(a: i64, b: i64, c: i64, d: i64) -> i64 {
    max(max(max(a, b), c), d)
}

impl Linie {
    fn new(coords: &str) -> Self {
        if let [a, _, c] = coords.split_whitespace().collect::<Vec<&str>>()[..] {
            if let [x1, y1] = a
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>()[..]
            {
                if let [x2, y2] = c
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<i64>>()[..]
                {
                    return Self { x1, y1, x2, y2 };
                }
            }
        }
        panic!();
    }

    /// Is this line vertival or horizontal?
    fn is_ortogonal(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn minmax_coord(&self) -> (i64, i64) {
        return (
            min4(self.x1, self.x2, self.y1, self.y2),
            max4(self.x1, self.x2, self.y1, self.y2),
        );
    }

    /// Return a description of the Linie in the form
    /// a * x + b * y + c == 0
    /// and its bounding box
    /// as the form (a, b, c, min_x, max_x, min_y, max_y)
    fn graph_form(&self) -> (i64, i64, i64, i64, i64, i64, i64) {
        let (left_x, left_y, right_x, right_y) = if self.x1 < self.x2 {
            (self.x1, self.y1, self.x2, self.y2)
        } else {
            (self.x2, self.y2, self.x1, self.y1)
        };

        let (a, b, c) = if left_x == right_x {
            (-1, 0, left_x)
        } else if left_y == right_y {
            (0, -1, left_y)
        } else {
            let a: f32 = ((self.y2 - self.y1) as f32) / ((self.x2 - self.x1) as f32);
            let a = a as i64;
            if a != 1 && a != -1 {
                panic!("we only handle lines with integer coordinates");
            }
            let c = self.y1 - a * self.x1;
            (a, -1, c)
        };

        return (
            a,
            b,
            c,
            min(self.x1, self.x2),
            max(self.x1, self.x2),
            min(self.y1, self.y2),
            max(self.y1, self.y2),
        );
    }

    /// Are there intersection points of two lines that are vertical or horizontal?
    fn intersections_ortogonal(&self, other: &Self) -> impl Iterator<Item = (i64, i64)> {
        let left_x = max(min(self.x1, self.x2), min(other.x1, other.x2));
        let right_x = min(max(self.x1, self.x2), max(other.x1, other.x2));
        let top_y = max(min(self.y1, self.y2), min(other.y1, other.y2));
        let bottom_y = min(max(self.y1, self.y2), max(other.y1, other.y2));

        // yield expressions in Rust are still experimental, so we don't use them.
        let mut result: Vec<(i64, i64)> = Vec::new();
        for x in left_x..=right_x {
            for y in top_y..=bottom_y {
                result.push((x, y));
            }
        }
        return result.into_iter();
    }

    fn coords(&self) -> Vec<(i64, i64)> {
        return if self.x1 == self.x2 && self.y1 == self.y2 {
            vec![(self.x1, self.y1)]
        } else if self.x1 == self.x2 {
            let (a, b) = if self.y1 < self.y2 {
                (self.y1, self.y2)
            } else {
                (self.y2, self.y1)
            };
            (a..=b).map(|v: i64| (self.x1, v)).collect()
        } else if self.y1 == self.y2 {
            let (a, b) = if self.x1 < self.x2 {
                (self.x1, self.x2)
            } else {
                (self.x2, self.x1)
            };
            (a..=b).map(|v: i64| (v, self.y1)).collect()
        } else {
            let (left_x, left_y, right_x, right_y) = if self.x1 < self.x2 {
                (self.x1, self.y1, self.x2, self.y2)
            } else {
                (self.x2, self.y2, self.x1, self.y1)
            };
            let step = if left_y < right_y { 1 } else { -1 };
            let mut y = left_y;

            let mut diag_coods = Vec::new();
            for x in left_x..=right_x {
                diag_coods.push((x, y));
                y += step;
            }
            diag_coods
        };
    }

    fn intersections(&self, other: &Self) -> impl Iterator<Item = (i64, i64)> {
        if self.is_ortogonal() && other.is_ortogonal() {
            return self
                .intersections_ortogonal(other)
                .collect::<Vec<(i64, i64)>>()
                .into_iter();
        }
        let (a1, b1, c1, min_x1, max_x1, min_y1, max_y1) = self.graph_form();
        let (a2, b2, c2, min_x2, max_x2, min_y2, max_y2) = other.graph_form();

        let mut result: Vec<(i64, i64)> = Vec::new();

        // colinear (overlapping) diagonal lines?
        if a1 == a2 && b1 == b2 {
            if c1 == c2 {
                let (y, step) = if a1 < 0 {
                    (min(max_y1, max_y2), -1)
                } else {
                    (max(min_y1, min_y2), 1)
                };
                let mut y = y;
                for x in max(min_x1, min_x2)..=min(max_x1, max_x2) {
                    result.push((x, y));
                    y += step;
                }
            }
            return result.into_iter();
        }

        if let Some((x, y)) = integer_intersection(a1, b1, c1, a2, b2, c2) {
            // is the intersction point inside the bounds of the lines?
            if min_x1 <= x
                && x <= max_x1
                && min_x2 <= x
                && x <= max_x2
                && min_y1 <= y
                && y <= max_y1
                && min_y2 <= y
                && y <= max_y2
            {
                result.push((x, y));
            }
        }
        return result.into_iter();
    }
}

fn parse_input(vents_map: &str) -> impl Iterator<Item = Linie> + '_ {
    return vents_map.lines().map(Linie::new);
}

fn part_1(vents_map: &str) -> usize {
    let vents: Vec<Linie> = parse_input(vents_map)
        .filter(|l| l.is_ortogonal())
        .collect();
    let mut overlaps: HashSet<(i64, i64)> = HashSet::new();

    for (idx, a) in vents.iter().enumerate() {
        for b in &vents[(idx + 1)..] {
            for coord in a.intersections_ortogonal(b) {
                overlaps.insert(coord);
            }
        }
    }

    return overlaps.len();
}

fn part_2_slow(vents_map: &str) -> usize {
    let vents: Vec<Linie> = parse_input(vents_map).collect();
    let mut overlaps: HashSet<(i64, i64)> = HashSet::new();

    for (idx, a) in vents.iter().enumerate() {
        let a_coords: HashSet<(i64, i64)> = HashSet::from_iter(a.coords());
        for b in &vents[(idx + 1)..] {
            for coord in b.coords() {
                if a_coords.contains(&coord) {
                    overlaps.insert(coord);
                }
            }
        }
    }

    return overlaps.len();
}

fn part_2(vents_map: &str) -> usize {
    let vents: Vec<Linie> = parse_input(vents_map).collect();
    let mut overlaps: HashSet<(i64, i64)> = HashSet::new();

    for (idx, a) in vents.iter().enumerate() {
        for b in &vents[(idx + 1)..] {
            for intersection_point in a.intersections(b) {
                overlaps.insert(intersection_point);
            }
        }
    }

    return overlaps.len();
}

#[allow(unused)]
fn testcase_helper_reverse(
    ax1: i64,
    ay1: i64,
    ax2: i64,
    ay2: i64,
    bx1: i64,
    by1: i64,
    bx2: i64,
    by2: i64,
) -> Vec<(i64, i64)> {
    return testcase_helper(bx1, by1, bx2, by2, ax1, ay1, ax2, ay2);
}

#[allow(unused)]
fn testcase_helper(
    ax1: i64,
    ay1: i64,
    ax2: i64,
    ay2: i64,
    bx1: i64,
    by1: i64,
    bx2: i64,
    by2: i64,
) -> Vec<(i64, i64)> {
    let mut result = Linie::new(&format!("{0},{1} -> {2},{3}", ax1, ay1, ax2, ay2))
        .intersections_ortogonal(&Linie::new(&format!(
            "{0},{1} -> {2},{3}",
            bx1, by1, bx2, by2
        )))
        .collect::<Vec<(i64, i64)>>();
    result.sort_by(testcase_cmp);
    return result;
}

#[allow(unused)]
fn testcase_cmp(a: &(i64, i64), b: &(i64, i64)) -> Ordering {
    let o = a.0.partial_cmp(&b.0).unwrap();
    if o != Ordering::Equal {
        return o;
    }
    return a.1.partial_cmp(&b.1).unwrap();
}

#[test]
fn test_0() {
    assert_eq!(testcase_helper(8, 0, 8, 5, 0, 1, 14, 1), vec![(8, 1)]);
    assert_eq!(
        testcase_helper_reverse(8, 0, 8, 5, 0, 1, 14, 1),
        vec![(8, 1)]
    );
}

#[test]
fn test_1() {
    assert_eq!(testcase_helper(8, 0, 8, 5, 0, 1, 14, 1), vec![(8, 1)]);
    assert_eq!(
        testcase_helper_reverse(8, 0, 8, 5, 0, 1, 14, 1),
        vec![(8, 1)]
    );
}

#[test]
fn test_2() {
    let mut expected = vec![(5, 1), (6, 1), (7, 1), (8, 1)];
    expected.sort_by(testcase_cmp);
    assert_eq!(testcase_helper(0, 1, 8, 1, 5, 1, 14, 1), expected);
    assert_eq!(testcase_helper_reverse(0, 1, 8, 1, 5, 1, 14, 1), expected);
}

#[test]
fn test_3() {
    assert_eq!(testcase_helper(8, 0, 8, 1, 0, 1, 14, 1), vec![(8, 1)]);
    assert_eq!(
        testcase_helper_reverse(8, 0, 8, 1, 0, 1, 14, 1),
        vec![(8, 1)]
    );
}

#[test]
fn test_4() {
    assert_eq!(testcase_helper(8, 2, 14, 2, 8, 1, 8, 6), vec![(8, 2)]);
    assert_eq!(
        testcase_helper_reverse(8, 2, 14, 2, 8, 1, 8, 6),
        vec![(8, 2)]
    );
}

#[test]
fn test_5() {
    let mut expected = vec![(11, 1), (12, 1), (13, 1), (14, 1)];
    expected.sort_by(testcase_cmp);
    assert_eq!(testcase_helper(0, 1, 14, 1, 11, 1, 14, 1), expected);
    assert_eq!(testcase_helper_reverse(0, 1, 14, 1, 11, 1, 14, 1), expected);
}

#[test]
fn test_6() {
    assert_eq!(testcase_helper(0, 1, 14, 1, 8, 1, 8, 5), vec![(8, 1)]);
    assert_eq!(
        testcase_helper_reverse(0, 1, 14, 1, 8, 1, 8, 5),
        vec![(8, 1)]
    );
}

#[test]
fn test_7() {
    assert_eq!(testcase_helper(0, 1, 8, 1, 8, 0, 8, 5), vec![(8, 1)]);
    assert_eq!(
        testcase_helper_reverse(0, 1, 8, 1, 8, 0, 8, 5),
        vec![(8, 1)]
    );
}

#[test]
fn test_8() {
    let mut expected = vec![(0, 1), (1, 1), (2, 1), (3, 1)];
    expected.sort_by(testcase_cmp);
    assert_eq!(testcase_helper(0, 1, 14, 1, 0, 1, 3, 1), expected);
    assert_eq!(testcase_helper_reverse(0, 1, 14, 1, 0, 1, 3, 1), expected);
}

#[test]
fn test_9() {
    assert_eq!(testcase_helper(0, 1, 14, 1, 8, 1, 8, 1), vec![(8, 1)]);
    assert_eq!(
        testcase_helper_reverse(0, 1, 14, 1, 8, 1, 8, 1),
        vec![(8, 1)]
    );
}

#[test]
fn test_10() {
    assert_eq!(testcase_helper(10, 1, 10, 6, 10, 2, 10, 2), vec![(10, 2)]);
    assert_eq!(
        testcase_helper_reverse(10, 1, 10, 6, 10, 2, 10, 2),
        vec![(10, 2)]
    );
}

#[test]
fn test_11() {
    assert_eq!(testcase_helper(0, 1, 14, 1, 7, 1, 7, 1), vec![(7, 1)]);
    assert_eq!(
        testcase_helper_reverse(0, 1, 14, 1, 7, 1, 7, 1),
        vec![(7, 1)]
    );
}

#[test]
fn test_12() {
    assert_eq!(testcase_helper(0, 1, 14, 1, 8, 4, 8, 5), Vec::new());
    assert_eq!(testcase_helper_reverse(0, 1, 14, 1, 8, 4, 8, 5), Vec::new());
}

#[test]
fn test_13() {
    assert_eq!(testcase_helper(6, 2, 6, 7, 8, 3, 12, 3), Vec::new());
    assert_eq!(testcase_helper_reverse(6, 2, 6, 7, 8, 3, 12, 3), Vec::new());
}

#[test]
fn test_14() {
    assert_eq!(testcase_helper(0, 1, 4, 1, 8, 1, 14, 1), Vec::new());
    assert_eq!(testcase_helper_reverse(0, 1, 4, 1, 8, 1, 14, 1), Vec::new());
}

#[test]
fn test_15() {
    assert_eq!(testcase_helper(8, 1, 8, 2, 0, 4, 14, 4), Vec::new());
    assert_eq!(testcase_helper_reverse(8, 1, 8, 2, 0, 4, 14, 4), Vec::new());
}

#[test]
fn test_16() {
    assert_eq!(testcase_helper(0, 2, 6, 2, 8, 1, 8, 7), Vec::new());
    assert_eq!(testcase_helper_reverse(0, 2, 6, 2, 8, 1, 8, 7), Vec::new());
}

#[test]
fn test_17() {
    assert_eq!(testcase_helper(0, 1, 14, 1, 14, 1, 14, 1), vec![(14, 1)]);
    assert_eq!(
        testcase_helper_reverse(0, 1, 14, 1, 14, 1, 14, 1),
        vec![(14, 1)]
    );
}

#[test]
fn test_18() {
    assert_eq!(testcase_helper(0, 1, 14, 1, 0, 1, 0, 1), vec![(0, 1)]);
    assert_eq!(
        testcase_helper_reverse(0, 1, 14, 1, 0, 1, 0, 1),
        vec![(0, 1)]
    );
}

#[test]
fn test_19() {
    let mut expected = vec![(4, 3), (4, 4), (4, 5), (4, 6)];
    expected.sort_by(testcase_cmp);
    assert_eq!(testcase_helper(4, 1, 4, 6, 4, 3, 4, 8), expected);
    assert_eq!(testcase_helper_reverse(4, 1, 4, 6, 4, 3, 4, 8), expected);
}

#[test]
fn test_20() {
    let mut expected = vec![(5, 8), (5, 9), (5, 10), (5, 11)];
    expected.sort_by(testcase_cmp);
    assert_eq!(testcase_helper(5, 1, 5, 11, 5, 8, 5, 11), expected);
    assert_eq!(testcase_helper_reverse(5, 1, 5, 11, 5, 8, 5, 11), expected);
}

#[test]
fn test_21() {
    let mut expected = vec![(1, 0), (1, 1), (1, 2), (1, 3)];
    expected.sort_by(testcase_cmp);
    assert_eq!(testcase_helper(1, 0, 1, 7, 1, 0, 1, 3), expected);
    assert_eq!(testcase_helper_reverse(1, 0, 1, 7, 1, 0, 1, 3), expected);
}

#[test]
fn test_22() {
    assert_eq!(testcase_helper(2, 0, 2, 8, 2, 4, 2, 4), vec![(2, 4)]);
    assert_eq!(
        testcase_helper_reverse(2, 0, 2, 8, 2, 4, 2, 4),
        vec![(2, 4)]
    );
}

#[test]
fn test_23() {
    assert_eq!(testcase_helper(5, 0, 5, 2, 5, 7, 5, 9), Vec::new());
    assert_eq!(testcase_helper_reverse(5, 0, 5, 2, 5, 7, 5, 9), Vec::new());
}

#[test]
fn test_24() {
    assert_eq!(testcase_helper(5, 0, 5, 10, 5, 0, 5, 0), vec![(5, 0)]);
    assert_eq!(
        testcase_helper_reverse(5, 0, 5, 10, 5, 0, 5, 0),
        vec![(5, 0)]
    );
}

#[test]
fn test_25() {
    assert_eq!(testcase_helper(3, 0, 3, 10, 3, 10, 3, 10), vec![(3, 10)]);
    assert_eq!(
        testcase_helper_reverse(3, 0, 3, 10, 3, 10, 3, 10),
        vec![(3, 10)]
    );
}

#[test]
fn test_26() {
    let mut expected = vec![(5, 1), (6, 1), (7, 1), (8, 1)];
    expected.sort_by(testcase_cmp);
    assert_eq!(testcase_helper(0, 1, 14, 1, 5, 1, 8, 1), expected);
    assert_eq!(testcase_helper_reverse(0, 1, 14, 1, 5, 1, 8, 1), expected);
}

#[test]
fn test_27() {
    let mut expected = vec![(4, 3), (4, 4), (4, 5), (4, 6)];
    expected.sort_by(testcase_cmp);
    assert_eq!(testcase_helper(4, 1, 4, 8, 4, 3, 4, 6), expected);
    assert_eq!(testcase_helper_reverse(4, 1, 4, 8, 4, 3, 4, 6), expected);
}

#[test]
fn test_28() {
    assert_eq!(testcase_helper(2, 1, 11, 1, 6, 6, 6, 11), Vec::new());
    assert_eq!(
        testcase_helper_reverse(2, 1, 11, 1, 6, 6, 6, 11),
        Vec::new()
    );
}

#[test]
fn test_29() {
    assert_eq!(testcase_helper(9, 1, 19, 1, 4, 6, 4, 11), Vec::new());
    assert_eq!(
        testcase_helper_reverse(9, 1, 19, 1, 4, 6, 4, 11),
        Vec::new()
    );
}

#[test]
fn test_30() {
    assert_eq!(testcase_helper(8, 2, 8, 7, 3, 11, 14, 11), Vec::new());
    assert_eq!(
        testcase_helper_reverse(8, 2, 8, 7, 3, 11, 14, 11),
        Vec::new()
    );
}

#[test]
fn test_31() {
    assert_eq!(testcase_helper(3, 2, 3, 7, 9, 11, 19, 11), Vec::new());
    assert_eq!(
        testcase_helper_reverse(3, 2, 3, 7, 9, 11, 19, 11),
        Vec::new()
    );
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), 5);
}

#[test]
fn test_b() {
    assert_eq!(part_2_slow(TEST_INPUT), 12);
}

#[test]
fn test_c() {
    assert_eq!(part_2(TEST_INPUT), 12);
}

#[test]
fn test_math_a() {
    assert_eq!(integer_intersection(-1, -1, 2, -1, 0, 8), Some((8, -6)));
}

#[test]
fn test_math_b() {
    assert_eq!(integer_intersection(1, -1, -10, -1, -1, 2), Some((6, -4)));
}

#[test]
fn test_math_c() {
    assert_eq!(integer_intersection(0, -1, -7, -1, -1, 2), Some((9, -7)));
}

#[test]
fn test_math_d() {
    assert_eq!(integer_intersection(1, -1, -10, -1, -1, 3), None);
}

#[test]
fn test_math_e() {
    assert_eq!(
        Linie::new("4,-2 -> 10,-8").graph_form(),
        (-1, -1, 2, 4, 10, -8, -2)
    );
}

#[test]
fn test_math_f() {
    assert_eq!(
        Linie::new("5,-7 -> 11,-7").graph_form(),
        (0, -1, -7, 5, 11, -7, -7)
    );
}

#[test]
fn test_math_g() {
    assert_eq!(
        Linie::new("8,-2 -> 8,-9").graph_form(),
        (-1, 0, 8, 8, 8, -9, -2)
    );
}

/// Find integer intersecton point of
/// a1 * x + b1 * y + c1 == 0
/// and a2 * x + b2 * y + c2 == 0
/// where both lines are not colinear and not parallel.
fn integer_intersection(
    a1: i64,
    b1: i64,
    c1: i64,
    a2: i64,
    b2: i64,
    c2: i64,
) -> Option<(i64, i64)> {
    if let Some((x, y)) = integer_intersection_f32(a1, b1, c1, a2, b2, c2) {
        let x = x as i64;
        let y = y as i64;

        // if x < -1000 || x > 1000 || y < -1000 || y > 1000 {
        //     println!("x {0} y {1}", x, y);
        //     println!("a1 {0} b1 {1} c1 {2}", a1, b1, c1);
        //     println!("a2 {0} b2 {1} c2 {2}", a2, b2, c2);
        //     return None;
        // }

        // We only want to use intersection points that are on integer coordinates.
        if (a1 * x + b1 * y + c1 == 0) && (a2 * x + b2 * y + c2 == 0) {
            return Some((x, y));
        }
        return None;
    }
    return None;
}
fn integer_intersection_f32(
    a1: i64,
    b1: i64,
    c1: i64,
    a2: i64,
    b2: i64,
    c2: i64,
) -> Option<(f32, f32)> {
    // 0=(-1)*x+(-1)*y+2=(-1)*x+0*y+8
    //
    //
    //
    // 0=x+(-1)*y-10=(-1)*x+(-1)*y+2
    //
    //
    // 0=0*x+(-1)*y-5=(-1)*x+(-1)*y+2
    //
    //
    //
    //
    // 0	1	2	3	4	5	6	7	8	9	10	11	12	13	14	15
    // 1			x
    // 2				x				o
    // 3					x			o
    // 4						x		o
    // 5							x	o
    // 6								o
    // 7								o	x
    // 8								o		x
    // 9								o			x
    // 10								o
    // 11								o
    // 12
    // 13
    // 14
    //
    //
    //
    //
    //
    // 0	1	2	3	4	5	6	7	8	9	10	11	12	13	14	15
    // 1			x						o
    // 2				x				o
    // 3					x		o
    // 4						o
    // 5					o		x
    // 6				o				x
    // 7			o						x
    // 8		o								x
    // 9											x
    // 10
    // 11
    // 12
    // 13
    // 14
    //
    //
    //
    //
    //
    //
    //
    //
    // 0	1	2	3	4	5	6	7	8	9	10	11	12	13	14	15
    // 1			x
    // 2				x
    // 3					x
    // 4						x
    // 5							x
    // 6								x
    // 7				o	o	o	o	o	o	o	o	o
    // 8										x
    // 9											x
    // 10
    // 11
    // 12
    // 13
    // 14
    //
    //
    //   a         b    c
    // (-1)*x + (-1)*y +2
    // (-1)*x +    0*y +8
    //
    //    1*x + (-1)*y -10
    // (-1)*x + (-1)*y +2
    //
    //    0*x + (-1)*y -7
    // (-1)*x + (-1)*y +2

    if a1 == 0 {
        let y: f32 = ((-1 * c1) as f32) / (b1 as f32);
        let x: f32 = ((((-1 * b2) as f32) * y) - (c2 as f32)) / (a2 as f32);
        return Some((x, y));
    }

    if a2 == 0 {
        let y: f32 = ((-1 * c2) as f32) / (b2 as f32);
        let x: f32 = ((((-1 * b1) as f32) * y) - (c1 as f32)) / (a1 as f32);
        return Some((x, y));
    }

    if b1 == 0 {
        let x: f32 = ((-1 * c1) as f32) / (a1 as f32);
        let y: f32 = ((((-1 * a2) as f32) * x) - (c2 as f32)) / (b2 as f32);
        return Some((x, y));
    }

    if b2 == 0 {
        let x: f32 = ((-1 * c2) as f32) / (a2 as f32);
        let y: f32 = ((((-1 * a1) as f32) * x) - (c1 as f32)) / (b1 as f32);
        return Some((x, y));
    }

    let a1: f32 = a1 as f32;
    let b1: f32 = b1 as f32;
    let c1: f32 = c1 as f32;
    let a2: f32 = a2 as f32;
    let mut b2: f32 = b2 as f32;
    let mut c2: f32 = c2 as f32;

    //    1*x + (-1)*y -10
    // (-1)*x + (-1)*y +2
    //
    //  1    -1    -10
    // -1    -1     +2           factor == 1
    //
    //
    //  1    -1    -10
    //       -2     -8    addition
    //       y=-4

    let factor: f32 = -1.0 * a2 / a1; // a2 + factor * a1 == 0

    // eliminate x from second line
    b2 += b1 * factor;
    c2 += c1 * factor;

    let y: f32 = (-1.0 * c2) / b2;

    //    1*x + (-1)*y -10
    //    1*x + (-1)*(-4) -10
    //    x + (-1)*(-4) -10
    //    x = 6
    let x: f32 = ((b1 * y + c1) * -1.0) / a1;

    return Some((x, y));
}

fn main() {
    println!(
        "min max coords (size of puzzle area): {0:?}",
        parse_input(INPUT).fold((std::i64::MAX, std::i64::MIN), |o, l| {
            let minmax = l.minmax_coord();
            return (min(o.0, minmax.0), max(o.1, minmax.1));
        })
    );

    println!("part 1: {0}", part_1(INPUT));
    println!("starting to compute part 2 slow variant");
    println!("part 2: {0}", part_2_slow(INPUT));
    println!("part 2: {0}", part_2(INPUT));
    println!("done");
}
