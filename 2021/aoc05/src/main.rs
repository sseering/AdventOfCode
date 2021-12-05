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

    // Are there intersection points of two lines that vertical or horizontal?
    fn intersection_ortogonal(&self, other: &Self) -> impl Iterator<Item = (i64, i64)> {
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
            for coord in a.intersection_ortogonal(b) {
                overlaps.insert(coord);
            }
        }
    }

    return overlaps.len();
}

fn part_2(vents_map: &str) -> usize {
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
        .intersection_ortogonal(&Linie::new(&format!(
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
    assert_eq!(part_2(TEST_INPUT), 12);
}

#[allow(unused)]
fn math() {
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
    //    0*x + (-1)*y -5
    // (-1)*x + (-1)*y +2
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
    println!("part 2: {0}", part_2(INPUT));
    println!("done");
}
