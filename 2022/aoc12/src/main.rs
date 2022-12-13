use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Add;

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point2D {
    x: usize,
    y: usize,
}

impl Point2D {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
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

#[derive(Eq, PartialEq)]
struct Point2DWithDist {
    p: Point2D,
    d: i32,
}

impl Point2DWithDist {
    fn new(p: Point2D, d: i32) -> Self {
        Self { p, d }
    }
}

impl Ord for Point2DWithDist {
    fn cmp(&self, other: &Self) -> Ordering {
        let res = self.d.cmp(&other.d);
        if res != Ordering::Equal {
            return res;
        }
        let res = self.p.x.cmp(&other.p.x);
        if res != Ordering::Equal {
            return res;
        }
        return self.p.y.cmp(&other.p.y);
    }
}

impl PartialOrd for Point2DWithDist {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// #[derive(Copy, Clone)]
// struct Color {
//     r: u8,
//     g: u8,
//     b: u8,
// }
//
// impl Color {
//     fn path() -> Self {
//         Self { r: 255, g: 0, b: 0 }
//     }
//
//     fn start() -> Self {
//         Self {
//             r: 255,
//             g: 255,
//             b: 0,
//         }
//     }
//
//     fn end() -> Self {
//         Self { r: 0, g: 255, b: 0 }
//     }
//
//     fn discovered(height: u8) -> Self {
//         Self {
//             r: 0,
//             g: 0,
//             b: (255 / 28) * (2 + height),
//         }
//     }
//
//     fn unseen(height: u8) -> Self {
//         let v: u8 = (255 / 26) * height;
//         Self { r: v, g: v, b: v }
//     }
// }

fn neighbors(
    from: &Point2D,
    height_map: &Vec<Vec<u8>>,
    done: &Vec<Vec<bool>>,
    width: usize,
    height: usize,
) -> impl Iterator<Item = Point2D> {
    let from_height_p1 = height_map[from.y][from.x] + 1;

    let mut res: Vec<Point2D> = Vec::new();

    let dest = from + &Point2D::new(1, 0);
    if dest.x < width {
        if !done[dest.y][dest.x] && from_height_p1 >= height_map[dest.y][dest.x] {
            res.push(dest);
        }
    }

    let dest = from + &Point2D::new(0, 1);
    if dest.y < height {
        if !done[dest.y][dest.x] && from_height_p1 >= height_map[dest.y][dest.x] {
            res.push(dest);
        }
    }

    if from.x > 0 {
        let dest = Point2D::new(from.x - 1, from.y);
        if !done[dest.y][dest.x] && from_height_p1 >= height_map[dest.y][dest.x] {
            res.push(dest);
        }
    }

    if from.y > 0 {
        let dest = Point2D::new(from.x, from.y - 1);
        if !done[dest.y][dest.x] && from_height_p1 >= height_map[dest.y][dest.x] {
            res.push(dest);
        }
    }

    return res.into_iter();
}

fn part_1(height_map: &str) -> Option<i32> {
    let mut start: Option<Point2D> = None;
    let mut end: Option<Point2D> = None;

    let height_map: Vec<Vec<u8>> = height_map
        .lines()
        .enumerate()
        .map(|(y_idx, line)| -> Option<Vec<u8>> {
            return line
                .chars()
                .enumerate()
                .map(|(x_idx, c)| match c {
                    'S' => {
                        start = Some(Point2D::new(x_idx, y_idx));
                        Some(0)
                    }
                    'E' => {
                        end = Some(Point2D::new(x_idx, y_idx));
                        Some(('z' as u8) - ('a' as u8))
                    }
                    'a'..='z' => Some((u32::from(c) - u32::from('a')) as u8),

                    _ => None,
                })
                .collect();
        })
        .collect::<Option<Vec<Vec<u8>>>>()?;

    let start = start?;
    let end = end?;

    let width = height_map.first()?.len();
    let height = height_map.len();

    let mut dist = vec![vec![i32::MAX; width]; height];
    dist[start.y][start.x] = 0;

    let mut done = vec![vec![false; width]; height];
    let mut prev = vec![vec![Point2D::new(0, 0); width]; height];
    let mut queue: BinaryHeap<Reverse<Point2DWithDist>> = BinaryHeap::new();
    queue.push(Reverse(Point2DWithDist::new(start, 0)));

    while let Some(current) = queue.pop() {
        let current = current.0.p;
        if done[current.y][current.x] {
            continue;
        }
        let new_dist = dist[current.y][current.x] + 1;
        for neigh in neighbors(&current, &height_map, &done, width, height) {
            if new_dist < dist[neigh.y][neigh.x] {
                dist[neigh.y][neigh.x] = new_dist;
                prev[neigh.y][neigh.x] = current;
            }
            queue.push(Reverse(Point2DWithDist::new(neigh, dist[neigh.y][neigh.x])));
        }
        done[current.y][current.x] = true;
        if current == end {
            break;
        }
    }

    // let mut img = vec![vec![Color::path(); width]; height];
    // for y in 0..height {
    //     for x in 0..width {
    //         img[y][x] = if done[y][x] {
    //             Color::discovered(height_map[y][x])
    //         } else {
    //             Color::unseen(height_map[y][x])
    //         };
    //     }
    // }
    // let mut walk = prev[end.y][end.x];
    // while walk != start {
    //     img[walk.y][walk.x] = Color::path();
    //     walk = prev[walk.y][walk.x];
    // }
    // img[start.y][start.x] = Color::start();
    // img[end.y][end.x] = Color::end();
    // println!("");
    // println!("");
    // println!("P3");
    // println!("{} {}", width, height);
    // println!("255");
    // for y in 0..height {
    //     for x in 0..width {
    //         print!("{} {} {} ", img[y][x].r, img[y][x].g, img[y][x].b);
    //     }
    //     println!("");
    // }
    // println!("");
    // println!("");

    return Some(dist[end.y][end.x]);
}

fn part_2(height_map: &str) -> Option<i32> {
    return None;
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), Some(31));
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), None);
}

fn main() {
    println!("part 1: {}", part_1(INPUT).unwrap_or(9999999));
    println!("part 2: {}", part_2(INPUT).unwrap_or(9999999));
    println!("done.");
}
