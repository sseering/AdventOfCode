use std::ops::Add;

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT_A: &str = include_str!("../test-input-a.txt");

#[allow(unused)]
const TEST_INPUT_B: &str = include_str!("../test-input-b.txt");

#[derive(Copy, Clone)]
struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn r(&mut self) {
        self.x += 1;
    }

    fn l(&mut self) {
        self.x -= 1;
    }

    fn u(&mut self) {
        self.y -= 1;
    }

    fn d(&mut self) {
        self.y += 1;
    }

    fn follow(&mut self, head: &Point2D) {
        let mut dx = 0;
        let mut dy = 0;

        if head.x > self.x + 1 {
            dx = 1;
            dy = if head.y > self.y {
                1
            } else if head.y < self.y {
                -1
            } else {
                0
            };
        } else if head.x < self.x - 1 {
            dx = -1;
            dy = if head.y > self.y {
                1
            } else if head.y < self.y {
                -1
            } else {
                0
            };
        } else if head.y > self.y + 1 {
            dy = 1;
            dx = if head.x > self.x {
                1
            } else if head.x < self.x {
                -1
            } else {
                0
            };
        } else if head.y < self.y - 1 {
            dy = -1;
            dx = if head.x > self.x {
                1
            } else if head.x < self.x {
                -1
            } else {
                0
            };
        }

        self.x += dx;
        self.y += dy;
    }
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

struct RopeState {
    head: Point2D,
    knots: Vec<Point2D>,
    seen: Vec<Vec<bool>>,
    seen_offset: Point2D,
}

impl RopeState {
    const GROW_BY: usize = 5;

    fn new(num_knots_following_tail: usize) -> Self {
        let mut seen = vec![vec![false; 5]; 5];
        seen[0][0] = true;
        let knots = vec![Point2D::new(); num_knots_following_tail];
        return Self {
            head: Point2D::new(),
            knots: knots,
            seen: seen,
            seen_offset: Point2D::new(),
        };
    }

    fn grow_seen(&mut self, access_point: &Point2D) {
        let x_size = self.seen.len();
        let y_size = self.seen.first().unwrap().len();
        if access_point.x < 0 {
            for _ in 0..RopeState::GROW_BY {
                self.seen.insert(0, vec![false; y_size]);
            }
            self.seen_offset.x += RopeState::GROW_BY as i32;
        } else if (access_point.x as usize) >= x_size {
            for _ in 0..RopeState::GROW_BY {
                self.seen.push(vec![false; y_size]);
            }
        }
        if access_point.y < 0 {
            let new_len = y_size + RopeState::GROW_BY;
            for sub_vec in self.seen.iter_mut() {
                let mut new_vec = Vec::with_capacity(new_len);
                new_vec.extend([false; RopeState::GROW_BY].iter());
                new_vec.append(sub_vec);
                *sub_vec = new_vec;
            }
            self.seen_offset.y += RopeState::GROW_BY as i32;
        } else if (access_point.y as usize) >= y_size {
            for sub_vec in self.seen.iter_mut() {
                sub_vec.extend([false; RopeState::GROW_BY].iter());
            }
        }
    }

    fn process_move(&mut self, move_sub_handler: fn(&mut Point2D), num_steps: u32) {
        for _ in 0..num_steps {
            move_sub_handler(&mut self.head);

            self.knots.first_mut().unwrap().follow(&self.head);

            let num_following_knots = self.knots.len();
            for idx_a in 0..(num_following_knots - 1) {
                // we can't use .windows(2) here as we need to change the vector contents
                let a: Point2D = self.knots[idx_a];
                let b: &mut Point2D = &mut self.knots[idx_a + 1];
                b.follow(&a);
            }

            let seen_pos: Point2D = self.knots.last().unwrap() + &self.seen_offset;

            self.grow_seen(&seen_pos);
            let seen_pos: Point2D = self.knots.last().unwrap() + &self.seen_offset; // growing might have changed seen_offset, so we need to recalculate this
            self.seen[seen_pos.x as usize][seen_pos.y as usize] = true;
        }
    }

    fn r(&mut self, num_steps: u32) {
        self.process_move(Point2D::r, num_steps);
    }

    fn l(&mut self, num_steps: u32) {
        self.process_move(Point2D::l, num_steps);
    }

    fn u(&mut self, num_steps: u32) {
        self.process_move(Point2D::u, num_steps);
    }

    fn d(&mut self, num_steps: u32) {
        self.process_move(Point2D::d, num_steps);
    }

    fn score_part_1(&self) -> u32 {
        return self
            .seen
            .iter()
            .map(|sub_vec| {
                sub_vec
                    .iter()
                    .map(|&seen_val| if seen_val { 1 } else { 0 })
                    .sum::<u32>()
            })
            .sum();
    }
}

fn part_1_2(num_knots_following_tail: usize, motions: &str) -> Option<u32> {
    let mut state = RopeState::new(num_knots_following_tail);

    for line in motions.lines() {
        let mut split = line.split_whitespace();
        let direction = split.next()?;
        let num_steps: u32 = split.next()?.parse().ok()?;

        match direction {
            "R" => {
                state.r(num_steps);
            }
            "L" => {
                state.l(num_steps);
            }
            "U" => {
                state.u(num_steps);
            }
            "D" => {
                state.d(num_steps);
            }
            _ => {
                return None;
            }
        }
    }
    return Some(state.score_part_1());
}

fn part_1(motions: &str) -> Option<u32> {
    return part_1_2(1, motions);
}

fn part_2(motions: &str) -> Option<u32> {
    return part_1_2(9, motions);
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT_A), Some(13));
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT_A), Some(1));
}

#[test]
fn test_c() {
    assert_eq!(part_2(TEST_INPUT_B), Some(36));
}

fn main() {
    // let mut test: Vec<Vec<bool>> = Vec::new();
    // for _ in 0..5 {
    // test.push(vec![false; 10]);
    // }
    // for idx in [1, 3] {
    // test[idx].push(true);
    // }
    // for sub in test {
    // println!("{}", sub.len());
    // }

    // println!("-----------");

    // for idx in [1, 3] {
    // test2[idx].push(true);
    // }
    // for sub in test2 {
    // println!("{}", sub.len());
    // }

    // println!("--------size-");

    // let mut test3: Vec<Vec<bool>> = vec![vec![false; 10]; 5];
    // for idx in [1, 3] {
    // test3[idx].puvec![Point2D::new(); num_knots_following_tail];
    // }
    // let mut test_idx: i32 = -1;
    // test_idx += 1;
    // if test_idx != 1 && test_idx != 3 {
    // *sub = Vec::new();
    // }
    // }
    // for sub in test3 {

    //
    //
    //
    //
    // // println!("{}", sub.len());
    // }

    // let mut test3: Vec<Point2D> = vec![Point2D::new(); 5];
    // for idx in [1, 3] {
    // test3[idx].x = 31337;
    // }
    // for sub in test3 {
    // println!("{}", sub.x);
    // }

    //let mut rust = ['r', 'u', 's', 't'];

    //for mut win in rust.windows(2) {
    //win = &['a', 'b'];
    //let (a, b) = if let [a, b] = win { (a, b) } else { panic!() };
    //println!("{} {}", a, b);
    //}

    //for mut win in rust.windows(2) {
    //let (a, b) = if let [a, b] = win { (a, b) } else { panic!() };
    //println!("{} {}", a, b);
    //}

    println!("part 1: {}", part_1(INPUT).unwrap_or(99999));
    println!("part 2: {}", part_2(INPUT).unwrap_or(99999));
    println!("done.");
}
