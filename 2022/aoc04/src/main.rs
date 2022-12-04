use std::str::FromStr;

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

struct IdAssigment {
    s: i32,
    e: i32,
}

impl IdAssigment {
    fn other_fully_contained(&self, other: &IdAssigment) -> bool {
        return other.s >= self.s && other.e <= self.e;
    }

    fn other_overlaps(&self, other: &IdAssigment) -> bool {
        return (self.s <= other.s && other.s <= self.e)
            || (self.s <= other.e && other.e <= self.e)
            || (other.s <= self.e && self.e <= other.e)
            || (other.s <= self.e && self.e <= other.e);
    }
}

impl FromStr for IdAssigment {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut s = input.split('-');
        let a = s.next().ok_or(())?;
        let b = s.next().ok_or(())?;
        let a: i32 = a.parse().map_err(|_| ())?;
        let b: i32 = b.parse().map_err(|_| ())?;
        Ok(IdAssigment { s: a, e: b })
    }
}

struct IdAssigmentPair {
    a: IdAssigment,
    b: IdAssigment,
}

impl IdAssigmentPair {
    fn new(aa: IdAssigment, bb: IdAssigment) -> Self {
        Self { a: aa, b: bb }
    }

    fn fully_containment_score(&self) -> i32 {
        // return (if self.a.other_fully_contained(&self.b) {
        //     1
        // } else {
        //     0
        // }) + (if self.b.other_fully_contained(&self.a) {
        //     1
        // } else {
        //     0
        // });
        return if self.a.other_fully_contained(&self.b) || self.b.other_fully_contained(&self.a) {
            1
        } else {
            0
        };
    }

    fn overlap_score(&self) -> i32 {
        return if self.a.other_overlaps(&self.b) { 1 } else { 0 };
    }
}

fn part_1(id_assignments: &str) -> Option<i32> {
    id_assignments
        .lines()
        .map(|line| -> Option<i32> {
            let mut s = line.split(',');
            let a = s.next()?;
            let b = s.next()?;

            let a = IdAssigment::from_str(a).ok()?;
            let b = IdAssigment::from_str(b).ok()?;

            return Some(IdAssigmentPair::new(a, b).fully_containment_score());
        })
        .sum()
}

fn part_2(id_assignments: &str) -> Option<i32> {
    id_assignments
        .lines()
        .map(|line| -> Option<i32> {
            let mut s = line.split(',');
            let a = s.next()?;
            let b = s.next()?;

            let a = IdAssigment::from_str(a).ok()?;
            let b = IdAssigment::from_str(b).ok()?;

            return Some(IdAssigmentPair::new(a, b).overlap_score());
        })
        .sum()
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), Some(2));
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), Some(4));
}

fn main() {
    println!("part 1: {}", part_1(INPUT).unwrap());
    println!("part 2: {}", part_2(INPUT).unwrap());
    println!("done.");
}
