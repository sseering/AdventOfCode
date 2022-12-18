use std::mem;

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

struct MonkeyLines {
    item_worry_levels: String,
    op: String,
    dest_test: String,
    dest_true: String,
    dest_false: String,
}

impl MonkeyLines {
    fn new() -> Self {
        Self {
            item_worry_levels: String::new(),
            op: String::new(),
            dest_test: String::new(),
            dest_true: String::new(),
            dest_false: String::new(),
        }
    }

    fn is_empty(&self) -> bool {
        return self.item_worry_levels.is_empty()
            && self.op.is_empty()
            && self.dest_test.is_empty()
            && self.dest_true.is_empty()
            && self.dest_false.is_empty();
    }
}

struct Monkey {
    item_worry_levels: Vec<i64>,
    op: Box<dyn Fn(i64) -> i64>,
    dest_test: i64,
    dest_true: usize,
    dest_false: usize,
    num_inspections: u32,
}

impl Monkey {
    fn from_lines(lines: &MonkeyLines) -> Option<Self> {
        let mut iwls: Vec<i64> = Vec::new();
        for split in lines.item_worry_levels.split_whitespace().skip(2) {
            let iwl: i64 = if split.contains(",") {
                split[..(split.len() - 1)].parse().ok()?
            } else {
                split.parse().ok()?
            };
            iwls.push(iwl);
        }

        let mut split = lines.op.split_whitespace().skip(4);
        let a = split.next()?;
        let b = split.next()?;

        let op: Option<Box<dyn Fn(i64) -> i64>> = if a == "*" {
            if b == "old" {
                Some(Box::new(|p| p * p))
            } else {
                let c: i64 = b.parse().ok()?;
                Some(Box::new(move |p| p * c))
            }
        } else if a == "+" {
            let c: i64 = b.parse().ok()?;
            Some(Box::new(move |p| p + c))
        } else {
            None
        };
        let op = op?;

        let dte: i64 = lines.dest_test[19..].parse().ok()?;
        let dtr: usize = lines.dest_true[25..].parse().ok()?;
        let dfa: usize = lines.dest_false[26..].parse().ok()?;

        return Some(Self {
            item_worry_levels: iwls,
            op,
            dest_test: dte,
            dest_true: dtr,
            dest_false: dfa,
            num_inspections: 0,
        });
    }

    fn exchange_iwls(&mut self) -> Vec<i64> {
        let res = mem::replace(&mut self.item_worry_levels, Vec::new());
        return res;
    }
}

struct TwoBiggest {
    a: u32,
    b: u32,
}

impl TwoBiggest {
    fn new() -> Self {
        Self {
            a: u32::MIN,
            b: u32::MIN,
        }
    }

    fn update(&mut self, val: u32) {
        if val > self.a {
            let prev = self.a;
            self.a = val;
            if prev > self.b {
                self.b = prev;
            }
        } else if val > self.b {
            self.b = val;
        }
    }

    fn part_1_2_score(&self) -> u64 {
        return (self.a as u64) * (self.b as u64);
    }
}

fn parse(monkey_notes: &str) -> Option<Vec<Monkey>> {
    let mut monkey_line_chucks: Vec<MonkeyLines> = Vec::new();
    let mut current_chunk: MonkeyLines = MonkeyLines::new();

    for line in monkey_notes.lines() {
        let line = line.trim();
        if line.len() == 0 || line.starts_with("Monkey ") {
            if !current_chunk.is_empty() {
                monkey_line_chucks.push(current_chunk);
            }
            current_chunk = MonkeyLines::new();
        } else if line.starts_with("Starting items: ") {
            current_chunk.item_worry_levels = String::from(line);
        } else if line.starts_with("Operation: new = old ") {
            current_chunk.op = String::from(line);
        } else if line.starts_with("Test: divisible by ") {
            current_chunk.dest_test = String::from(line);
        } else if line.starts_with("If true: throw to monkey ") {
            current_chunk.dest_true = String::from(line);
        } else if line.starts_with("If false: throw to monkey ") {
            current_chunk.dest_false = String::from(line);
        } else {
            return None;
        }
    }
    if !current_chunk.is_empty() {
        monkey_line_chucks.push(current_chunk);
    }

    let monkeys: Vec<Monkey> = monkey_line_chucks
        .into_iter()
        .map(|mlc| Monkey::from_lines(&mlc))
        .collect::<Option<Vec<Monkey>>>()?;

    return Some(monkeys);
}

