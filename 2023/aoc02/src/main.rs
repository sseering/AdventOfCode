use regex::Regex;

#[allow(unused)]
const TEST_INPUT_A: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
#[allow(unused)]
const TEST_INPUT_B: &str = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
#[allow(unused)]
const TEST_INPUT_C: &str =
    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
#[allow(unused)]
const TEST_INPUT_D: &str =
    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
#[allow(unused)]
const TEST_INPUT_E: &str = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

impl Hand {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn set(&mut self, color: String, num: u32) -> bool {
        if color == "red" {
            if self.red != 0 {
                return false;
            }
            self.red = num;
            return true;
        }
        if color == "green" {
            if self.green != 0 {
                return false;
            }
            self.green = num;
            return true;
        }
        if color == "blue" {
            if self.blue != 0 {
                return false;
            }
            self.blue = num;
            return true;
        }
        return false;
    }
}

#[derive(PartialEq, Eq, Debug)]
struct GameRecord {
    game_id: i32,
    hands: Vec<Hand>,
}

impl GameRecord {
    fn part_1_possile(&self) -> bool {
        for hand in self.hands.iter() {
            if hand.red > 12 {
                return false;
            }
            if hand.green > 13 {
                return false;
            }
            if hand.blue > 14 {
                return false;
            }
        }

        return true;
    }

    fn power_set(&self) -> u32 {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for hand in self.hands.iter() {
            red = u32::max(red, hand.red);
            green = u32::max(green, hand.green);
            blue = u32::max(blue, hand.blue);
        }

        return red * green * blue;
    }
}

fn parse_game_record(gr: &str) -> Option<GameRecord> {
    let start_re = Regex::new(r"Game\s+(\d+):\s+(\d+)\s+(red|blue|green)").ok()?;
    let repeating_re = Regex::new(r"((,|;)\s+(\d+)\s+(red|blue|green))").ok()?;

    let start_caps = start_re.captures(gr)?;
    let game_id: i32 = start_caps[1].to_string().parse().ok()?;
    let mut res = GameRecord {
        game_id,
        hands: Vec::new(),
    };

    let mut hand = Hand::new();

    let mut num: u32 = start_caps[2].to_string().parse().ok()?;
    let mut color: String = start_caps[3].to_string();
    if !hand.set(color, num) {
        return None;
    }

    for cap in repeating_re.captures_iter(gr) {
        let next_hand = cap.get(2)?.as_str() == ";";
        if next_hand {
            res.hands.push(hand);
            hand = Hand::new();
        }

        num = cap.get(3).unwrap().as_str().parse().unwrap();
        color = String::from(cap.get(4).unwrap().as_str());
        if !hand.set(color, num) {
            return None;
        }
    }

    res.hands.push(hand);

    return Some(res);
}

fn part_1(game_records: &str) -> Option<i32> {
    let mut res = 0;
    for line in game_records.lines() {
        let gr = parse_game_record(line)?;
        if gr.part_1_possile() {
            res += gr.game_id;
        }
    }

    return Some(res);
}

fn part_2(game_records: &str) -> Option<u32> {
    let mut res = 0;
    for line in game_records.lines() {
        let gr = parse_game_record(line)?;
        res += gr.power_set();
    }

    return Some(res);
}

#[test]
fn test_aa() {
    assert_eq!(
        parse_game_record(TEST_INPUT_A),
        Some(GameRecord {
            game_id: 1,
            hands: vec![
                Hand {
                    red: 4,
                    green: 0,
                    blue: 3
                },
                Hand {
                    red: 1,
                    green: 2,
                    blue: 6
                },
                Hand {
                    red: 0,
                    green: 2,
                    blue: 0
                }
            ]
        })
    );
}

#[test]
fn test_ab() {
    assert_eq!(
        parse_game_record(TEST_INPUT_B),
        Some(GameRecord {
            game_id: 2,
            hands: vec![
                Hand {
                    red: 0,
                    green: 2,
                    blue: 1
                },
                Hand {
                    red: 1,
                    green: 3,
                    blue: 4
                },
                Hand {
                    red: 0,
                    green: 1,
                    blue: 1
                }
            ]
        })
    );
}

#[test]
fn test_ac() {
    assert_eq!(
        parse_game_record(TEST_INPUT_C),
        Some(GameRecord {
            game_id: 3,
            hands: vec![
                Hand {
                    red: 20,
                    green: 8,
                    blue: 6
                },
                Hand {
                    red: 4,
                    green: 13,
                    blue: 5
                },
                Hand {
                    red: 1,
                    green: 5,
                    blue: 0
                }
            ]
        })
    );
}

#[test]
fn test_ad() {
    assert_eq!(
        parse_game_record(TEST_INPUT_D),
        Some(GameRecord {
            game_id: 4,
            hands: vec![
                Hand {
                    red: 3,
                    green: 1,
                    blue: 6
                },
                Hand {
                    red: 6,
                    green: 3,
                    blue: 0
                },
                Hand {
                    red: 14,
                    green: 3,
                    blue: 15
                }
            ]
        })
    );
}

#[test]
fn test_ae() {
    assert_eq!(
        parse_game_record(TEST_INPUT_E),
        Some(GameRecord {
            game_id: 5,
            hands: vec![
                Hand {
                    red: 6,
                    green: 3,
                    blue: 1
                },
                Hand {
                    red: 1,
                    green: 2,
                    blue: 2
                }
            ]
        })
    );
}

#[test]
fn test_af() {
    assert_eq!(
        parse_game_record(TEST_INPUT_A).map(|gr| gr.part_1_possile()),
        Some(true)
    );
}

#[test]
fn test_ag() {
    assert_eq!(
        parse_game_record(TEST_INPUT_B).map(|gr| gr.part_1_possile()),
        Some(true)
    );
}

#[test]
fn test_ah() {
    assert_eq!(
        parse_game_record(TEST_INPUT_C).map(|gr| gr.part_1_possile()),
        Some(false)
    );
}

#[test]
fn test_ai() {
    assert_eq!(
        parse_game_record(TEST_INPUT_D).map(|gr| gr.part_1_possile()),
        Some(false)
    );
}

#[test]
fn test_aj() {
    assert_eq!(
        parse_game_record(TEST_INPUT_E).map(|gr| gr.part_1_possile()),
        Some(true)
    );
}

#[test]
fn test_ak() {
    assert_eq!(
        parse_game_record(TEST_INPUT_A).map(|gr| gr.power_set()),
        Some(48)
    );
}

#[test]
fn test_al() {
    assert_eq!(
        parse_game_record(TEST_INPUT_B).map(|gr| gr.power_set()),
        Some(12)
    );
}

#[test]
fn test_am() {
    assert_eq!(
        parse_game_record(TEST_INPUT_C).map(|gr| gr.power_set()),
        Some(1560)
    );
}

#[test]
fn test_an() {
    assert_eq!(
        parse_game_record(TEST_INPUT_D).map(|gr| gr.power_set()),
        Some(630)
    );
}

#[test]
fn test_ao() {
    assert_eq!(
        parse_game_record(TEST_INPUT_E).map(|gr| gr.power_set()),
        Some(36)
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
    println!("done.");
}
