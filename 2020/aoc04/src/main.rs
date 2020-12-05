//  --- Day 4: Passport Processing ---
//
// You arrive at the airport only to realize that you grabbed your North Pole Credentials instead of your passport. While these documents are extremely similar, North Pole Credentials aren't issued by a country and therefore aren't actually valid documentation for travel in most of the world.
//
// It seems like you're not the only one having problems, though; a very long line has formed for the automatic passport scanners, and the delay could upset your travel itinerary.
//
// Due to some questionable network security, you realize you might be able to solve both of these problems at the same time.
//
// The automatic passport scanners are slow because they're having trouble detecting which passports have all required fields. The expected fields are as follows:
//
//     byr (Birth Year)
//     iyr (Issue Year)
//     eyr (Expiration Year)
//     hgt (Height)
//     hcl (Hair Color)
//     ecl (Eye Color)
//     pid (Passport ID)
//     cid (Country ID)
//
// Passport data is validated in batch files (your puzzle input). Each passport is represented as a sequence of key:value pairs separated by spaces or newlines. Passports are separated by blank lines.
//
// Here is an example batch file containing four passports:
//
// ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
// byr:1937 iyr:2017 cid:147 hgt:183cm
//
// iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
// hcl:#cfa07d byr:1929
//
// hcl:#ae17e1 iyr:2013
// eyr:2024
// ecl:brn pid:760753108 byr:1931
// hgt:179cm
//
// hcl:#cfa07d eyr:2025 pid:166559648
// iyr:2011 ecl:brn hgt:59in
//
// The first passport is valid - all eight fields are present. The second passport is invalid - it is missing hgt (the Height field).
//
// The third passport is interesting; the only missing field is cid, so it looks like data from North Pole Credentials, not a passport at all! Surely, nobody would mind if you made the system temporarily ignore missing cid fields. Treat this "passport" as valid.
//
// The fourth passport is missing two fields, cid and byr. Missing cid is fine, but missing any other field is not, so this passport is invalid.
//
// According to the above rules, your improved system would report 2 valid passports
//
// The first half of this puzzle is complete! It provides one gold star: *
// --- Part Two ---
//
// The line is moving more quickly now, but you overhear airport security talking about how passports with invalid data are getting through. Better add some data validation, quick!
//
// You can continue to ignore the cid field, but each other field has strict rules about what values are valid for automatic validation:
//
//     byr (Birth Year) - four digits; at least 1920 and at most 2002.
//     iyr (Issue Year) - four digits; at least 2010 and at most 2020.
//     eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
//     hgt (Height) - a number followed by either cm or in:
//         If cm, the number must be at least 150 and at most 193.
//         If in, the number must be at least 59 and at most 76.
//     hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
//     ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
//     pid (Passport ID) - a nine-digit number, including leading zeroes.
//     cid (Country ID) - ignored, missing or not.
//
// Your job is to count the passports where all required fields are both present and valid according to the above rules. Here are some example values:
//
// byr valid:   2002
// byr invalid: 2003
//
// hgt valid:   60in
// hgt valid:   190cm
// hgt invalid: 190in
// hgt invalid: 190
//
// hcl valid:   #123abc
// hcl invalid: #123abz
// hcl invalid: 123abc
//
// ecl valid:   brn
// ecl invalid: wat
//
// pid valid:   000000001
// pid invalid: 0123456789
//
// Here are some invalid passports:
//
// eyr:1972 cid:100
// hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
//
// iyr:2019
// hcl:#602927 eyr:1967 hgt:170cm
// ecl:grn pid:012533040 byr:1946
//
// hcl:dab227 iyr:2012
// ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
//
// hgt:59cm ecl:zzz
// eyr:2038 hcl:74454a iyr:2023
// pid:3556412378 byr:2007
//
// Here are some valid passports:
//
// pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
// hcl:#623a2f
//
// eyr:2029 ecl:blu cid:129 byr:1989
// iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
//
// hcl:#888785
// hgt:164cm byr:2001 iyr:2015 cid:88
// pid:545766238 ecl:hzl
// eyr:2022
//
// iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
//
// Count the number of valid passports - those that have all required fields and valid values. Continue to treat cid as optional. In your batch file, how many passports are valid?

use std::str::FromStr;

#[allow(unused)]
const TEST_INPUT_A: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

#[allow(unused)]
const TEST_INPUT_B: &str = "ecl:not_a_valid_color pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

