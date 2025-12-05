#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_a() {
        assert_eq!(part_1(TEST_INPUT), Some(3));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_2(TEST_INPUT), Some(14));
    }

    #[test]
    fn test_aaa() {
        assert_eq!(
            FreshnessRange { begin: 4, end: 7 }
                .merge_ranges(&FreshnessRange { begin: 12, end: 15 }),
            None
        );
    }
    #[test]
    fn test_aab() {
        assert_eq!(
            FreshnessRange { begin: 33, end: 66 }.merge_ranges(&FreshnessRange {
                begin: 444,
                end: 555
            }),
            None
        );
    }
    #[test]
    fn test_aac() {
        assert_eq!(
            FreshnessRange { begin: 4, end: 22 }
                .merge_ranges(&FreshnessRange { begin: 6, end: 14 }),
            Some(FreshnessRange { begin: 4, end: 22 })
        );
    }
    #[test]
    fn test_aad() {
        assert_eq!(
            FreshnessRange {
                begin: 555,
                end: 5555
            }
            .merge_ranges(&FreshnessRange {
                begin: 444,
                end: 6666
            }),
            Some(FreshnessRange {
                begin: 444,
                end: 6666
            })
        );
    }
    #[test]
    fn test_aae() {
        assert_eq!(
            FreshnessRange { begin: 2, end: 10 }
                .merge_ranges(&FreshnessRange { begin: 5, end: 15 }),
            Some(FreshnessRange { begin: 2, end: 15 })
        );
    }
    #[test]
    fn test_aaf() {
        assert_eq!(
            FreshnessRange {
                begin: 55,
                end: 555
            }
            .merge_ranges(&FreshnessRange {
                begin: 200,
                end: 2222
            }),
            Some(FreshnessRange {
                begin: 55,
                end: 2222
            })
        );
    }
    #[test]
    fn test_aag() {
        assert_eq!(
            FreshnessRange { begin: 16, end: 26 }
                .merge_ranges(&FreshnessRange { begin: 3, end: 20 }),
            Some(FreshnessRange { begin: 3, end: 26 })
        );
    }
    #[test]
    fn test_aah() {
        assert_eq!(
            FreshnessRange { begin: 4, end: 44 }
                .merge_ranges(&FreshnessRange { begin: 21, end: 99 }),
            Some(FreshnessRange { begin: 4, end: 99 })
        );
    }
    #[test]
    fn test_aai() {
        assert_eq!(
            FreshnessRange {
                begin: 123,
                end: 234
            }
            .merge_ranges(&FreshnessRange {
                begin: 123,
                end: 234
            }),
            Some(FreshnessRange {
                begin: 123,
                end: 234
            })
        );
    }
    #[test]
    fn test_aaj() {
        assert_eq!(
            FreshnessRange {
                begin: 123,
                end: 234
            }
            .merge_ranges(&FreshnessRange {
                begin: 235,
                end: 500
            }),
            Some(FreshnessRange {
                begin: 123,
                end: 500
            })
        );
    }
    #[test]
    fn test_aak() {
        assert_eq!(
            FreshnessRange { begin: 55, end: 66 }
                .merge_ranges(&FreshnessRange { begin: 5, end: 55 }),
            Some(FreshnessRange { begin: 5, end: 66 })
        );
    }
    #[test]
    fn test_aal() {
        assert_eq!(
            FreshnessRange {
                begin: 7777,
                end: 7788
            }
            .merge_ranges(&FreshnessRange {
                begin: 0,
                end: 7776
            }),
            Some(FreshnessRange {
                begin: 0,
                end: 7788
            })
        );
    }
    #[test]
    fn test_aam() {
        assert_eq!(
            FreshnessRange { begin: 2, end: 3 }.merge_ranges(&FreshnessRange { begin: 4, end: 99 }),
            Some(FreshnessRange { begin: 2, end: 99 })
        );
    }
    #[test]
    fn test_aan() {
        assert_eq!(
            FreshnessRange { begin: 0, end: 100 }.merge_ranges(&FreshnessRange {
                begin: 50,
                end: 100
            }),
            Some(FreshnessRange { begin: 0, end: 100 })
        );
    }
    #[test]
    fn test_aao() {
        assert_eq!(
            FreshnessRange { begin: 33, end: 44 }
                .merge_ranges(&FreshnessRange { begin: 22, end: 44 }),
            Some(FreshnessRange { begin: 22, end: 44 })
        );
    }
    #[test]
    fn test_aap() {
        assert_eq!(
            FreshnessRange { begin: 5, end: 555 }
                .merge_ranges(&FreshnessRange { begin: 5, end: 12 }),
            Some(FreshnessRange { begin: 5, end: 555 })
        );
    }
    #[test]
    fn test_aaq() {
        assert_eq!(
            FreshnessRange { begin: 0, end: 10 }
                .merge_ranges(&FreshnessRange { begin: 0, end: 42 }),
            Some(FreshnessRange { begin: 0, end: 42 })
        );
    }
    #[test]
    fn test_aar() {
        assert_eq!(
            FreshnessRange {
                begin: 123,
                end: 222
            }
            .merge_ranges(&FreshnessRange {
                begin: 222,
                end: 444
            }),
            Some(FreshnessRange {
                begin: 123,
                end: 444
            })
        );
    }
    #[test]
    fn test_aas() {
        assert_eq!(
            FreshnessRange { begin: 0, end: 1 }
                .merge_ranges(&FreshnessRange { begin: 1, end: 100 }),
            Some(FreshnessRange { begin: 0, end: 100 })
        );
    }
    #[test]
    fn test_aat() {
        assert_eq!(
            FreshnessRange {
                begin: 22,
                end: 2222
            }
            .merge_ranges(&FreshnessRange {
                begin: 2222,
                end: 4444
            }),
            Some(FreshnessRange {
                begin: 22,
                end: 4444
            })
        );
    }
    #[test]
    fn test_aau() {
        assert_eq!(
            FreshnessRange { begin: 3, end: 5 }.merge_ranges(&FreshnessRange { begin: 5, end: 7 }),
            Some(FreshnessRange { begin: 3, end: 7 })
        );
    }
    #[test]
    fn test_aav() {
        assert_eq!(
            FreshnessRange {
                begin: 555,
                end: 666
            }
            .merge_ranges(&FreshnessRange { begin: 0, end: 0 }),
            None
        );
    }
    #[test]
    fn test_aaw() {
        assert_eq!(
            FreshnessRange { begin: 55, end: 66 }
                .merge_ranges(&FreshnessRange { begin: 54, end: 54 }),
            Some(FreshnessRange { begin: 54, end: 66 })
        );
    }
    #[test]
    fn test_aax() {
        assert_eq!(
            FreshnessRange { begin: 26, end: 44 }
                .merge_ranges(&FreshnessRange { begin: 26, end: 26 }),
            Some(FreshnessRange { begin: 26, end: 44 })
        );
    }
    #[test]
    fn test_aay() {
        assert_eq!(
            FreshnessRange { begin: 0, end: 6 }.merge_ranges(&FreshnessRange { begin: 5, end: 5 }),
            Some(FreshnessRange { begin: 0, end: 6 })
        );
    }
    #[test]
    fn test_aaz() {
        assert_eq!(
            FreshnessRange {
                begin: 222,
                end: 333
            }
            .merge_ranges(&FreshnessRange {
                begin: 333,
                end: 333
            }),
            Some(FreshnessRange {
                begin: 222,
                end: 333
            })
        );
    }
    #[test]
    fn test_aba() {
        assert_eq!(
            FreshnessRange {
                begin: 123,
                end: 345
            }
            .merge_ranges(&FreshnessRange {
                begin: 346,
                end: 346
            }),
            Some(FreshnessRange {
                begin: 123,
                end: 346
            })
        );
    }
    #[test]
    fn test_abb() {
        assert_eq!(
            FreshnessRange {
                begin: 6666,
                end: 7777
            }
            .merge_ranges(&FreshnessRange {
                begin: 9999,
                end: 9999
            }),
            None
        );
    }
    #[test]
    fn test_abc() {
        assert_eq!(
            FreshnessRange { begin: 5, end: 5 }
                .merge_ranges(&FreshnessRange { begin: 12, end: 22 }),
            None
        );
    }
    #[test]
    fn test_abd() {
        assert_eq!(
            FreshnessRange { begin: 0, end: 0 }
                .merge_ranges(&FreshnessRange { begin: 1, end: 333 }),
            Some(FreshnessRange { begin: 0, end: 333 })
        );
    }
    #[test]
    fn test_abe() {
        assert_eq!(
            FreshnessRange { begin: 2, end: 2 }
                .merge_ranges(&FreshnessRange { begin: 2, end: 222 }),
            Some(FreshnessRange { begin: 2, end: 222 })
        );
    }
    #[test]
    fn test_abf() {
        assert_eq!(
            FreshnessRange { begin: 55, end: 55 }
                .merge_ranges(&FreshnessRange { begin: 22, end: 66 }),
            Some(FreshnessRange { begin: 22, end: 66 })
        );
    }
    #[test]
    fn test_abg() {
        assert_eq!(
            FreshnessRange { begin: 99, end: 99 }
                .merge_ranges(&FreshnessRange { begin: 10, end: 99 }),
            Some(FreshnessRange { begin: 10, end: 99 })
        );
    }
    #[test]
    fn test_abh() {
        assert_eq!(
            FreshnessRange {
                begin: 100,
                end: 100
            }
            .merge_ranges(&FreshnessRange { begin: 1, end: 99 }),
            Some(FreshnessRange { begin: 1, end: 100 })
        );
    }
    #[test]
    fn test_abi() {
        assert_eq!(
            FreshnessRange { begin: 66, end: 66 }
                .merge_ranges(&FreshnessRange { begin: 2, end: 22 }),
            None
        );
    }
    #[test]
    fn test_abj() {
        assert_eq!(
            FreshnessRange { begin: 7, end: 7 }.merge_ranges(&FreshnessRange { begin: 4, end: 4 }),
            None
        );
    }
    #[test]
    fn test_abk() {
        assert_eq!(
            FreshnessRange { begin: 99, end: 99 }
                .merge_ranges(&FreshnessRange { begin: 98, end: 98 }),
            Some(FreshnessRange { begin: 98, end: 99 })
        );
    }
    #[test]
    fn test_abl() {
        assert_eq!(
            FreshnessRange { begin: 5, end: 5 }.merge_ranges(&FreshnessRange { begin: 5, end: 5 }),
            Some(FreshnessRange { begin: 5, end: 5 })
        );
    }
    #[test]
    fn test_abm() {
        assert_eq!(
            FreshnessRange {
                begin: 111,
                end: 111
            }
            .merge_ranges(&FreshnessRange {
                begin: 112,
                end: 112
            }),
            Some(FreshnessRange {
                begin: 111,
                end: 112
            })
        );
    }
    #[test]
    fn test_abn() {
        assert_eq!(
            FreshnessRange {
                begin: 5555,
                end: 5555
            }
            .merge_ranges(&FreshnessRange {
                begin: 6666,
                end: 6666
            }),
            None
        );
    }
    #[test]
    fn test_abo() {
        assert_eq!(
            FreshnessRange { begin: 22, end: 22 }
                .merge_ranges(&FreshnessRange { begin: 55, end: 55 }),
            None
        );
    }
    #[test]
    fn test_abp() {
        assert_eq!(
            FreshnessRange { begin: 0, end: 0 }.merge_ranges(&FreshnessRange { begin: 1, end: 1 }),
            Some(FreshnessRange { begin: 0, end: 1 })
        );
    }
    #[test]
    fn test_abq() {
        assert_eq!(
            FreshnessRange { begin: 0, end: 0 }.merge_ranges(&FreshnessRange { begin: 0, end: 0 }),
            Some(FreshnessRange { begin: 0, end: 0 })
        );
    }
    #[test]
    fn test_abr() {
        assert_eq!(
            FreshnessRange { begin: 5, end: 5 }.merge_ranges(&FreshnessRange { begin: 4, end: 4 }),
            Some(FreshnessRange { begin: 4, end: 5 })
        );
    }
    #[test]
    fn test_abs() {
        assert_eq!(
            FreshnessRange { begin: 99, end: 99 }
                .merge_ranges(&FreshnessRange { begin: 77, end: 77 }),
            None
        );
    }

    #[test]
    fn test_overlapping_ranges() {
        let range1 = FreshnessRange { begin: 1, end: 5 };
        let range2 = FreshnessRange { begin: 3, end: 7 };
        assert_eq!(
            range1.merge_ranges(&range2),
            Some(FreshnessRange { begin: 1, end: 7 })
        );

        let range1 = FreshnessRange { begin: 3, end: 7 };
        let range2 = FreshnessRange { begin: 1, end: 5 };
        assert_eq!(
            range1.merge_ranges(&range2),
            Some(FreshnessRange { begin: 1, end: 7 })
        );
    }

    #[test]
    fn test_adjacent_ranges() {
        let range1 = FreshnessRange { begin: 1, end: 5 };
        let range2 = FreshnessRange { begin: 5, end: 10 };
        assert_eq!(
            range1.merge_ranges(&range2),
            Some(FreshnessRange { begin: 1, end: 10 })
        );

        let range1 = FreshnessRange { begin: 5, end: 10 };
        let range2 = FreshnessRange { begin: 1, end: 5 };
        assert_eq!(
            range1.merge_ranges(&range2),
            Some(FreshnessRange { begin: 1, end: 10 })
        );
    }

    #[test]
    fn test_non_overlapping_ranges() {
        let range1 = FreshnessRange { begin: 1, end: 5 };
        let range2 = FreshnessRange { begin: 7, end: 10 };
        assert_eq!(range1.merge_ranges(&range2), None);

        let range1 = FreshnessRange { begin: 10, end: 20 };
        let range2 = FreshnessRange { begin: 1, end: 5 };
        assert_eq!(range1.merge_ranges(&range2), None);
    }

    #[test]
    fn test_identical_ranges() {
        let range1 = FreshnessRange { begin: 1, end: 5 };
        let range2 = FreshnessRange { begin: 1, end: 5 };
        assert_eq!(
            range1.merge_ranges(&range2),
            Some(FreshnessRange { begin: 1, end: 5 })
        );
    }
}