fn part_1_monkeys(monkey_notes: &str, num_rounds: i32) -> Option<Vec<Monkey>> {
    let mut monkeys = parse(monkey_notes)?;
    let num_monkeys = monkeys.len();

    for _ in 0..num_rounds {
        for monkey_idx in 0..num_monkeys {
            let iwls = monkeys[monkey_idx].exchange_iwls();
            for iwl in iwls {
                let iwl = (monkeys[monkey_idx].op)(iwl) / 3;
                monkeys[monkey_idx].num_inspections += 1;
                let dst_idx = if iwl % monkeys[monkey_idx].dest_test == 0 {
                    monkeys[monkey_idx].dest_true
                } else {
                    monkeys[monkey_idx].dest_false
                };
                monkeys[dst_idx].item_worry_levels.push(iwl);
            }
        }
    }

    return Some(monkeys);
}

fn part_1(monkey_notes: &str, num_rounds: i32) -> Option<u64> {
    let monkeys = part_1_monkeys(monkey_notes, num_rounds)?;

    let mut tb = TwoBiggest::new();

    for m in monkeys {
        tb.update(m.num_inspections);
    }

    return Some(tb.part_1_2_score());
}

fn part_2_monkeys(monkey_notes: &str, num_rounds: i32) -> Option<Vec<Monkey>> {
    let mut monkeys = parse(monkey_notes)?;
    let num_monkeys = monkeys.len();
    let mut value_limiter: i64 = 1;

    for monkey in monkeys.iter() {
        value_limiter = num::integer::lcm(value_limiter, monkey.dest_test);
    }

    for _ in 0..num_rounds {
        for monkey_idx in 0..num_monkeys {
            let iwls = monkeys[monkey_idx].exchange_iwls();
            for iwl in iwls {
                let iwl = (monkeys[monkey_idx].op)(iwl) % value_limiter;
                monkeys[monkey_idx].num_inspections += 1;
                let dst_idx = if iwl % monkeys[monkey_idx].dest_test == 0 {
                    monkeys[monkey_idx].dest_true
                } else {
                    monkeys[monkey_idx].dest_false
                };
                monkeys[dst_idx].item_worry_levels.push(iwl);
            }
        }
    }

    return Some(monkeys);
}

fn part_2(monkey_notes: &str, num_rounds: i32) -> Option<u64> {
    let monkeys = part_2_monkeys(monkey_notes, num_rounds)?;

    let mut tb = TwoBiggest::new();

    for m in monkeys {
        tb.update(m.num_inspections);
    }

    return Some(tb.part_1_2_score());
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT, 20), Some(10605));
}

#[test]
fn test_aa() {
    if let Some(m) = part_1_monkeys(TEST_INPUT, 1) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].item_worry_levels, vec![20, 23, 27, 26]);
        assert_eq!(m[1].item_worry_levels, vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(m[2].item_worry_levels.len(), 0);
        assert_eq!(m[3].item_worry_levels.len(), 0);
    } else {
        panic!();
    }
}

#[test]
fn test_ab() {
    if let Some(m) = part_1_monkeys(TEST_INPUT, 2) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].item_worry_levels, vec![695, 10, 71, 135, 350]);
        assert_eq!(m[1].item_worry_levels, vec![43, 49, 58, 55, 362]);
        assert_eq!(m[2].item_worry_levels.len(), 0);
        assert_eq!(m[3].item_worry_levels.len(), 0);
    } else {
        panic!();
    }
}

#[test]
fn test_ac() {
    if let Some(m) = part_1_monkeys(TEST_INPUT, 3) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].item_worry_levels, vec![16, 18, 21, 20, 122]);
        assert_eq!(m[1].item_worry_levels, vec![1468, 22, 150, 286, 739]);
        assert_eq!(m[2].item_worry_levels.len(), 0);
        assert_eq!(m[3].item_worry_levels.len(), 0);
    } else {
        panic!();
    }
}

#[test]
fn test_ad() {
    if let Some(m) = part_1_monkeys(TEST_INPUT, 4) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].item_worry_levels, vec![491, 9, 52, 97, 248, 34]);
        assert_eq!(m[1].item_worry_levels, vec![39, 45, 43, 258]);
        assert_eq!(m[2].item_worry_levels.len(), 0);
        assert_eq!(m[3].item_worry_levels.len(), 0);
    } else {
        panic!();
    }
}

