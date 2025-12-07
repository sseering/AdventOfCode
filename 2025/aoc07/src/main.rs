use std::collections::VecDeque;

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_a() {
        assert_eq!(part_1(TEST_INPUT), Some(21));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_2_slow(TEST_INPUT), Some(40));
    }

    #[test]
    fn test_c() {
        assert_eq!(part_2(TEST_INPUT), Some(40));
    }
}

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct ManifoldDiagram {
    start_col: usize,
    start_row: usize, // 0 is the topmost row. The row index grows downwards.
    splitters: Vec<Vec<usize>>, // splitters[col][n] == row of the nth splitter in this col. Both dimensions are sorted.
    num_columns: usize,

    #[allow(unused)]
    num_rows: usize,
}

impl ManifoldDiagram {
    fn follow_beam_to_splitter(
        &self,
        stat_col_idx: usize,
        start_row_idx: usize,
    ) -> Option<(usize, usize)> {
        if stat_col_idx >= self.splitters.len() {
            return None;
        }
        let column_v = &self.splitters[stat_col_idx];
        let pp = column_v.partition_point(|&idx| idx <= start_row_idx);
        if pp >= column_v.len() {
            return None;
        }

        return Some((pp, column_v[pp]));
    }
}

fn parse_1_2(input: &str) -> Option<ManifoldDiagram> {
    let mut lines = input.lines();
    let mut first_line: Option<&str> = None;
    while first_line.is_none() {
        let line = lines.next()?.trim();
        if line.is_empty() {
            continue;
        }
        first_line = Some(line);
    }

    let first_line = first_line?;
    let num_cols = first_line.len();

    let mut splitters: Vec<Vec<usize>> = Vec::with_capacity(num_cols);
    for _ in 0..num_cols {
        splitters.push(Vec::new());
    }

    let mut start_col: usize = num_cols;
    let mut start_row: usize = num_cols;

    let mut handle_line = |line: &str, row_idx: usize| -> Option<()> {
        for (col_idx, c) in line.char_indices() {
            match c {
                '.' => { /* nothing to do */ }
                'S' => {
                    if start_col != num_cols {
                        return None;
                    }
                    start_col = col_idx;
                    start_row = row_idx;
                }
                '^' => {
                    let column_v = &mut splitters[col_idx];
                    let pp = column_v.partition_point(|&r| r <= row_idx);
                    column_v.insert(pp, row_idx);
                }
                _ => {
                    return None;
                }
            }
        }

        return Some(());
    };

    handle_line(first_line, 0)?;

    let mut row_idx: usize = 1;
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }

        handle_line(line, row_idx)?;

        row_idx += 1;
    }

    if start_col == num_cols {
        return None;
    }

    return Some(ManifoldDiagram {
        start_col,
        start_row,
        splitters,
        num_columns: num_cols,
        num_rows: row_idx,
    });
}

fn _part_1(diagram: &ManifoldDiagram) -> u64 {
    let mut num_splits = 0;
    let mut beams: Vec<(usize, usize)> = vec![(diagram.start_col, diagram.start_row)];
    let mut splitters_counted: Vec<Vec<bool>> = Vec::with_capacity(diagram.num_columns);
    for v in &diagram.splitters {
        splitters_counted.push(vec![false; v.len()]);
    }

    while !beams.is_empty() {
        let (c, r) = beams.pop().unwrap();

        match diagram.follow_beam_to_splitter(c, r) {
            Some((splitter_idx, splitter_row)) => {
                if splitters_counted[c][splitter_idx] {
                    continue;
                }

                num_splits += 1;
                splitters_counted[c][splitter_idx] = true;

                beams.push((c - 1, splitter_row));
                beams.push((c + 1, splitter_row));
            }
            None => {
                // nothing to do
            }
        }
    }

    return num_splits;
}

fn part_1(input: &str) -> Option<u64> {
    let md = parse_1_2(input)?;
    let res = _part_1(&md);
    return Some(res);
}

fn part_2(input: &str) -> Option<u64> {
    let diagram = parse_1_2(input)?;

    const SCORE_UNKNOWN: u64 = u64::MAX;
    let mut splitters_timeline_scores: Vec<Vec<u64>> = Vec::with_capacity(diagram.num_columns);
    for v in &diagram.splitters {
        splitters_timeline_scores.push(vec![SCORE_UNKNOWN; v.len()]);
    }

    let mut did_solve_splitter = true;
    while did_solve_splitter {
        did_solve_splitter = false;

        for (col_idx, col_v) in diagram.splitters.iter().enumerate() {
            for (vec_idx, &row_idx) in col_v.iter().enumerate() {
                if splitters_timeline_scores[col_idx][vec_idx] != SCORE_UNKNOWN {
                    continue;
                }

                let lcol = col_idx.wrapping_sub(1);
                let left_score = match diagram.follow_beam_to_splitter(lcol, row_idx) {
                    None => 1,
                    Some((hit_idx, _)) => splitters_timeline_scores[lcol][hit_idx],
                };
                let rcol = col_idx.wrapping_add(1);
                let right_score = match diagram.follow_beam_to_splitter(rcol, row_idx) {
                    None => 1,
                    Some((hit_idx, _)) => splitters_timeline_scores[rcol][hit_idx],
                };

                if left_score == SCORE_UNKNOWN || right_score == SCORE_UNKNOWN {
                    continue;
                }

                splitters_timeline_scores[col_idx][vec_idx] = left_score + right_score;
                did_solve_splitter = true;
            }
        }
    }

    return Some(splitters_timeline_scores[diagram.start_col][0]);
}

#[allow(unused)]
fn part_2_slow(input: &str) -> Option<u64> {
    let diagram = parse_1_2(input)?;

    let mut num_timelines = 1;
    let mut beams: VecDeque<(usize, usize)> =
        VecDeque::from([(diagram.start_col, diagram.start_row)]);

    while !beams.is_empty() {
        let (c, r) = beams.pop_back()?;

        if let Some((_, splitter_row)) = diagram.follow_beam_to_splitter(c, r) {
            if num_timelines % 500000 == 0 {
                println!("c {} r {} l {}", c, r, beams.len());
            }

            num_timelines += 1;

            beams.push_back((c - 1, splitter_row));
            beams.push_back((c + 1, splitter_row));
        }
    }

    return Some(num_timelines);
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
