use std::fmt;

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

struct BingoBoard {
    rows: Vec<Vec<i32>>,
    cols: Vec<Vec<i32>>,
}

impl BingoBoard {
    fn new(line_a: &str, line_b: &str, line_c: &str, line_d: &str, line_e: &str) -> Self {
        let mut rows: Vec<Vec<i32>> = Vec::new();
        let mut cols: Vec<Vec<i32>> =
            vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        let mut nums: Vec<i32> = line_a
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        for idx in 0..5 {
            cols[idx].push(nums[idx]);
        }
        rows.push(nums);

        nums = line_b
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        for idx in 0..5 {
            cols[idx].push(nums[idx]);
        }
        rows.push(nums);

        nums = line_c
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        for idx in 0..5 {
            cols[idx].push(nums[idx]);
        }
        rows.push(nums);

        nums = line_d
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        for idx in 0..5 {
            cols[idx].push(nums[idx]);
        }
        rows.push(nums);

        nums = line_e
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        for idx in 0..5 {
            cols[idx].push(nums[idx]);
        }
        rows.push(nums);

        Self { rows, cols }
    }

    fn process_draw(&mut self, num: i32) {
        for row in self.rows.iter_mut() {
            row.retain(|&v| v != num);
        }
        for col in self.cols.iter_mut() {
            col.retain(|&v| v != num);
        }
    }

    fn part_1_score(&self, drawn_num: i32) -> Option<i32> {
        let mut cleared = false;
        for row in self.rows.iter() {
            if row.len() <= 0 {
                cleared = true;
                break;
            }
        }

        if !cleared {
            for col in self.cols.iter() {
                if col.len() <= 0 {
                    cleared = true;
                    break;
                }
            }
        }

        if cleared {
            return Some(
                self.rows
                    .iter()
                    .map::<i32, _>(|r| r.iter().sum())
                    .sum::<i32>()
                    * drawn_num,
            );
        }
        return None;
    }
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s: String = String::new();
        for (idx, row) in self.rows.iter().enumerate() {
            s = s + &format!("[{0:>3}, {1:>3}, {2:>3}, {3:>3}, {4:>3}]      [{5:>3}, {6:>3}, {7:>3}, {8:>3}, {9:>3}]\n", row[0], row[1], row[2], row[3], row[4], self.cols[0][idx], self.cols[1][idx], self.cols[2][idx], self.cols[3][idx], self.cols[4][idx]);
        }
        return writeln!(f, "{}", s);
    }
}

fn parse_bingo_data(bingo_data: &str) -> (Vec<BingoBoard>, Vec<i32>) {
    let mut lines = bingo_data.lines();
    let draws_str = lines.next().unwrap();

    let draws: Vec<i32> = draws_str
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<BingoBoard> = Vec::new();

    while lines.next().is_some() {
        let line_a: &str = lines.next().unwrap();
        let line_b: &str = lines.next().unwrap();
        let line_c: &str = lines.next().unwrap();
        let line_d: &str = lines.next().unwrap();
        let line_e: &str = lines.next().unwrap();
        boards.push(BingoBoard::new(line_a, line_b, line_c, line_d, line_e));
    }

    return (boards, draws);
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), 4512);
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), 1924);
}

fn part_1(bingo_data: &str) -> i32 {
    let (boards, draws) = parse_bingo_data(bingo_data);
    let mut boards = boards;
    for draw in draws {
        for board in boards.iter_mut() {
            board.process_draw(draw);
            if let Some(winning_score) = board.part_1_score(draw) {
                return winning_score;
            }
        }
    }
    panic!();
}

fn part_2(bingo_data: &str) -> i32 {
    let (boards, draws) = parse_bingo_data(bingo_data);
    let mut boards = boards;
    let mut draws = draws.iter();
    while boards.len() > 1 {
        let &draw = draws.next().unwrap();
        for board in boards.iter_mut() {
            board.process_draw(draw);
        }
        boards.retain(|b| b.part_1_score(draw).is_none());
    }
    if boards.len() == 0 {
        panic!();
    }
    let board = boards.iter_mut().next().unwrap();
    for &draw in draws {
        board.process_draw(draw);
        if let Some(winning_score) = board.part_1_score(draw) {
            return winning_score;
        }
    }
    panic!();
}

fn main() {
    let (boards, draws) = parse_bingo_data(TEST_INPUT);
    println!("{0:?}", draws);
    for b in boards {
        println!("{0}", b);
    }

    println!("part 1: {0}", part_1(INPUT));
    println!("part 2: {0}", part_2(INPUT));
    println!("done");
}