#[test]
fn test_ae() {
    if let Some(m) = part_1_monkeys(TEST_INPUT, 5) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].item_worry_levels, vec![15, 17, 16, 88, 1037]);
        assert_eq!(m[1].item_worry_levels, vec![20, 110, 205, 524, 72]);
        assert_eq!(m[2].item_worry_levels.len(), 0);
        assert_eq!(m[3].item_worry_levels.len(), 0);
    } else {
        panic!();
    }
}

#[test]
fn test_af() {
    if let Some(m) = part_1_monkeys(TEST_INPUT, 6) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].item_worry_levels, vec![8, 70, 176, 26, 34]);
        assert_eq!(m[1].item_worry_levels, vec![481, 32, 36, 186, 2190]);
        assert_eq!(m[2].item_worry_levels.len(), 0);
        assert_eq!(m[3].item_worry_levels.len(), 0);
    } else {
        panic!();
    }
}

#[test]
fn test_ag() {
    if let Some(m) = part_1_monkeys(TEST_INPUT, 7) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].item_worry_levels, vec![162, 12, 14, 64, 732, 17]);
        assert_eq!(m[1].item_worry_levels, vec![148, 372, 55, 72]);
        assert_eq!(m[2].item_worry_levels.len(), 0);
        assert_eq!(m[3].item_worry_levels.len(), 0);
    } else {
        panic!();
    }
}

#[test]
fn test_ah() {
    if let Some(m) = part_1_monkeys(TEST_INPUT, 8) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].item_worry_levels, vec![51, 126, 20, 26, 136]);
        assert_eq!(m[1].item_worry_levels, vec![343, 26, 30, 1546, 36]);
        assert_eq!(m[2].item_worry_levels.len(), 0);
        assert_eq!(m[3].item_worry_levels.len(), 0);
    } else {
        panic!();
    }
}

#[test]
fn test_ai() {
    if let Some(m) = part_1_monkeys(TEST_INPUT, 9) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].item_worry_levels, vec![116, 10, 12, 517, 14]);
        assert_eq!(m[1].item_worry_levels, vec![108, 267, 43, 55, 288]);
        assert_eq!(m[2].item_worry_levels.len(), 0);
        assert_eq!(m[3].item_worry_levels.len(), 0);
    } else {
        panic!();
    }
}

#[test]
fn test_aj() {
    if let Some(m) = part_1_monkeys(TEST_INPUT, 10) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].item_worry_levels, vec![91, 16, 20, 98]);
        assert_eq!(m[1].item_worry_levels, vec![481, 245, 22, 26, 1092, 30]);
        assert_eq!(m[2].item_worry_levels.len(), 0);
        assert_eq!(m[3].item_worry_levels.len(), 0);
    } else {
        panic!();
    }
}

#[test]
fn test_ak() {
    if let Some(m) = part_1_monkeys(TEST_INPUT, 15) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].item_worry_levels, vec![83, 44, 8, 184, 9, 20, 26, 102]);
        assert_eq!(m[1].item_worry_levels, vec![110, 36]);
        assert_eq!(m[2].item_worry_levels.len(), 0);
        assert_eq!(m[3].item_worry_levels.len(), 0);
    } else {
        panic!();
    }
}

#[test]
fn test_al() {
    if let Some(m) = part_1_monkeys(TEST_INPUT, 20) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].item_worry_levels, vec![10, 12, 14, 26, 34]);
        assert_eq!(m[1].item_worry_levels, vec![245, 93, 53, 199, 115]);
        assert_eq!(m[2].item_worry_levels.len(), 0);
        assert_eq!(m[3].item_worry_levels.len(), 0);
    } else {
        panic!();
    }
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT, 10000), Some(2713310158));
}

#[test]
fn test_ba() {
    if let Some(m) = part_2_monkeys(TEST_INPUT, 1) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].num_inspections, 2);
        assert_eq!(m[1].num_inspections, 4);
        assert_eq!(m[2].num_inspections, 3);
        assert_eq!(m[3].num_inspections, 6);
    } else {
        panic!();
    }
}

