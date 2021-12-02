#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn forward(&mut self, distance: i32, part_1: bool) {
        self.horizontal += distance;
        if !part_1 {
            self.depth += distance * self.aim;
        }
    }

    fn up(&mut self, distance: i32, part_1: bool) {
        if part_1 {
            self.depth -= distance;
        } else {
            self.aim -= distance;
        }
    }

    fn down(&mut self, distance: i32, part_1: bool) {
        if part_1 {
            self.depth += distance;
        } else {
            self.aim += distance;
        }
    }

    fn day_2_score(&self) -> i32 {
        self.depth * self.horizontal
    }
}

fn day_2_part_1(course: &str) -> i32 {
    return day_2(course, true);
}

fn day_2_part_2(course: &str) -> i32 {
    return day_2(course, false);
}

fn day_2(course: &str, part_1: bool) -> i32 {
    let mut submarine = Position::new();

    course.lines().for_each(|s| {
        let mut split = s.split(' ');
        if let Some(direction) = split.next() {
            if let Some(distance) = split.next() {
                let distance = distance.parse::<i32>().unwrap();
                match direction {
                    "forward" => {
                        submarine.forward(distance, part_1);
                    }
                    "down" => {
                        submarine.down(distance, part_1);
                    }
                    "up" => {
                        submarine.up(distance, part_1);
                    }
                    _ => {}
                }
            }
        }
    });

    return submarine.day_2_score();
}

#[test]
fn test_a() {
    assert_eq!(day_2_part_1(TEST_INPUT), 150);
}

#[test]
fn test_b() {
    assert_eq!(day_2_part_2(TEST_INPUT), 900);
}

fn main() {
    println!("part 1: {}", day_2_part_1(INPUT));
    println!("part 2: {}", day_2_part_2(INPUT));
    println!("done");
}
