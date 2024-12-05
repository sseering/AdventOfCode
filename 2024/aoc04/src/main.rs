use std::ops::{Add, Index, Mul, Sub};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord2D {
    row: i32,
    col: i32,
}

impl Add<&Coord2D> for &Coord2D {
    type Output = Coord2D;

    fn add(self, other: &Coord2D) -> Coord2D {
        Coord2D {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl Sub<&Coord2D> for &Coord2D {
    type Output = Coord2D;

    fn sub(self, other: &Coord2D) -> Coord2D {
        Coord2D {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

impl Mul<i32> for &Coord2D {
    type Output = Coord2D;

    fn mul(self, rhs: i32) -> Coord2D {
        Coord2D {
            row: self.row * rhs,
            col: self.col * rhs,
        }
    }
}

struct LetterMatrix {
    letters: Vec<Vec<char>>,
}

impl LetterMatrix {
    fn contains(&self, c: &Coord2D) -> bool {
        return c.row >= 0
            && c.col >= 0
            && (c.row as usize) < self.letters.len()
            && (c.col as usize) < self.letters[0].len();
    }
}

impl Index<&Coord2D> for LetterMatrix {
    type Output = char;

    fn index(&self, c: &Coord2D) -> &Self::Output {
        return &self.letters[c.row as usize][c.col as usize];
    }
}

fn part_1_search_1_direction(
    letter_matrix: &LetterMatrix,
    mut pos: Coord2D,
    step: &Coord2D,
) -> usize {
    let mut num_xmas_found: usize = 0;

    while letter_matrix.contains(&pos) {
        let mut num_backteps: i32 = -1;
        match letter_matrix[&pos] {
            'X' => {
                let p1 = &pos + step;
                let p3 = &pos + &(step * 3);
                if letter_matrix.contains(&p3)
                    && letter_matrix[&p1] == 'M'
                    && letter_matrix[&(&pos + &(step * 2))] == 'A'
                    && letter_matrix[&p3] == 'S'
                {
                    num_xmas_found += 1;
                    pos = &p3 + &(step * 4);
                } else {
                    pos = &pos + &(step * 4);
                }
            }
            'M' => {
                num_backteps = 1;
            }
            'A' => {
                num_backteps = 2;
            }
            'S' => {
                num_backteps = 3;
            }
            _ => {
                pos = &pos + &(step * 4);
            }
        }
        if num_backteps != -1 {
            let back = &pos - &(step * num_backteps);
            if letter_matrix.contains(&back) && letter_matrix[&back] == 'X' {
                pos = back;
            } else {
                pos = &pos + &(step * 4);
            }
        }
    }

    return num_xmas_found;
}

fn parse_part_1_2(puzzle_input: &str) -> LetterMatrix {
    return LetterMatrix {
        letters: puzzle_input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                return line.chars().collect();
            })
            .collect(),
    };
}

fn part_1(puzzle_input: &str) -> Option<usize> {
    let letter_matrix = parse_part_1_2(puzzle_input);
    let num_rows = letter_matrix.letters.len() as i32;
    let max_row_idx = num_rows - 1;
    let num_cols = letter_matrix.letters[0].len() as i32;
    let max_col_idx = num_cols - 1;

    let mut num_xmas_found: usize = 0;

    for row in 0..num_rows {
        let step = Coord2D { row: 0, col: 1 };
        num_xmas_found += part_1_search_1_direction(&letter_matrix, Coord2D { row, col: 0 }, &step);

        let step = Coord2D { row: 0, col: -1 };
        num_xmas_found += part_1_search_1_direction(
            &letter_matrix,
            Coord2D {
                row,
                col: max_col_idx,
            },
            &step,
        );
    }

    for col in 0..num_cols {
        let step = Coord2D { row: 1, col: 0 };
        num_xmas_found += part_1_search_1_direction(&letter_matrix, Coord2D { row: 0, col }, &step);

        let step = Coord2D { row: -1, col: 0 };
        num_xmas_found += part_1_search_1_direction(
            &letter_matrix,
            Coord2D {
                row: max_row_idx,
                col,
            },
            &step,
        );
    }

    for row in 0..num_rows {
        let step = Coord2D { row: 1, col: 1 };
        num_xmas_found += part_1_search_1_direction(&letter_matrix, Coord2D { row, col: 0 }, &step);

        let step = Coord2D { row: -1, col: -1 };
        num_xmas_found += part_1_search_1_direction(
            &letter_matrix,
            Coord2D {
                row: max_row_idx - row,
                col: max_col_idx,
            },
            &step,
        );
    }

    for col in 1..num_cols {
        let step = Coord2D { row: 1, col: 1 };
        num_xmas_found += part_1_search_1_direction(&letter_matrix, Coord2D { row: 0, col }, &step);

        let step = Coord2D { row: -1, col: -1 };
        num_xmas_found += part_1_search_1_direction(
            &letter_matrix,
            Coord2D {
                row: max_row_idx,
                col: max_col_idx - col,
            },
            &step,
        );
    }

    for row in 0..num_rows {
        let step = Coord2D { row: 1, col: -1 };
        num_xmas_found += part_1_search_1_direction(
            &letter_matrix,
            Coord2D {
                row,
                col: max_col_idx,
            },
            &step,
        );

        let step = Coord2D { row: -1, col: 1 };
        num_xmas_found += part_1_search_1_direction(
            &letter_matrix,
            Coord2D {
                row: max_row_idx - row,
                col: 0,
            },
            &step,
        );
    }

    for col in 1..num_cols {
        let step = Coord2D { row: 1, col: -1 };
        num_xmas_found += part_1_search_1_direction(
            &letter_matrix,
            Coord2D {
                row: 0,
                col: max_col_idx - col,
            },
            &step,
        );

        let step = Coord2D { row: -1, col: 1 };
        num_xmas_found += part_1_search_1_direction(
            &letter_matrix,
            Coord2D {
                row: max_row_idx,
                col,
            },
            &step,
        );
    }

    return Some(num_xmas_found);
}

fn part_2_is_x_mas(
    letter_matrix: &LetterMatrix,
    pos: &Coord2D,
    a: &Coord2D,
    b: &Coord2D,
    c: &Coord2D,
    d: &Coord2D,
) -> bool {
    return letter_matrix[&(pos + a)] == 'M'
        && letter_matrix[&(pos + b)] == 'M'
        && letter_matrix[&(pos + c)] == 'S'
        && letter_matrix[&(pos + d)] == 'S';
}

fn part_2(puzzle_input: &str) -> Option<usize> {
    let letter_matrix = parse_part_1_2(puzzle_input);
    let num_rows = letter_matrix.letters.len() as i32;
    let num_cols = letter_matrix.letters[0].len() as i32;

    let tl = Coord2D { row: -1, col: -1 };
    let tr = Coord2D { row: -1, col: 1 };
    let bl = Coord2D { row: 1, col: -1 };
    let br = Coord2D { row: 1, col: 1 };
    let mut num_found = 0;
    for row in 1..(num_rows - 1) {
        for col in 1..(num_cols - 1) {
            let pos = Coord2D { row, col };
            if letter_matrix[&pos] == 'A' {
                if part_2_is_x_mas(&letter_matrix, &pos, &tl, &tr, &br, &bl)
                    || part_2_is_x_mas(&letter_matrix, &pos, &bl, &tl, &tr, &br)
                    || part_2_is_x_mas(&letter_matrix, &pos, &br, &bl, &tl, &tr)
                    || part_2_is_x_mas(&letter_matrix, &pos, &tr, &br, &bl, &tl)
                {
                    num_found += 1;
                }
            }
        }
    }
    return Some(num_found);
}

#[test]
fn test_a() {
    assert_eq!(
        part_1(
            "
..X...
.SAMX.
.A..A.
XMAS.S
.X....
"
        ),
        Some(4)
    );
}

#[test]
fn test_b() {
    assert_eq!(
        part_1(
            "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"
        ),
        Some(18)
    );
}

#[test]
fn test_c() {
    assert_eq!(
        part_1(
            "
....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
"
        ),
        Some(18)
    );
}

#[test]
fn test_d() {
    assert_eq!(
        part_1(
            "
....XXMAS.
..........
..........
..........
..........
..........
..........
..........
..........
..........
"
        ),
        Some(1)
    );
}

#[test]
fn test_e() {
    assert_eq!(
        part_2(
            "
M.S
.A.
M.S
"
        ),
        Some(1)
    );
}

#[test]
fn test_f() {
    assert_eq!(
        part_2(
            "
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
"
        ),
        Some(9)
    );
}

fn main() {
    match part_1(INPUT) {
        Some(cv) => {
            println!("Part 1: {0}.", cv);
        }
        None => {
            println!("Part 1 failed.");
        }
    }
    match part_2(INPUT) {
        Some(cv) => {
            println!("Part 2: {0}.", cv);
        }
        None => {
            println!("Part 2 failed.");
        }
    }
    println!("Done.");
}
