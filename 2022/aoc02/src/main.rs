use std::str::FromStr;

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

enum HalfTurn {
    Rock,
    Paper,
    Scissors,
}

impl HalfTurn {
    fn points(&self) -> u32 {
        match self {
            HalfTurn::Rock => 1,
            HalfTurn::Paper => 2,
            HalfTurn::Scissors => 3,
        }
    }
}

enum WinLossDraw {
    Win,
    Loss,
    Draw,
}

impl FromStr for WinLossDraw {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "X" => Ok(WinLossDraw::Loss),
            "Y" => Ok(WinLossDraw::Draw),
            "Z" => Ok(WinLossDraw::Win),
            _ => Err(()),
        }
    }
}

impl WinLossDraw {
    fn points(&self) -> u32 {
        match self {
            WinLossDraw::Loss => 0,
            WinLossDraw::Draw => 3,
            WinLossDraw::Win => 6,
        }
    }

    fn to_half_turn(&self, opponent: &HalfTurn) -> HalfTurn {
        match opponent {
            HalfTurn::Rock => match self {
                WinLossDraw::Draw => HalfTurn::Rock,
                WinLossDraw::Loss => HalfTurn::Scissors,
                WinLossDraw::Win => HalfTurn::Paper,
            },
            HalfTurn::Paper => match self {
                WinLossDraw::Win => HalfTurn::Scissors,
                WinLossDraw::Draw => HalfTurn::Paper,
                WinLossDraw::Loss => HalfTurn::Rock,
            },

            HalfTurn::Scissors => match self {
                WinLossDraw::Loss => HalfTurn::Paper,
                WinLossDraw::Win => HalfTurn::Rock,
                WinLossDraw::Draw => HalfTurn::Scissors,
            },
        }
    }
}

impl FromStr for HalfTurn {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" => Ok(HalfTurn::Rock),
            "B" => Ok(HalfTurn::Paper),
            "C" => Ok(HalfTurn::Scissors),
            "X" => Ok(HalfTurn::Rock),
            "Y" => Ok(HalfTurn::Paper),
            "Z" => Ok(HalfTurn::Scissors),
            _ => Err(()),
        }
    }
}

struct Turn {
    a: HalfTurn,
    b: HalfTurn,
}

impl Turn {
    fn new(strategy_guide_line: &str, part1: bool) -> Self {
        let mut split = strategy_guide_line.split_whitespace();
        let aa = HalfTurn::from_str(split.next().unwrap()).unwrap();
        let bb = if part1 {
            HalfTurn::from_str(split.next().unwrap()).unwrap()
        } else {
            WinLossDraw::from_str(split.next().unwrap())
                .unwrap()
                .to_half_turn(&aa)
        };
        return Self { a: aa, b: bb };
    }

    fn win_loss_draw_points(&self) -> u32 {
        (match self.b {
            HalfTurn::Rock => match self.a {
                HalfTurn::Rock => WinLossDraw::Draw,
                HalfTurn::Paper => WinLossDraw::Loss,
                HalfTurn::Scissors => WinLossDraw::Win,
            },
            HalfTurn::Paper => match self.a {
                HalfTurn::Rock => WinLossDraw::Win,
                HalfTurn::Paper => WinLossDraw::Draw,
                HalfTurn::Scissors => WinLossDraw::Loss,
            },

            HalfTurn::Scissors => match self.a {
                HalfTurn::Rock => WinLossDraw::Loss,
                HalfTurn::Paper => WinLossDraw::Win,
                HalfTurn::Scissors => WinLossDraw::Draw,
            },
        })
        .points()
    }
}

fn part_1(strategy_guide: &str) -> u32 {
    return strategy_guide
        .lines()
        .map(|line| Turn::new(line, true))
        .map(|turn| turn.win_loss_draw_points() + turn.b.points())
        .sum();
}

fn part_2(strategy_guide: &str) -> u32 {
    return strategy_guide
        .lines()
        .map(|line| Turn::new(line, false))
        .map(|turn| turn.win_loss_draw_points() + turn.b.points())
        .sum();
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), 15);
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), 12);
}

fn main() {
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
    println!("done.");
}