#[test]
fn test_bb() {
    if let Some(m) = part_2_monkeys(TEST_INPUT, 1) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].num_inspections, 2);
        assert_eq!(m[1].num_inspections, 4);
        assert_eq!(m[2].num_inspections, 3);
        assert_eq!(m[3].num_inspections, 6);
    } else {
        panic!();
    }
}

#[test]
fn test_bc() {
    if let Some(m) = part_2_monkeys(TEST_INPUT, 20) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].num_inspections, 99);
        assert_eq!(m[1].num_inspections, 97);
        assert_eq!(m[2].num_inspections, 8);
        assert_eq!(m[3].num_inspections, 103);
    } else {
        panic!();
    }
}

#[test]
fn test_bd() {
    if let Some(m) = part_2_monkeys(TEST_INPUT, 1000) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].num_inspections, 5204);
        assert_eq!(m[1].num_inspections, 4792);
        assert_eq!(m[2].num_inspections, 199);
        assert_eq!(m[3].num_inspections, 5192);
    } else {
        panic!();
    }
}

#[test]
fn test_be() {
    if let Some(m) = part_2_monkeys(TEST_INPUT, 2000) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].num_inspections, 10419);
        assert_eq!(m[1].num_inspections, 9577);
        assert_eq!(m[2].num_inspections, 392);
        assert_eq!(m[3].num_inspections, 10391);
    } else {
        panic!();
    }
}

#[test]
fn test_bf() {
    if let Some(m) = part_2_monkeys(TEST_INPUT, 3000) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].num_inspections, 15638);
        assert_eq!(m[1].num_inspections, 14358);
        assert_eq!(m[2].num_inspections, 587);
        assert_eq!(m[3].num_inspections, 15593);
    } else {
        panic!();
    }
}

#[test]
fn test_bg() {
    if let Some(m) = part_2_monkeys(TEST_INPUT, 4000) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].num_inspections, 20858);
        assert_eq!(m[1].num_inspections, 19138);
        assert_eq!(m[2].num_inspections, 780);
        assert_eq!(m[3].num_inspections, 20797);
    } else {
        panic!();
    }
}

#[test]
fn test_bh() {
    if let Some(m) = part_2_monkeys(TEST_INPUT, 5000) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].num_inspections, 26075);
        assert_eq!(m[1].num_inspections, 23921);
        assert_eq!(m[2].num_inspections, 974);
        assert_eq!(m[3].num_inspections, 26000);
    } else {
        panic!();
    }
}

#[test]
fn test_bi() {
    if let Some(m) = part_2_monkeys(TEST_INPUT, 6000) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].num_inspections, 31294);
        assert_eq!(m[1].num_inspections, 28702);
        assert_eq!(m[2].num_inspections, 1165);
        assert_eq!(m[3].num_inspections, 31204);
    } else {
        panic!();
    }
}

#[test]
fn test_bj() {
    if let Some(m) = part_2_monkeys(TEST_INPUT, 7000) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].num_inspections, 36508);
        assert_eq!(m[1].num_inspections, 33488);
        assert_eq!(m[2].num_inspections, 1360);
        assert_eq!(m[3].num_inspections, 36400);
    } else {
        panic!();
    }
}

#[test]
fn test_bk() {
    if let Some(m) = part_2_monkeys(TEST_INPUT, 8000) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].num_inspections, 41728);
        assert_eq!(m[1].num_inspections, 38268);
        assert_eq!(m[2].num_inspections, 1553);
        assert_eq!(m[3].num_inspections, 41606);
    } else {
        panic!();
    }
}

#[test]
fn test_bl() {
    if let Some(m) = part_2_monkeys(TEST_INPUT, 9000) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].num_inspections, 46945);
        assert_eq!(m[1].num_inspections, 43051);
        assert_eq!(m[2].num_inspections, 1746);
        assert_eq!(m[3].num_inspections, 46807);
    } else {
        panic!();
    }
}

#[test]
fn test_bm() {
    if let Some(m) = part_2_monkeys(TEST_INPUT, 10000) {
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].num_inspections, 52166);
        assert_eq!(m[1].num_inspections, 47830);
        assert_eq!(m[2].num_inspections, 1938);
        assert_eq!(m[3].num_inspections, 52013);
    } else {
        panic!();
    }
}

fn main() {
    println!("part 1: {}", part_1(INPUT, 20).unwrap_or(9999999));
    println!("part 2: {}", part_2(INPUT, 10000).unwrap_or(9999999));
    println!("done.");
}
