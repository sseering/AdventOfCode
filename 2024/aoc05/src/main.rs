const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), Some(143));
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), Some(123));
}

struct Combinations2Iter<'a> {
    a_idx: usize,
    b_idx: usize,
    plen: usize,
    payload: &'a Vec<usize>,
}

impl<'a> Combinations2Iter<'a> {
    fn new(payload: &'a Vec<usize>) -> Self {
        Self {
            a_idx: 0,
            b_idx: 0,
            plen: payload.len(),
            payload,
        }
    }
}

impl<'a> Iterator for Combinations2Iter<'a> {
    type Item = (usize, usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        // I considered using .combiantions() from the itertools crate instead of this.
        // But that function returns Vec`s which can't be destrucured.
        // Which makes the syntax ugly.
        // So I roll my own.

        self.b_idx += 1;

        if self.b_idx < self.plen {
            return Some((
                self.payload[self.a_idx],
                self.a_idx,
                self.payload[self.b_idx],
                self.b_idx,
            ));
        }

        self.a_idx += 1;
        self.b_idx = self.a_idx + 1;

        if self.b_idx < self.plen {
            return Some((
                self.payload[self.a_idx],
                self.a_idx,
                self.payload[self.b_idx],
                self.b_idx,
            ));
        }

        return None;
    }
}

fn parse_1_2(print_job: &str) -> Option<(Vec<(usize, usize)>, Vec<Vec<usize>>)> {
    let mut ordering_rules: Vec<(usize, usize)> = Vec::new();
    let mut pages_lists: Vec<Vec<usize>> = Vec::new();

    for line in print_job.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if line.contains('|') {
            let (before, after) = line.split_once('|')?;
            ordering_rules.push((before.parse().ok()?, after.parse().ok()?));
        } else {
            pages_lists.push(
                line.split(',')
                    .map(|s| s.parse::<usize>().ok())
                    .collect::<Option<Vec<usize>>>()?,
            );
        }
    }

    return Some((ordering_rules, pages_lists));
}

fn part_1(print_job: &str) -> Option<usize> {
    let (ordering_rules, pages_lists) = parse_1_2(print_job)?;
    let mut res = 0;

    for pages in pages_lists {
        let plen = pages.len();

        let mut is_good = true;

        'outer: for (a, _, b, _) in Combinations2Iter::new(&pages) {
            for (before, after) in &ordering_rules {
                if b == *before && a == *after {
                    is_good = false;
                    break 'outer;
                }
            }
        }

        if is_good {
            res += pages[plen / 2]
        }
    }

    return Some(res);
}

fn part_2(print_job: &str) -> Option<usize> {
    let (ordering_rules, pages_lists) = parse_1_2(print_job)?;
    let mut res = 0;

    for mut pages in pages_lists.into_iter() {
        let plen = pages.len();

        let mut num_swaps = 0;
        let mut did_swap = true;
        while did_swap {
            did_swap = false;

            'outer: for (a, _, b, b_idx) in Combinations2Iter::new(&pages) {
                for (before, after) in &ordering_rules {
                    if b == *before && a == *after {
                        num_swaps += 1;
                        did_swap = true;
                        pages.swap(b_idx - 1, b_idx);
                        break 'outer;
                    }
                }
            }
        }

        if num_swaps > 0 {
            res += pages[plen / 2]
        }
    }

    return Some(res);
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
