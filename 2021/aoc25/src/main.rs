#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[derive(Eq, PartialEq, Clone, Debug)]
enum Location {
    Empty,
    SouthCumber,
    EastCumber,
}

impl Location {
    fn from_char(c: char) -> Self {
        match c {
            '>' => Self::EastCumber,
            'v' => Self::SouthCumber,
            '.' => Self::Empty,
            _ => panic!(),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Seafloor {
    locations: Vec<Vec<Location>>,
    width: usize,
    height: usize,
}

impl Seafloor {
    fn new(scan: &str) -> Self {
        let mut lines = scan.lines();
        let line_a = lines.next().unwrap();
        let width = line_a.len();

        let mut locations = Vec::with_capacity(width);
        locations.push(line_a.chars().map(Location::from_char).collect());

        for line in lines {
            if line.len() != width {
                panic!();
            }
            locations.push(line.chars().map(Location::from_char).collect());
        }

        let height = locations.len();

        Self {
            locations,
            width,
            height,
        }
    }

    fn move_cucumbers(&mut self) -> bool {
        let mut had_movement = false;

        let old = self.locations.clone();
        for y in 0..self.height {
            let sublist = &old[y];
            for x in 0..self.width {
                if sublist[x] != Location::EastCumber {
                    continue;
                }
                let next = (x + 1) % self.width;
                if sublist[next] == Location::Empty {
                    let l = &mut self.locations[y];
                    l[x] = Location::Empty;
                    l[next] = Location::EastCumber;
                    had_movement = true;
                }
            }
        }

        let old = self.locations.clone();
        for y in 0..self.height {
            let sublist = &old[y];
            let next = (y + 1) % self.height;
            let next_sublist = &old[next];
            for x in 0..self.width {
                if sublist[x] != Location::SouthCumber {
                    continue;
                }
                if next_sublist[x] == Location::Empty {
                    self.locations[y][x] = Location::Empty;
                    self.locations[next][x] = Location::SouthCumber;
                    had_movement = true;
                }
            }
        }

        return had_movement;
    }
}
fn part_1(scan: &str) -> u32 {
    let mut seafloor = Seafloor::new(scan);
    let mut num_moves = 1;

    while seafloor.move_cucumbers() {
        num_moves += 1;
    }

    return num_moves;
}

#[test]
fn test_a_1() {
    let a_str = include_str!("../test-input-a.txt");
    let b_str = include_str!("../test-input-a-1.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    assert_eq!(a.move_cucumbers(), true);
    assert_eq!(a, b);
}

#[test]
fn test_b_1() {
    let a_str = include_str!("../test-input-b.txt");
    let b_str = include_str!("../test-input-b-1.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    assert_eq!(a.move_cucumbers(), true);
    assert_eq!(a, b);
}

#[test]
fn test_b_2() {
    let a_str = include_str!("../test-input-b.txt");
    let b_str = include_str!("../test-input-b-2.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    assert_eq!(a.move_cucumbers(), true);
    assert_eq!(a.move_cucumbers(), true);
    assert_eq!(a, b);
}

#[test]
fn test_b_3() {
    let a_str = include_str!("../test-input-b.txt");
    let b_str = include_str!("../test-input-b-3.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    assert_eq!(a.move_cucumbers(), true);
    assert_eq!(a.move_cucumbers(), true);
    assert_eq!(a.move_cucumbers(), true);
    assert_eq!(a, b);
}

#[test]
fn test_b_4() {
    let a_str = include_str!("../test-input-b.txt");
    let b_str = include_str!("../test-input-b-4.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    assert_eq!(a.move_cucumbers(), true);
    assert_eq!(a.move_cucumbers(), true);
    assert_eq!(a.move_cucumbers(), true);
    assert_eq!(a.move_cucumbers(), true);
    assert_eq!(a, b);
}

#[test]
fn test_c_1() {
    let a_str = include_str!("../test-input-c.txt");
    let b_str = include_str!("../test-input-c-1.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    assert_eq!(a.move_cucumbers(), true);
    assert_eq!(a, b);
}

#[test]
fn test_c_2() {
    let a_str = include_str!("../test-input-c.txt");
    let b_str = include_str!("../test-input-c-2.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    for _ in 0..2 {
        assert_eq!(a.move_cucumbers(), true);
    }
    assert_eq!(a, b);
}

#[test]
fn test_c_3() {
    let a_str = include_str!("../test-input-c.txt");
    let b_str = include_str!("../test-input-c-3.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    for _ in 0..3 {
        assert_eq!(a.move_cucumbers(), true);
    }
    assert_eq!(a, b);
}

#[test]
fn test_c_4() {
    let a_str = include_str!("../test-input-c.txt");
    let b_str = include_str!("../test-input-c-4.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    for _ in 0..4 {
        assert_eq!(a.move_cucumbers(), true);
    }
    assert_eq!(a, b);
}

#[test]
fn test_c_5() {
    let a_str = include_str!("../test-input-c.txt");
    let b_str = include_str!("../test-input-c-5.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    for _ in 0..5 {
        assert_eq!(a.move_cucumbers(), true);
    }
    assert_eq!(a, b);
}

#[test]
fn test_c_10() {
    let a_str = include_str!("../test-input-c.txt");
    let b_str = include_str!("../test-input-c-10.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    for _ in 0..10 {
        assert_eq!(a.move_cucumbers(), true);
    }
    assert_eq!(a, b);
}

#[test]
fn test_c_20() {
    let a_str = include_str!("../test-input-c.txt");
    let b_str = include_str!("../test-input-c-20.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    for _ in 0..20 {
        assert_eq!(a.move_cucumbers(), true);
    }
    assert_eq!(a, b);
}

#[test]
fn test_c_30() {
    let a_str = include_str!("../test-input-c.txt");
    let b_str = include_str!("../test-input-c-30.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    for _ in 0..30 {
        assert_eq!(a.move_cucumbers(), true);
    }
    assert_eq!(a, b);
}

#[test]
fn test_c_40() {
    let a_str = include_str!("../test-input-c.txt");
    let b_str = include_str!("../test-input-c-40.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    for _ in 0..40 {
        assert_eq!(a.move_cucumbers(), true);
    }
    assert_eq!(a, b);
}

#[test]
fn test_c_50() {
    let a_str = include_str!("../test-input-c.txt");
    let b_str = include_str!("../test-input-c-50.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    for _ in 0..50 {
        assert_eq!(a.move_cucumbers(), true);
    }
    assert_eq!(a, b);
}

#[test]
fn test_c_55() {
    let a_str = include_str!("../test-input-c.txt");
    let b_str = include_str!("../test-input-c-55.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    for _ in 0..55 {
        assert_eq!(a.move_cucumbers(), true);
    }
    assert_eq!(a, b);
}

#[test]
fn test_c_56() {
    let a_str = include_str!("../test-input-c.txt");
    let b_str = include_str!("../test-input-c-56.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    for _ in 0..56 {
        assert_eq!(a.move_cucumbers(), true);
    }
    assert_eq!(a, b);
}

#[test]
fn test_c_57() {
    let a_str = include_str!("../test-input-c.txt");
    let b_str = include_str!("../test-input-c-57.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    for _ in 0..57 {
        assert_eq!(a.move_cucumbers(), true);
    }
    assert_eq!(a, b);
}

#[test]
fn test_c_58() {
    let a_str = include_str!("../test-input-c.txt");
    let b_str = include_str!("../test-input-c-58.txt");
    let mut a = Seafloor::new(a_str);
    let b = Seafloor::new(b_str);
    for _ in 0..57 {
        assert_eq!(a.move_cucumbers(), true);
    }

    assert_eq!(a.move_cucumbers(), false);
    assert_eq!(a, b);
}

#[test]
fn test_d() {
    let a_str = include_str!("../test-input-c.txt");
    assert_eq!(part_1(a_str), 58);
}

fn main() {
    println!("part 1: {}", part_1(INPUT));
    println!("Done.");
}
