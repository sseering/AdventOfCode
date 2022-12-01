#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

fn part_1(calories: &str) -> u32 {
    let mut current_cals: u32 = 0;
    let mut max_cals: u32 = 0;

    for line in calories.lines() {
        if line.len() == 0 {
            if current_cals > max_cals {
                max_cals = current_cals;
            }
            current_cals = 0;
        } else {
            current_cals += line.parse::<u32>().unwrap();
        }
    }
    if current_cals > max_cals {
        max_cals = current_cals;
    }

    return max_cals;
}

#[derive(Debug)]
struct ThreeBigCals {
    a: u32,
    b: u32,
    c: u32,
}

impl ThreeBigCals {
    fn new() -> Self {
        Self { a: 0, b: 0, c: 0 }
    }

    fn update(&mut self, cals: u32) {
        let mut cals = cals;
        if cals > self.a {
            (self.a, cals) = (cals, self.a);
        }
        if cals > self.b {
            (self.b, cals) = (cals, self.b);
        }
        if cals > self.c {
            self.c = cals;
        }
    }

    fn part_2_cals(&self) -> u32 {
        return self.a + self.b + self.c;
    }
}

fn part_2(calories: &str) -> u32 {
    let mut tbc = ThreeBigCals::new();

    let mut current_cals: u32 = 0;

    for line in calories.lines() {
        if line.len() == 0 {
            tbc.update(current_cals);
            current_cals = 0;
        } else {
            current_cals += line.parse::<u32>().unwrap();
        }
    }
    tbc.update(current_cals);

    return tbc.part_2_cals();
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), 24000);
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), 45000);
}

fn main() {
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
    println!("done.");
}