#[allow(unused)]
const TEST_INPUT_P2_ALL_INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

#[allow(unused)]
const TEST_INPUT_P2_4_VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

#[allow(unused)]
const INPUT: &str = include_str!("input");

#[allow(non_camel_case_types, dead_code)]
enum EyeColor {
    amb,
    blu,
    brn,
    gry,
    grn,
    hzl,
    oth,
}

impl FromStr for EyeColor {
    // alternatively use the enum_derive crate

    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "amb" => Ok(EyeColor::amb),
            "blu" => Ok(EyeColor::blu),
            "brn" => Ok(EyeColor::brn),
            "gry" => Ok(EyeColor::gry),
            "grn" => Ok(EyeColor::grn),
            "hzl" => Ok(EyeColor::hzl),
            "oth" => Ok(EyeColor::oth),
            _ => Err(()),
            // _ => {
            //     println!("unknown eye color [{}]", input);
            //     Err(())
            // }
        }
    }
}

struct PassportProperty<T> {
    /// Was this field ever seen in the passport string? Even if the string does not give a valid value.
    field_seen: bool,
    /// Only Some() when a value value was given in a passport string.
    val: Option<T>,
    label: &'static str,
}

impl<T> PassportProperty<T> {
    fn new(label: &'static str) -> Self {
        Self {
            field_seen: false,
            val: None,
            label,
        }
    }

    fn set(&mut self, val: Option<T>) {
        if self.field_seen {
            panic!("Error: setting field {} twice", self.label);
        }
        self.field_seen = true;
        self.val = val;
    }
}

struct Passport {
    byr: PassportProperty<i32>,
    iyr: PassportProperty<i32>,
    eyr: PassportProperty<i32>,
    hgt: PassportProperty<i32>, // in cm
    hcl: PassportProperty<(u8, u8, u8)>,
    ecl: PassportProperty<EyeColor>,
    pid: PassportProperty<String>,
    cid: PassportProperty<()>,
}

impl Passport {
    fn new(passport_str: &String) -> Passport {
        let mut res = Passport {
            byr: PassportProperty::new("byr"),
            iyr: PassportProperty::new("iyr"),
            eyr: PassportProperty::new("eyr"),
            hgt: PassportProperty::new("hgt"),
            hcl: PassportProperty::new("hcl"),
            ecl: PassportProperty::new("ecl"),
            pid: PassportProperty::new("pid"),
            cid: PassportProperty::new("cid"),
        };

        let fields: Vec<(&str, &str)> = passport_str
            .lines()
            .map(|line| -> Vec<&str> {
                return line.split_whitespace().collect();
            })
            .collect::<Vec<Vec<&str>>>()
            .concat()
            .into_iter()
            .map(|kv| -> (&str, &str) {
                let mut s = kv.split(":");
                return (s.next().unwrap(), s.next().unwrap());
            })
            .collect();

        for (k, v) in fields {
            match k {
                "byr" => {
                    res.byr
                        .set(Passport::parse_4_digit_int_in_range(v, 1920..=2002));
                }
                "iyr" => {
                    res.iyr
                        .set(Passport::parse_4_digit_int_in_range(v, 2010..=2020));
                }
                "eyr" => {
                    res.eyr
                        .set(Passport::parse_4_digit_int_in_range(v, 2020..=2030));
                }
                "hgt" => {
                    res.hgt.set(Passport::height_in_cm_from_str(v));
                }
                "hcl" => {
                    res.hcl.set(Passport::hair_color_from_str(v));
                }
                "ecl" => {
                    res.ecl.set(EyeColor::from_str(v).ok());
                }
                "pid" => {
                    res.pid.set(Passport::verify_is_9_digits_str(v));
                }
                "cid" => {
                    res.cid.set(Some(()));
                }
                _ => {
                    println!("Unrecognized key {0}", k);
                }
            }
        }

        return res;
    }

    fn verify_is_9_digits_str(s: &str) -> Option<String> {
        if s.len() != 9 {
            return None;
        }
        for c in s.chars() {
            if !('0'..='9').contains(&c) {
                return None;
            }
        }
        return Some(s.to_string());
    }

    fn parse_4_digit_int_in_range(s: &str, range: std::ops::RangeInclusive<i32>) -> Option<i32> {
        if s.len() != 4 {
            return None;
        }
        let res: i32 = s.parse().ok()?;
        if !range.contains(&res) {
            return None;
        }
        return Some(res);
    }

