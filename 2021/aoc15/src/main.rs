#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[derive(PartialEq, Eq)]
struct Point2D {
    x: usize,
    y: usize,
}

impl Point2D {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn neighbors(&self, width: usize, height: usize) -> Vec<Point2D> {
        let mut result: Vec<Point2D> = Vec::new();

        if self.x > 0 {
            result.push(Point2D::new(self.x - 1, self.y));
        }

        if self.y > 0 {
            result.push(Point2D::new(self.x, self.y - 1));
        }

        let nx = self.x + 1;
        if nx < width {
            result.push(Point2D::new(nx, self.y));
        }

        let ny = self.y + 1;
        if ny < height {
            result.push(Point2D::new(self.x, ny));
        }

        return result;
    }
}

struct SimpleMinQueue {
    buckets: Vec<Vec<Point2D>>,
    smallest_nonempty_bucket: usize,
    num_elements: usize,
}

impl SimpleMinQueue {
    // Navive (but good enough) implementation of a min priority queue.
    //
    // I don't want to use external crates as that feels like cheating the challenge so I had to roll my own.
    fn new() -> Self {
        Self {
            buckets: Vec::new(),
            smallest_nonempty_bucket: usize::MAX,
            num_elements: 0,
        }
    }

    fn add_point_with_dist(&mut self, p: Point2D, distance: u16) {
        let distance: usize = distance as usize;
        let mut l = self.buckets.len();
        while distance >= l {
            self.buckets.push(Vec::new());
            l += 1;
        }

        self.buckets[distance].push(p);
        if distance < self.smallest_nonempty_bucket {
            self.smallest_nonempty_bucket = distance;
        }
        self.num_elements += 1;
    }

    fn pop_min(&mut self) -> (Point2D, u16) {
        let distance: u16 = self.smallest_nonempty_bucket as u16;
        let p = self.buckets[self.smallest_nonempty_bucket].pop().unwrap();
        self.num_elements -= 1;

        if self.num_elements <= 0 {
            self.smallest_nonempty_bucket = usize::MAX;
        } else {
            while self.buckets[self.smallest_nonempty_bucket].len() <= 0 {
                self.smallest_nonempty_bucket += 1;
            }
        }

        return (p, distance);
    }

    fn decrease_distance(&mut self, p: &Point2D, old_dist: u16, new_dist: u16) {
        if new_dist >= old_dist {
            panic!();
        }
        let old_dist: usize = old_dist as usize;
        let new_dist: usize = new_dist as usize;
        let to_remove = self.buckets[old_dist]
            .iter()
            .position(|in_bucket| in_bucket == p)
            .unwrap();
        let reinsert_this = self.buckets[old_dist].swap_remove(to_remove);
        self.buckets[new_dist].push(reinsert_this);
        if new_dist < self.smallest_nonempty_bucket {
            self.smallest_nonempty_bucket = new_dist;
        }
    }

    fn not_empty(&self) -> bool {
        return self.num_elements > 0;
    }
}

fn parse_risk_map(risk_map: &str) -> (Vec<Vec<u8>>, usize, usize) {
    let mut risk_levels: Vec<Vec<u8>> = Vec::new();

    let mut width: usize = 0;
    for line in risk_map.lines() {
        width = line.len();
        risk_levels.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }
    let height = risk_levels.len();

    return (risk_levels, width, height);
}

fn part_1(risk_map: &str) -> u16 {
    // Solve with Dijkstra's algorithm, variant using a min priotiry queue

    let (risk_levels, width, height) = parse_risk_map(risk_map);
    let mut distances: Vec<Vec<u16>> = Vec::new();
    let mut min_dist_positions: SimpleMinQueue = SimpleMinQueue::new();

    for _ in 0..height {
        distances.push(vec![u16::MAX; width]);
    }
    distances[0][0] = 0;
    min_dist_positions.add_point_with_dist(Point2D::new(0, 0), 0);

    while min_dist_positions.not_empty() {
        let (current, dist_of_current) = min_dist_positions.pop_min();
        for neigh in current.neighbors(width, height) {
            let old_dist = distances[neigh.y][neigh.x];
            let new_dist = dist_of_current + (risk_levels[neigh.y][neigh.x] as u16);

            if new_dist < old_dist {
                distances[neigh.y][neigh.x] = new_dist;
                if old_dist == u16::MAX {
                    min_dist_positions.add_point_with_dist(neigh, new_dist);
                } else {
                    min_dist_positions.decrease_distance(&neigh, old_dist, new_dist);
                }
            }
        }
    }

    return distances[height - 1][width - 1];
}

#[allow(unused)]
fn part_2_read_risk(
    p: &Point2D,
    base_width: usize,
    base_height: usize,
    risk_levels: &Vec<Vec<u8>>,
) -> u8 {
    let nx = p.x % base_width;
    let ny = p.y % base_height;
    let add_x: u8 = (p.x / base_width) as u8;
    let add_y: u8 = (p.y / base_height) as u8;

    return 1 + ((risk_levels[ny][nx] + add_x + add_y - 1) % 9);
}

#[allow(unused)]
fn part_2(risk_map: &str) -> u16 {
    // Solve with Dijkstra's algorithm, variant using a min priotiry queue

    let (risk_levels, base_width, base_height) = parse_risk_map(risk_map);
    let width = 5 * base_width;
    let height = 5 * base_height;
    let mut distances: Vec<Vec<u16>> = Vec::new();
    let mut min_dist_positions: SimpleMinQueue = SimpleMinQueue::new();

    for _ in 0..height {
        distances.push(vec![u16::MAX; width]);
    }
    distances[0][0] = 0;

    let mut current: (usize, usize) = (0, 0);
    min_dist_positions.add_point_with_dist(Point2D::new(0, 0), 0);

    while min_dist_positions.not_empty() {
        let (current, dist_of_current) = min_dist_positions.pop_min();
        for neigh in current.neighbors(width, height) {
            let risk = part_2_read_risk(&neigh, base_width, base_height, &risk_levels);
            let old_dist = distances[neigh.y][neigh.x];
            let new_dist = dist_of_current + (risk as u16);

            if new_dist < old_dist {
                distances[neigh.y][neigh.x] = new_dist;
                if old_dist == u16::MAX {
                    min_dist_positions.add_point_with_dist(neigh, new_dist);
                } else {
                    min_dist_positions.decrease_distance(&neigh, old_dist, new_dist);
                }
            }
        }
    }

    return distances[height - 1][width - 1];
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), 40);
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), 315);
}

fn main() {
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
    println!("Done");
}
