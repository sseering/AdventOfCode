use std::collections::HashSet;
use std::{fs::OpenOptions, io::Write};

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn test_a() {
        assert_eq!(
            parse_1_2(TEST_INPUT),
            Some(RedTiles {
                tiles: vec![
                    Coord2d { x: 7, y: 1 },
                    Coord2d { x: 11, y: 1 },
                    Coord2d { x: 11, y: 7 },
                    Coord2d { x: 9, y: 7 },
                    Coord2d { x: 9, y: 5 },
                    Coord2d { x: 2, y: 5 },
                    Coord2d { x: 2, y: 3 },
                    Coord2d { x: 7, y: 3 }
                ],
                min_x_idx: 6
            })
        );
    }

    #[test]
    fn test_b() {
        let red_tiles = parse_1_2(TEST_INPUT);
        assert!(red_tiles.is_some());
        let red_tiles = red_tiles.unwrap();
        let convex_hull = red_tiles.convex_hull();

        let exptected = vec![
            Coord2d { x: 2, y: 3 },
            Coord2d { x: 2, y: 5 },
            Coord2d { x: 9, y: 7 },
            Coord2d { x: 11, y: 7 },
            Coord2d { x: 11, y: 1 },
            Coord2d { x: 7, y: 1 },
        ];

        assert_eq!(convex_hull, exptected);
    }

    #[test]
    fn test_c() {
        assert_eq!(part_1(TEST_INPUT), Some(50));
    }

    macro_rules! turn_direction_testcase {
        ($fn_name:ident, $expected:expr, $x1: literal, $y1: literal, $x2: literal, $y2: literal, $x3: literal, $y3: literal) => {
            #[test]
            fn $fn_name() {
                let p3 = vec![
                    Coord2d { x: $x1, y: $y1 },
                    Coord2d { x: $x2, y: $y2 },
                    Coord2d { x: $x3, y: $y3 },
                ];
                assert_eq!(turn_direction(&p3[0], &p3[1], &p3[2]), $expected);
                let path = format!("/tmp/{}.ppm", stringify!($fn_name));
                write_netbpm(&path, &p3, &HashSet::from([2]));
            }
        };
    }

    turn_direction_testcase!(test_cl_a, TurnDirection::Coliniar, 5, 5, 15, 5, 25, 5);
    turn_direction_testcase!(test_cl_b, TurnDirection::Coliniar, 5, 5, 15, 5, 35, 5);
    turn_direction_testcase!(test_cl_c, TurnDirection::Coliniar, 5, 5, 15, 5, 7, 5);
    turn_direction_testcase!(test_cl_d, TurnDirection::Coliniar, 5, 5, 15, 5, 2, 5);
    turn_direction_testcase!(test_cl_e, TurnDirection::Coliniar, 5, 5, 5, 15, 5, 25);
    turn_direction_testcase!(test_cl_f, TurnDirection::Coliniar, 5, 5, 5, 15, 5, 35);
    turn_direction_testcase!(test_cl_g, TurnDirection::Coliniar, 5, 5, 5, 15, 5, 7);
    turn_direction_testcase!(test_cl_h, TurnDirection::Coliniar, 5, 5, 5, 15, 5, 2);
    turn_direction_testcase!(test_cl_i, TurnDirection::Coliniar, 5, 5, 15, 15, 25, 25);
    turn_direction_testcase!(test_cl_j, TurnDirection::Coliniar, 5, 5, 15, 15, 35, 35);
    turn_direction_testcase!(test_cl_k, TurnDirection::Coliniar, 5, 5, 15, 15, 7, 7);
    turn_direction_testcase!(test_cl_l, TurnDirection::Coliniar, 5, 5, 15, 15, 2, 2);
    turn_direction_testcase!(test_cc_a, TurnDirection::CCW, 5, 5, 15, 5, 25, 2);
    turn_direction_testcase!(test_cc_b, TurnDirection::CCW, 5, 5, 15, 5, 15, 3);
    turn_direction_testcase!(test_cc_c, TurnDirection::CCW, 5, 5, 15, 5, 7, 1);
    turn_direction_testcase!(test_cc_d, TurnDirection::CCW, 5, 5, 15, 5, 5, 0);
    turn_direction_testcase!(test_cc_e, TurnDirection::CCW, 5, 5, 15, 5, 2, 2);
    turn_direction_testcase!(test_cc_f, TurnDirection::CCW, 5, 5, 15, 15, 30, 25);
    turn_direction_testcase!(test_cc_g, TurnDirection::CCW, 5, 5, 15, 15, 20, 15);
    turn_direction_testcase!(test_cc_h, TurnDirection::CCW, 5, 5, 15, 15, 20, 8);
    turn_direction_testcase!(test_cc_i, TurnDirection::CCW, 5, 5, 15, 15, 6, 5);
    turn_direction_testcase!(test_cc_j, TurnDirection::CCW, 5, 5, 15, 15, 6, 2);
    turn_direction_testcase!(test_cw_a, TurnDirection::CW, 5, 5, 15, 5, 25, 12);
    turn_direction_testcase!(test_cw_b, TurnDirection::CW, 5, 5, 15, 5, 15, 13);
    turn_direction_testcase!(test_cw_c, TurnDirection::CW, 5, 5, 15, 5, 7, 11);
    turn_direction_testcase!(test_cw_d, TurnDirection::CW, 5, 5, 15, 5, 5, 10);
    turn_direction_testcase!(test_cw_e, TurnDirection::CW, 5, 5, 15, 5, 2, 12);
    turn_direction_testcase!(test_cw_f, TurnDirection::CW, 5, 5, 5, 15, 2, 25);
    turn_direction_testcase!(test_cw_g, TurnDirection::CW, 5, 5, 5, 15, 3, 15);
    turn_direction_testcase!(test_cw_h, TurnDirection::CW, 5, 5, 5, 15, 4, 7);
    turn_direction_testcase!(test_cw_i, TurnDirection::CW, 5, 5, 5, 15, 4, 5);
    turn_direction_testcase!(test_cw_j, TurnDirection::CW, 5, 5, 5, 15, 3, 2);
}

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Debug, PartialEq, Eq)]
struct Coord2d {
    // x grows to the right
    // y grows to the bottom
    x: i64,
    y: i64,
}