struct IngredientDatabase {
    ingredient_ids: Vec<u64>,
    fresh_ids: Vec<FreshnessRange>,
}

#[derive(PartialEq, Eq, Debug)]
struct FreshnessRange {
    // The fresh ID ranges are inclusive: the range 3-5 means that ingredient IDs 3, 4, and 5 are all fresh.
    begin: u64,
    end: u64,
}

impl FreshnessRange {
    fn contains(&self, ingredient_id: u64) -> bool {
        ingredient_id >= self.begin && ingredient_id <= self.end
    }

    fn merge_ranges(&self, other: &Self) -> Option<Self> {
        fn mr(l: &FreshnessRange, r: &FreshnessRange) -> Option<FreshnessRange> {
            if r.begin > l.end + 1 {
                return None;
            }
            return Some(FreshnessRange {
                begin: l.begin,
                end: u64::max(l.end, r.end),
            });
        }
        return if self.begin < other.begin {
            mr(self, other)
        } else {
            mr(other, self)
        };
    }

    fn size(&self) -> u64 {
        return self.end - self.begin + 1;
    }
}

fn parse_1_2(input: &str) -> Option<IngredientDatabase> {
    let mut ingredient_ids: Vec<u64> = Vec::new();
    let mut fresh_ids: Vec<FreshnessRange> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.contains('-') {
            let mut spl = line.split('-');
            let begin: u64 = spl.next()?.parse().ok()?;
            let end: u64 = spl.next()?.parse().ok()?;
            if spl.next().is_some() {
                return None;
            }
            if end < begin {
                return None;
            }
            fresh_ids.push(FreshnessRange { begin, end });
        } else {
            let ingredient_id: u64 = line.parse().ok()?;
            ingredient_ids.push(ingredient_id);
        }
    }

    return Some(IngredientDatabase {
        ingredient_ids,
        fresh_ids,
    });
}

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str) -> Option<u32> {
    let idb = parse_1_2(input)?;
    let mut res = 0;

    'outer: for ingredient in &idb.ingredient_ids {
        for range in &idb.fresh_ids {
            if range.contains(*ingredient) {
                res += 1;
                continue 'outer;
            }
        }
    }

    return Some(res);
}

fn part_2(input: &str) -> Option<u64> {
    let mut idb = parse_1_2(input)?;

    let mut did_merge = true;
    while did_merge {
        did_merge = false;

        let mut idx_a: usize = 0;
        while idx_a < idb.fresh_ids.len() - 1 {
            let mut idx_b = idx_a + 1;
            while idx_b < idb.fresh_ids.len() {
                match idb.fresh_ids[idx_a].merge_ranges(&idb.fresh_ids[idx_b]) {
                    Some(merged_range) => {
                        idb.fresh_ids.remove(idx_b);
                        idb.fresh_ids[idx_a] = merged_range;
                        did_merge = true;
                    }
                    None => {
                        idx_b += 1;
                    }
                }
            }

            idx_a += 1;
        }
    }

    return Some(idb.fresh_ids.iter().map(FreshnessRange::size).sum());
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
