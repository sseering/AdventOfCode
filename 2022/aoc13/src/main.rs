use std::cmp::min;
use std::cmp::Ordering;

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

#[derive(PartialEq, Eq, Clone)]
pub enum Packet {
    Int(i32),
    List(Box<Vec<Packet>>),
}

impl Packet {
    fn new() -> Self {
        Self::List(Box::new(Vec::new()))
    }

    fn one_int_list(i: i32) -> Self {
        return Packet::List(Box::new(vec![Packet::Int(i)]));
    }

    fn part_2_divider_packet_a() -> Self {
        Self::List(Box::new(vec![Self::one_int_list(2)]))
    }

    fn part_2_divider_packet_b() -> Self {
        Self::List(Box::new(vec![Self::one_int_list(6)]))
    }

    fn is_part_2_divider_packet(&self) -> bool {
        match self {
            Packet::Int(_) => {
                return false;
            }
            Packet::List(l) => {
                if l.len() != 1 {
                    return false;
                }
                match &l[0] {
                    Packet::Int(_) => {
                        return false;
                    }
                    Packet::List(ll) => {
                        if ll.len() != 1 {
                            return false;
                        }
                        match &ll[0] {
                            Packet::Int(i) => {
                                return (*i) == 2 || (*i) == 6;
                            }
                            Packet::List(_) => {
                                return false;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub mod parsing {

    #[derive(PartialEq, Eq)]
    enum ParserToken {
        OpenB,
        CloseB,
        Int(i32),
    }

    fn tokenize(packet: &str) -> Option<Vec<ParserToken>> {
        let mut packet = packet;
        let mut res: Vec<ParserToken> = Vec::new();
        while packet.len() > 0 {
            let findr = packet.find([',', '[', ']']);

            match findr {
                Some(idx) => {
                    if idx > 0 {
                        res.push(ParserToken::Int(packet[0..idx].parse().ok()?));
                    }

                    match &packet[idx..=idx] {
                        "[" => {
                            res.push(ParserToken::OpenB);
                        }
                        "]" => {
                            res.push(ParserToken::CloseB);
                        }
                        "," => { /* nothing */ }
                        _ => {
                            return None;
                        }
                    }

                    packet = &packet[(idx + 1)..];
                }
                None => {
                    res.push(ParserToken::Int(packet.parse().ok()?));
                    break;
                }
            }
        }

        return Some(res);
    }

    fn parse_packet_recursive(
        packet: &Vec<ParserToken>,
        from: usize,
        len: usize,
    ) -> Option<(Packet, usize)> {
        let mut from = from;

        match packet[from] {
            ParserToken::OpenB => {
                let mut res = Packet::new();
                from += 1;

                while from < len && packet[from] != ParserToken::CloseB {
                    match res {
                        Packet::List(ref mut l) => {
                            let (p, next) = parse_packet_recursive(packet, from, len)?;
                            l.push(p);
                            from = next;
                        }
                        _ => {
                            return None;
                        }
                    }
                }

                if from >= len {
                    return None;
                }

                return Some((res, from + 1));
            }
            ParserToken::Int(i) => return Some((Packet::Int(i), from + 1)),
            _ => return None,
        };
    }

    use super::Packet;

    pub fn parse_packet(packet: &str) -> Option<Packet> {
        let packet = tokenize(packet)?;

        let (res, _) = parse_packet_recursive(&packet, 0, packet.len())?;
        return Some(res);
    }

    pub fn parse_packet_list_into_pairs(packet_list: &str) -> Option<Vec<(Packet, Packet)>> {
        let mut res: Vec<(Packet, Packet)> = Vec::new();

        let mut lines = packet_list.lines();
        let mut a = lines.next();
        while let Some(aa) = a {
            let aa = aa.trim();
            if aa.is_empty() {
                return None;
            }
            let bb = lines.next()?.trim();
            if bb.is_empty() {
                return None;
            }
            let cc = lines.next();
            if let Some(ccc) = cc {
                if !ccc.is_empty() {
                    return None;
                }
            }

            res.push((parse_packet(aa)?, parse_packet(bb)?));

            a = lines.next();
        }

        return Some(res);
    }
}

fn part_1_cmp(left: &mut Packet, right: &mut Packet) -> Option<Ordering> {
    let mut idx: usize = 0;

    let l = match left {
        Packet::List(ll) => ll,
        Packet::Int(_) => {
            return None;
        }
    };
    let r = match right {
        Packet::List(rr) => rr,
        Packet::Int(_) => {
            return None;
        }
    };
    let len = min(l.len(), r.len());

    while idx < len {
        let left_val = &mut l[idx];
        let right_val = &mut r[idx];
        match left_val {
            Packet::Int(il) => match right_val {
                Packet::Int(ir) => {
                    let cmpr = (*il).cmp(ir);
                    if cmpr == Ordering::Equal {
                        idx += 1;
                    } else {
                        return Some(cmpr);
                    }
                }
                Packet::List(_) => {
                    l[idx] = Packet::one_int_list(*il);
                }
            },
            Packet::List(_) => match right_val {
                Packet::Int(ir) => {
                    r[idx] = Packet::one_int_list(*ir);
                }
                Packet::List(_) => {
                    let cmpr = part_1_cmp(left_val, right_val)?;
                    if cmpr == Ordering::Equal {
                        idx += 1;
                    } else {
                        return Some(cmpr);
                    }
                }
            },
        }
    }

    return Some(l.len().cmp(&r.len()));
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut sc = self.clone();
        let mut oc = other.clone();
        return part_1_cmp(&mut sc, &mut oc);
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut sc = self.clone();
        let mut oc = other.clone();
        match part_1_cmp(&mut sc, &mut oc) {
            Some(res) => {
                return res;
            }
            None => {
                panic!();
            }
        }
    }
}

fn part_1(packet_list: &str) -> Option<usize> {
    let res: usize = parsing::parse_packet_list_into_pairs(packet_list)?
        .iter_mut()
        .enumerate()
        .map(|(idx, (l, r))| -> Option<usize> {
            let cmprr = part_1_cmp(l, r);
            let cmpr = cmprr?;
            return Some(match cmpr {
                Ordering::Less => idx + 1,
                _ => 0,
            });
        })
        .sum::<Option<usize>>()?;

    return Some(res);
}

fn part_2(packet_list: &str) -> Option<usize> {
    let mut packets: Vec<Packet> = packet_list
        .lines()
        .filter(|l| !l.is_empty())
        .map(parsing::parse_packet)
        .collect::<Option<Vec<Packet>>>()?;
    packets.push(Packet::part_2_divider_packet_a());
    packets.push(Packet::part_2_divider_packet_b());

    packets.sort_unstable();

    let mut res = 1;

    for (idx, packet) in packets.iter().enumerate() {
        if packet.is_part_2_divider_packet() {
            res *= idx + 1;
        }
    }

    return Some(res);
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), Some(13));
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), Some(140));
}

#[test]
fn test_ba() {
    assert!(!Packet::new().is_part_2_divider_packet());
}

#[test]
fn test_bb() {
    assert!(!Packet::one_int_list(42).is_part_2_divider_packet());
}

#[test]
fn test_bc() {
    assert!(Packet::part_2_divider_packet_a().is_part_2_divider_packet());
}

#[test]
fn test_bd() {
    assert!(Packet::part_2_divider_packet_b().is_part_2_divider_packet());
}

fn main() {
    match part_1(INPUT) {
        Some(r) => {
            println!("part 1: {}", r);
        }
        None => {
            println!("part 1 failed.")
        }
    }
    match part_2(INPUT) {
        Some(r) => {
            println!("part 2: {}", r);
        }
        None => {
            println!("part 2 failed.")
        }
    }

    println!("done.");
}
