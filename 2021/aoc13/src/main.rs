use std::cmp::max;

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

struct FoldInstr {
    where_: i32,
    fold_along_y: bool,
}

impl FoldInstr {
    fn new(where_: i32, fold_along_y: bool) -> Self {
        Self {
            where_,
            fold_along_y,
        }
    }

    fn apply_fold(&self, x: usize, y: usize) -> (usize, usize) {
        // println!("applying {} {}", self.fold_along_y, self.where_);
        let nx = if self.fold_along_y {
            x
        } else {
            let d = max(0, (x as i32) - self.where_);
            ((x as i32) - 2 * d) as usize
        };

        let ny = if self.fold_along_y {
            let d = max(0, (y as i32) - self.where_);
            ((y as i32) - 2 * d) as usize
        } else {
            y
        };

        return (nx, ny);
    }
}

fn part_1(paper_and_fold: &str) -> u32 {
    return part_1_2(paper_and_fold, true);
}

fn part_2(paper_and_fold: &str) -> u32 {
    return part_1_2(paper_and_fold, false);
}

fn part_1_2(paper_and_fold: &str, part_1: bool) -> u32 {
    let mut dots: Vec<(usize, usize)> = Vec::new();
    let mut folds: Vec<FoldInstr> = Vec::new();
    let mut parsing_folds = false;
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    for line in paper_and_fold.lines() {
        if line.is_empty() {
            parsing_folds = true;
            continue;
        }

        if parsing_folds {
            let l = "fold along ".len();
            let fold_along_y = match &line[l..l + 1] {
                "y" => true,
                "x" => false,
                _ => panic!("not x or y"),
            };

            let l = "fold along y=".len();
            let where_: i32 = line[l..].parse().unwrap();
            folds.push(FoldInstr::new(where_, fold_along_y));

            continue;
        }
        if let [x_str, y_str] = line.split(',').collect::<Vec<&str>>()[..] {
            let x: usize = x_str.parse().unwrap();
            let y: usize = y_str.parse().unwrap();
            max_x = max(max_x, x);
            max_y = max(max_y, y);
            dots.push((x, y));
        } else {
            panic!("coordinate parsing error");
        }
    }

    let mut folded: Vec<Vec<bool>> = Vec::new();
    for _ in 0..=max_y {
        folded.push(vec![false; max_x + 1]);
    }

    let mut num_dots_after_fold = 0;
    let mut max_dotted_x: usize = 0;
    let mut max_dotted_y: usize = 0;
    for dot in &dots {
        let mut x = dot.0;
        let mut y = dot.1;

        for fi in &folds {
            // destructuring assignment here is only available in rust unstable, so we don't use it
            let new_dot = fi.apply_fold(x, y);
            x = new_dot.0;
            y = new_dot.1;

            if part_1 {
                break;
            }
        }

        max_dotted_x = max(max_dotted_x, x);
        max_dotted_y = max(max_dotted_y, y);
        if !folded[y][x] {
            num_dots_after_fold += 1;
        }
        folded[y][x] = true;
    }

    if !part_1 {
        for y in 0..=max_dotted_y {
            for x in 0..=max_dotted_x {
                print!("{}", if folded[y][x] { '█' } else { ' ' });
            }
            println!("");
        }
    }

    return num_dots_after_fold;
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), 17);
}

fn main() {
    // let mut map: [[bool; 20]; 20] = [[false; 20]; 20];

    // for (x, y) in [
    //     (6, 10),
    //     (0, 14),
    //     (9, 10),
    //     (0, 3),
    //     (10, 4),
    //     (4, 11),
    //     (6, 0),
    //     (6, 12),
    //     (4, 1),
    //     (0, 13),
    //     (10, 12),
    //     (3, 4),
    //     (3, 0),
    //     (8, 4),
    //     (1, 10),
    //     (2, 14),
    //     (8, 10),
    //     (9, 0),
    // ] {
    //     let y0 = y;
    //     let d1 = max(0, (y0 as i32) - 7);
    //     let y1 = y0 - 2 * d1;

    //     let d2 = max(0, (y1 as i32) - 5);
    //     let y2 = y1 - 2 * d2;

    //     let yy2 = (y0 - max(0, 2 * y0 - 14)) - max(0, 2 * y0 - max(0, 2 * y0 - 14) - 5);

    //     map[y1 as usize][x] = true;
    // }

    // for y in 0..20 {
    //     for x in 0..20 {
    //         print!("{}", if map[y][x] { '#' } else { '.' });
    //     }
    //     println!("");
    // }

    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
    println!("Done.");
}
