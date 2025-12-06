#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn test_a() {
        assert_eq!(
            match parse_1(TEST_INPUT) {
                None => None,
                Some(ws) => Some(part_1_2(&ws)),
            },
            Some(4277556)
        );
    }

    #[test]
    fn test_b() {
        assert_eq!(
            match parse_2(TEST_INPUT) {
                None => None,
                Some(ws) => Some(part_1_2(&ws)),
            },
            Some(3263827)
        );
    }
}

const INPUT: &str = include_str!("../input.txt");

#[derive(Copy, Clone, Debug)]
enum MathsHomeworkOp {
    Add,
    Mul,
}

impl TryFrom<&str> for MathsHomeworkOp {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "+" {
            return Ok(MathsHomeworkOp::Add);
        }
        if value == "*" {
            return Ok(MathsHomeworkOp::Mul);
        }
        return Err(());
    }
}

impl TryFrom<char> for MathsHomeworkOp {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value == '+' {
            return Ok(MathsHomeworkOp::Add);
        }
        if value == '*' {
            return Ok(MathsHomeworkOp::Mul);
        }
        return Err(());
    }
}

#[derive(Debug)]
struct MathsWorksheet {
    ops: Vec<MathsHomeworkOp>,
    numbers: Vec<Vec<u64>>,
}

fn parse_1(input: &str) -> Option<MathsWorksheet> {
    let mut lines = input.lines();
    let mut first_line: Option<&str> = None;
    while first_line.is_none() {
        let line = lines.next()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        first_line = Some(line);
    }

    let spl: Vec<&str> = first_line?.split_whitespace().collect();
    let num_maths_problems = spl.len();
    let mut numbers = Vec::with_capacity(num_maths_problems);
    for _ in 0..num_maths_problems {
        numbers.push(Vec::new());
    }
    let mut ops = Vec::with_capacity(num_maths_problems);

    fn handle_line(
        spl: &Vec<&str>,
        ops: &mut Vec<MathsHomeworkOp>,
        numbers: &mut Vec<Vec<u64>>,
    ) -> Option<()> {
        let first: char = spl[0].chars().next()?;
        if first == '+' || first == '*' {
            for &op_str in spl {
                let op: MathsHomeworkOp = op_str.try_into().ok()?;
                ops.push(op);
            }
        } else {
            for (idx, &num_str) in spl.iter().enumerate() {
                let num: u64 = num_str.parse().ok()?;
                numbers[idx].push(num);
            }
        }

        return Some(());
    }

    handle_line(&spl, &mut ops, &mut numbers)?;

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let spl: Vec<&str> = line.split_whitespace().collect();
        handle_line(&spl, &mut ops, &mut numbers)?;
    }

    return Some(MathsWorksheet { ops, numbers });
}

fn part_1_2(worksheet: &MathsWorksheet) -> u64 {
    let mut res = 0;

    for (idx, nums) in worksheet.numbers.iter().enumerate() {
        let op = worksheet.ops[idx];
        let mut work: u64 = match op {
            MathsHomeworkOp::Add => 0,
            MathsHomeworkOp::Mul => 1,
        };
        for num in nums {
            work = match op {
                MathsHomeworkOp::Add => work + num,
                MathsHomeworkOp::Mul => work * num,
            };
        }

        res += work;
    }

    return res;
}

fn parse_2(input: &str) -> Option<MathsWorksheet> {
    let mut lines = input.lines();
    let mut first_line: Option<&str> = None;
    while first_line.is_none() {
        let line = lines.next()?;
        if line.trim().is_empty() {
            continue;
        }
        first_line = Some(line);
    }
    let first_line = first_line?;

    let mut all_numbers: Vec<u64> = vec![0; first_line.len()];

    fn handle_number_line(all_numbers: &mut Vec<u64>, line: &str) -> Option<()> {
        for (idx, c) in line.chars().enumerate() {
            match c {
                ' ' => { /* nothing to do */ }
                '0' => {
                    all_numbers[idx] = all_numbers[idx] * 10 + 0;
                }
                '1' => {
                    all_numbers[idx] = all_numbers[idx] * 10 + 1;
                }
                '2' => {
                    all_numbers[idx] = all_numbers[idx] * 10 + 2;
                }
                '3' => {
                    all_numbers[idx] = all_numbers[idx] * 10 + 3;
                }
                '4' => {
                    all_numbers[idx] = all_numbers[idx] * 10 + 4;
                }
                '5' => {
                    all_numbers[idx] = all_numbers[idx] * 10 + 5;
                }
                '6' => {
                    all_numbers[idx] = all_numbers[idx] * 10 + 6;
                }
                '7' => {
                    all_numbers[idx] = all_numbers[idx] * 10 + 7;
                }
                '8' => {
                    all_numbers[idx] = all_numbers[idx] * 10 + 8;
                }
                '9' => {
                    all_numbers[idx] = all_numbers[idx] * 10 + 9;
                }
                _ => {
                    return None;
                }
            }
        }
        return Some(());
    }

    handle_number_line(&mut all_numbers, first_line)?;

    let mut ops: Vec<(usize, MathsHomeworkOp)> = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        if ops.len() != 0 {
            return None;
        }

        let mut chars = line.chars().peekable();
        let first = *chars.peek()?;

        if first == '+' || first == '*' {
            for (idx, c) in chars.enumerate() {
                if c == ' ' {
                    /* nothing to do */
                    continue;
                }
                ops.push((idx, c.try_into().ok()?));
            }
        } else {
            handle_number_line(&mut all_numbers, line)?;
        }
    }

    let num_ops = ops.len();
    let mut numbers: Vec<Vec<u64>> = Vec::with_capacity(num_ops);
    for _ in 0..num_ops {
        numbers.push(Vec::new());
    }

    for idx in 0..num_ops {
        let i1 = idx + 1;
        let start = ops[idx].0;
        let end = if i1 < num_ops {
            ops[i1].0 - 1
        } else {
            all_numbers.len()
        };

        for i in start..end {
            numbers[idx].push(all_numbers[i]);
        }
    }
    return Some(MathsWorksheet {
        numbers,
        ops: ops.into_iter().map(|p| p.1).collect(),
    });
}

fn main() {
    match parse_1(INPUT) {
        Some(ws) => {
            println!("part 1: {0}", part_1_2(&ws));
        }
        None => {
            println!("part 1 failed");
        }
    }
    match parse_2(INPUT) {
        Some(ws) => {
            println!("part 2: {0}", part_1_2(&ws));
        }
        None => {
            println!("part 2 failed");
        }
    }
    println!("Done.");
}
