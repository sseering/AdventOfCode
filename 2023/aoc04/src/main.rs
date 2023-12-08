use std::collections::HashSet;

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

struct Card {
    card_id: usize,
    winning_numbers: HashSet<u32>,
    numbers_card_owns: Vec<u32>,
}

fn parse_card(line: &str) -> Option<Card> {
    let mut s1 = line.split(':');
    let mut s11 = s1.next()?.split_whitespace();
    if s11.next()? != "Card" {
        return None;
    }
    let card_id: usize = s11.next()?.parse().ok()?;

    let mut s2 = s1.next()?.split('|');

    let winning_numbers: HashSet<u32> = s2
        .next()?
        .split_whitespace()
        .map(|v| v.parse().ok())
        .collect::<Option<HashSet<u32>>>()?;

    let numbers_i_have: Vec<u32> = s2
        .next()?
        .split_whitespace()
        .map(|v| v.parse().ok())
        .collect::<Option<Vec<u32>>>()?;

    return Some(Card {
        card_id,
        winning_numbers,
        numbers_card_owns: numbers_i_have,
    });
}

fn parse_card_pile(cards: &str) -> Option<Vec<Card>> {
    let mut res: Vec<Card> = Vec::new();
    for line in cards.lines() {
        let card = parse_card(line)?;

        if card.card_id != res.len() + 1 {
            return None;
        }

        res.push(card);
    }

    return Some(res);
}

fn part_1_points(card: &Card) -> u32 {
    let mut res = 0;
    for nih in &card.numbers_card_owns {
        if card.winning_numbers.contains(&nih) {
            if res == 0 {
                res = 1;
            } else {
                res *= 2;
            }
        }
    }

    return res;
}

fn part_1(cards: &str) -> Option<u32> {
    let cards = parse_card_pile(cards)?;
    let mut res = 0;

    for card in cards {
        res += part_1_points(&card);
    }

    return Some(res);
}

fn part_2(cards: &str) -> Option<u32> {
    let cards = parse_card_pile(cards)?;
    let num_cards = cards.len();
    let mut multiples: Vec<u32> = vec![1; num_cards];

    for (idx, card) in cards.iter().enumerate() {
        let num_matching = card
            .numbers_card_owns
            .iter()
            .filter(|nco| card.winning_numbers.contains(&nco))
            .count();

        let current = multiples[idx];
        for increment in 1..=num_matching {
            let update_idx = idx + increment;
            if update_idx < num_cards {
                multiples[update_idx] += current;
            }
        }
    }

    return Some(multiples.iter().sum());
}

#[test]
fn test_aa() {
    let card = parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
    assert_eq!(card.is_some(), true);
    assert_eq!(part_1_points(&card.unwrap()), 8);
}

#[test]
fn test_ab() {
    let card = parse_card("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
    assert_eq!(card.is_some(), true);

    assert_eq!(part_1_points(&card.unwrap()), 2);
}

#[test]
fn test_ac() {
    let card = parse_card("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");
    assert_eq!(card.is_some(), true);
    assert_eq!(part_1_points(&card.unwrap()), 2);
}

#[test]
fn test_ad() {
    let card = parse_card("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83");
    assert_eq!(card.is_some(), true);
    assert_eq!(part_1_points(&card.unwrap()), 1);
}

#[test]
fn test_ae() {
    let card = parse_card("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");
    assert_eq!(card.is_some(), true);
    assert_eq!(part_1_points(&card.unwrap()), 0);
}

#[test]
fn test_af() {
    let card = parse_card("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
    assert_eq!(card.is_some(), true);
    assert_eq!(part_1_points(&card.unwrap()), 0);
}

#[test]
fn test_ag() {
    let cards = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(part_2(cards), Some(30));
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