impl TryFrom<&str> for Coord2d {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut spl = value.trim().split(',');

        let x: i64 = spl.next().ok_or(())?.parse().map_err(|_| ())?;
        let y: i64 = spl.next().ok_or(())?.parse().map_err(|_| ())?;

        return Ok(Self { x, y });
    }
}

impl Coord2d {
    fn min_x(a: &mut Self, b: &Self) -> bool {
        match a.x.cmp(&b.x) {
            std::cmp::Ordering::Less => {
                return false;
            }
            std::cmp::Ordering::Equal => match a.y.cmp(&b.y) {
                std::cmp::Ordering::Less => {
                    return false;
                }
                std::cmp::Ordering::Equal => {
                    return false;
                }
                std::cmp::Ordering::Greater => {
                    *a = b.clone();
                    return true;
                }
            },
            std::cmp::Ordering::Greater => {
                *a = b.clone();
                return true;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RedTiles {
    tiles: Vec<Coord2d>,
    min_x_idx: usize,
}

impl RedTiles {
    fn convex_hull(mut self) -> Vec<Coord2d> {
        // Implemented using graham scan.

        let min_x_point = self.tiles[self.min_x_idx].clone();
        self.tiles.sort_unstable_by(|a, b| {
            let td = turn_direction(&min_x_point, a, b);
            if td == TurnDirection::Coliniar {
                return if distance_squared(&min_x_point, b) >= distance_squared(&min_x_point, a) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                };
            }
            return td.into();
        });

        let mut convex_hull: Vec<Coord2d> = Vec::with_capacity(self.tiles.len());
        for point in self.tiles {
            while convex_hull.len() > 1
                && turn_direction(
                    &convex_hull[convex_hull.len() - 2],
                    &convex_hull[convex_hull.len() - 1],
                    &point,
                ) != TurnDirection::CCW
            {
                convex_hull.pop();
            }
            convex_hull.push(point);
        }

        return convex_hull;
    }
}

#[derive(Debug, PartialEq, Eq)]
enum TurnDirection {
    CW,
    Coliniar,
    CCW,
}

impl From<TurnDirection> for std::cmp::Ordering {
    fn from(value: TurnDirection) -> Self {
        match value {
            TurnDirection::CCW => Self::Less,
            TurnDirection::Coliniar => Self::Equal,
            TurnDirection::CW => Self::Greater,
        }
    }
}

fn turn_direction(p1: &Coord2d, p2: &Coord2d, p3: &Coord2d) -> TurnDirection {
    // Implementation is a bit strange because of the coordinate system (y grows to the bottom, which is unusual).
    let cross = (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x);
    return if cross < 0 {
        TurnDirection::CCW
    } else if cross == 0 {
        TurnDirection::Coliniar
    } else {
        TurnDirection::CW
    };
}

fn parse_1_2(input: &str) -> Option<RedTiles> {
    let mut res = Vec::new();
    let mut low_idx = 0;
    let mut low = Coord2d {
        x: i64::MAX,
        y: i64::MAX,
    };

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let tile: Coord2d = line.try_into().ok()?;
        if Coord2d::min_x(&mut low, &tile) {
            low_idx = res.len();
        }

        res.push(tile);
    }

    return Some(RedTiles {
        tiles: res,
        min_x_idx: low_idx,
    });
}

fn distance_squared(p1: &Coord2d, p2: &Coord2d) -> i64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    return dx * dx + dy * dy;
}

fn part_1(input: &str) -> Option<u64> {
    let red_tiles = parse_1_2(input)?;
    let convex_hull = red_tiles.convex_hull();

    let mut largest = 0;
    let l = convex_hull.len();
    for idx_a in 0..(l - 1) {
        for idx_b in (idx_a + 1)..l {
            let dx = convex_hull[idx_a].x.abs_diff(convex_hull[idx_b].x) + 1;
            let dy = convex_hull[idx_a].y.abs_diff(convex_hull[idx_b].y) + 1;
            let size = dx * dy;
            largest = u64::max(largest, size);
        }
    }

    return Some(largest);
}

#[allow(unused)]
fn write_netbpm(path: &str, points: &Vec<Coord2d>, reds: &HashSet<usize>) {
    let mut min_x: i64 = i64::MAX;
    let mut min_y: i64 = i64::MAX;
    let mut max_x: i64 = i64::MIN;
    let mut max_y: i64 = i64::MIN;

    for c in points {
        min_x = i64::min(min_x, c.x);
        min_y = i64::min(min_y, c.y);
        max_x = i64::max(max_x, c.x);
        max_y = i64::max(max_y, c.y);
    }

    if min_x < 0 || min_y < 0 || max_x < 0 || max_y < 0 {
        panic!();
    }

    max_x += 2;
    max_y += 2;

    let mut pxls: Vec<u8> = vec![0; (max_x * max_y) as usize];

    const BLACK: u8 = 1;
    const RED: u8 = 2;

    for (idx, c) in points.iter().enumerate() {
        let pxl_idx = (max_x * c.y + c.x) as usize;

        pxls[pxl_idx] = if reds.contains(&idx) { RED } else { BLACK };
    }

    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();
    let header = format!("P3\n{0} {1}\n255\n", max_x, max_y);
    f.write_all(header.as_bytes()).unwrap();

    for pxl in pxls {
        f.write_all(match pxl {
            RED => b"255 0 0 ",
            BLACK => b"0 0 0 ",
            _ => b"255 255 255 ",
        })
        .unwrap();
    }
}

fn part_2(input: &str) -> Option<u64> {
    None
}

fn main() {
    match part_1(INPUT) {
        Some(answer) => {
            println!("part 1: {0}", answer);
        }
        None => {
            println!("part 1 failed");
        }
    }
    match part_2(INPUT) {
        Some(answer) => {
            println!("part 2: {0}", answer);
        }
        None => {
            println!("part 2 failed");
        }
    }

    println!("Done.");
}
