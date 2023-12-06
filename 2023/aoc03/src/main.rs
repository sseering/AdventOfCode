use std::collections::HashMap;

#[allow(unused)]
const TEST_INPUT: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

struct EngineSchematicNumber {
    value: u32,
    row: usize, // postition of the leftmost digit, topmost row is idx 0
    col: usize, // postition of the leftmost digit, leftmost col is idx 0
    len: usize, // in decimal digits
}

impl EngineSchematicNumber {
    fn has_value(&self) -> bool {
        self.len > 0
    }

    fn push_digit(&mut self, d: u32, row: usize, col: usize) -> Option<()> {
        if self.len == 0 {
            if d == 0 {
                // numbers shouldnt be allowed to start with a 0?
                return None;
            }
            self.row = row;
            self.col = col;
        }

        self.value = self.value * 10 + d;
        self.len += 1;
        return Some(());
    }
}

impl EngineSchematicNumber {
    fn new() -> Self {
        Self {
            value: 0,
            row: 0,
            col: 0,
            len: 0,
        }
    }

    fn overlaps(&self, c: &Coord2d) -> bool {
        return self.row == c.row && c.col >= self.col && c.col < (self.col + self.len);
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coord2d {
    row: usize, // topmost row is idx 0
    col: usize, // leftmost col is idx 0
}

struct EngineSymbol {
    pos: Coord2d,
    symbol: char,
}

impl EngineSymbol {
    fn new(row: usize, col: usize, symbol: char) -> Self {
        Self {
            pos: Coord2d { row, col },
            symbol,
        }
    }

    fn adjacent(&self, width: usize, height: usize) -> Vec<Coord2d> {
        let mut res = Vec::new();
        let p = &self.pos;

        for drow in (-1_isize)..2 {
            for dcol in (-1_isize)..2 {
                if drow == 0 && dcol == 0 {
                    continue;
                }

                let nr = (p.row as isize) + drow;
                if nr < 0 {
                    continue;
                }
                let nr = nr as usize;
                if nr >= width {
                    continue;
                }

                let nc = (p.col as isize) + dcol;
                if nc < 0 {
                    continue;
                }
                let nc = nc as usize;
                if nc >= height {
                    continue;
                }

                res.push(Coord2d { row: nr, col: nc });
            }
        }

        return res;
    }
}

struct EngineSchematic {
    engine_symbols: Vec<EngineSymbol>,
    found_nums: Vec<EngineSchematicNumber>,
    width: usize,
    height: usize,
}

fn parse_engine_schematic(es: &str) -> Option<EngineSchematic> {
    let mut next_num = EngineSchematicNumber::new();
    let mut found_nums: Vec<EngineSchematicNumber> = Vec::new();
    let mut engine_symbols: Vec<EngineSymbol> = Vec::new();

    let mut width = 0;
    let mut height = 0;
    for (row, line) in es.lines().enumerate() {
        width = line.len();
        height += 1;
        for (col, char) in line.chars().enumerate() {
            match char {
                '.' => {
                    if next_num.has_value() {
                        found_nums.push(next_num);
                        next_num = EngineSchematicNumber::new();
                    }
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    next_num.push_digit(char.to_digit(10)?, row, col)?;
                }
                _ => {
                    if next_num.has_value() {
                        found_nums.push(next_num);
                        next_num = EngineSchematicNumber::new();
                    }
                    engine_symbols.push(EngineSymbol::new(row, col, char))
                }
            }
        }
        if next_num.has_value() {
            found_nums.push(next_num);
            next_num = EngineSchematicNumber::new();
        }
    }

    return Some(EngineSchematic {
        engine_symbols,
        found_nums,
        width,
        height,
    });
}

fn part_1(es: &str) -> Option<u32> {
    let mut pes = parse_engine_schematic(es)?;

    let mut engine_part_numbers: Vec<EngineSchematicNumber> = Vec::new();

    for es in pes.engine_symbols {
        for adj in es.adjacent(pes.width, pes.height) {
            let mut idx = 0;
            while idx < pes.found_nums.len() {
                if pes.found_nums[idx].overlaps(&adj) {
                    let epn = pes.found_nums.remove(idx);
                    engine_part_numbers.push(epn);
                } else {
                    idx += 1;
                }
            }
        }
    }

    let res: u32 = engine_part_numbers.iter().map(|epn| epn.value).sum();

    return Some(res);
}

fn part_2(es: &str) -> Option<u32> {
    let mut pes = parse_engine_schematic(es)?;

    let mut gears: HashMap<Coord2d, Vec<EngineSchematicNumber>> = HashMap::new();

    for es in pes.engine_symbols {
        if es.symbol != '*' {
            continue;
        }

        for adj in es.adjacent(pes.width, pes.height) {
            let mut idx = 0;
            while idx < pes.found_nums.len() {
                if pes.found_nums[idx].overlaps(&adj) {
                    let epn = pes.found_nums.remove(idx);

                    gears.entry(es.pos.clone()).or_insert(Vec::new()).push(epn);
                } else {
                    idx += 1;
                }
            }
        }
    }

    let res: u32 = gears
        .values()
        .filter_map(|nums| {
            if nums.len() != 2 {
                None
            } else {
                Some(nums[0].value * nums[1].value)
            }
        })
        .sum();

    return Some(res);
}

#[test]
fn test_aa() {
    assert_eq!(part_1(TEST_INPUT), Some(4361));
}

#[test]
fn test_ab() {
    assert_eq!(part_2(TEST_INPUT), Some(467835));
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
    println!("done");
}