    fn hex_char_2_int(c: char) -> Option<u8> {
        match c {
            '0' => Some(0),
            '1' => Some(1),
            '2' => Some(2),
            '3' => Some(3),
            '4' => Some(4),
            '5' => Some(5),
            '6' => Some(6),
            '7' => Some(7),
            '8' => Some(8),
            '9' => Some(9),
            'a' => Some(10),
            'b' => Some(11),
            'c' => Some(12),
            'd' => Some(13),
            'e' => Some(14),
            'f' => Some(15),
            'A' => Some(10),
            'B' => Some(11),
            'C' => Some(12),
            'D' => Some(13),
            'E' => Some(14),
            'F' => Some(15),
            _ => None,
        }
    }

    fn hair_color_from_str(input: &str) -> Option<(u8, u8, u8)> {
        if input.len() != 7 {
            return None;
        }

        let mut chars = input.chars();
        if chars.next() != Some('#') {
            return None;
        }

        let r = Passport::hex_char_2_int(chars.next()?)? * 16
            + Passport::hex_char_2_int(chars.next()?)?;

        let g = Passport::hex_char_2_int(chars.next()?)? * 16
            + Passport::hex_char_2_int(chars.next()?)?;

        let b = Passport::hex_char_2_int(chars.next()?)? * 16
            + Passport::hex_char_2_int(chars.next()?)?;

        return Some((r, g, b));
    }

    fn height_in_cm_from_str(input: &str) -> Option<i32> {
        let l = input.len();
        if l < 3 {
            return None;
        }
        let cm = match &input[(l - 2)..l] {
            "in" => {
                let inches: i32 = input[0..(l - 2)].parse().ok()?;
                let cm: i32 = ((inches as f32) * 2.54).round() as i32;
                cm
            }
            "cm" => {
                let cm: i32 = input[0..(l - 2)].parse().ok()?;
                cm
            }
            _ => {
                return None;
            }
        };
        if !(150..=193).contains(&cm) {
            return None;
        }
        return Some(cm);
    }

    fn valid_p1(&self) -> bool {
        return self.byr.field_seen
            && self.iyr.field_seen
            && self.eyr.field_seen
            && self.hgt.field_seen
            && self.hcl.field_seen
            && self.ecl.field_seen
            && self.pid.field_seen;
    }

    fn valid_p2(&self) -> bool {
        return self.byr.val.is_some()
            && self.iyr.val.is_some()
            && self.eyr.val.is_some()
            && self.hgt.val.is_some()
            && self.hcl.val.is_some()
            && self.ecl.val.is_some()
            && self.pid.val.is_some();
    }
}

fn split_passport_batch_file(pbf: &str) -> impl Iterator<Item = String> {
    let mut passport_strs: Vec<String> = Vec::new();
    let mut current_passport = String::new();

    for line in pbf.lines() {
        let line = line.trim();
        if line.len() > 0 {
            current_passport.push_str(line);
            current_passport.push('\n');
        } else {
            current_passport = current_passport.trim().to_string();
            if current_passport.len() > 0 {
                passport_strs.push(current_passport);
            }
            current_passport = String::new();
        }
    }

    current_passport = current_passport.trim().to_string();
    if current_passport.len() > 0 {
        passport_strs.push(current_passport);
    }

    return passport_strs.into_iter();
}

fn part_1_2(passport_batch_file: &str) -> impl Iterator<Item = Passport> {
    return split_passport_batch_file(passport_batch_file).map(|s| Passport::new(&s));
}

#[allow(unused)]
fn part_1(passport_batch_file: &str) -> usize {
    return part_1_2(passport_batch_file)
        .filter(|p| p.valid_p1())
        .count();
}

#[allow(unused)]
fn part_2(passport_batch_file: &str) -> usize {
    return part_1_2(passport_batch_file)
        .filter(|p| p.valid_p2())
        .count();
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT_A), 2);
}

#[test]
fn test_b() {
    assert_eq!(part_1(TEST_INPUT_B), 2);
}

#[test]
fn test_p2_all_invalid() {
    assert_eq!(part_2(TEST_INPUT_P2_ALL_INVALID), 0);
}

#[test]
fn test_p2_4_valid() {
    assert_eq!(part_2(TEST_INPUT_P2_4_VALID), 4);
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
    println!("done");
}
