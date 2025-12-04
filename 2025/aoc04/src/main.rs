#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_a() {
        assert_eq!(part_1(TEST_INPUT), Some(13));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_2(TEST_INPUT), Some(43));
    }
}

const INPUT: &str = include_str!("../input.txt");

struct PaperRollDiagram {
    diagram: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl PaperRollDiagram {
    fn num_adjacient_rolls(&self, row: usize, col: usize) -> u32 {
        fn nar(row: usize, col: usize, dr: isize, dc: isize, prd: &PaperRollDiagram) -> u32 {
            let row = match dr {
                -1 => row.wrapping_sub(1),
                0 => row,
                1 => row.wrapping_add(1),
                _ => {
                    panic!();
                }
            };
            if row >= prd.height {
                return 0;
            }

            let col = match dc {
                -1 => col.wrapping_sub(1),
                0 => col,
                1 => col.wrapping_add(1),
                _ => {
                    panic!();
                }
            };
            if col >= prd.width {
                return 0;
            }

            return if prd.diagram[row][col] { 1 } else { 0 };
        }

        return nar(row, col, -1, -1, &self)
            + nar(row, col, -1, 0, &self)
            + nar(row, col, -1, 1, &self)
            + nar(row, col, 0, -1, &self)
            + nar(row, col, 0, 1, &self)
            + nar(row, col, 1, -1, &self)
            + nar(row, col, 1, 0, &self)
            + nar(row, col, 1, 1, &self);
    }

    fn remove_roll(&mut self, row: usize, col: usize) {
        self.diagram[row][col] = false;
    }
}

fn parse_1_2(input: &str) -> Option<PaperRollDiagram> {
    let mut res: Vec<Vec<bool>> = Vec::new();
    let mut line_len: usize = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line_len == 0 {
            line_len = line.len();
        } else {
            if line_len != line.len() {
                return None;
            }
        }

        let mut l: Vec<bool> = Vec::with_capacity(line_len);
        for c in line.chars() {
            match c {
                '.' => l.push(false),
                '@' => l.push(true),
                _ => {
                    return None;
                }
            }
        }

        res.push(l);
    }

    if res.is_empty() {
        return None;
    }

    let height = res.len();
    return Some(PaperRollDiagram {
        diagram: res,
        width: line_len,
        height,
    });
}

fn part_1(input: &str) -> Option<u32> {
    let prd = parse_1_2(input)?;
    let mut res = 0;

    for row in 0..prd.height {
        for col in 0..prd.width {
            if prd.diagram[row][col] && prd.num_adjacient_rolls(row, col) < 4 {
                res += 1;
            }
        }
    }

    return Some(res);
}

fn part_2(input: &str) -> Option<u32> {
    let mut prd = parse_1_2(input)?;
    let mut res = 0;

    let mut removed_some = true;
    while removed_some {
        removed_some = false;
        for row in 0..prd.height {
            for col in 0..prd.width {
                if prd.diagram[row][col] && prd.num_adjacient_rolls(row, col) < 4 {
                    res += 1;
                    removed_some = true;
                    prd.remove_roll(row, col);
                }
            }
        }
    }
    return Some(res);
}

fn main() {
    match part_1(INPUT) {
        Some(answer) => {
            println!("part 1: {0}", answer);
        }
        None => {
            println!("part 1 failed");
        }
    }
    match part_2(INPUT) {
        Some(answer) => {
            println!("part 2: {0}", answer);
        }
        None => {
            println!("part 2 failed");
        }
    }

    println!("Done.");
}
