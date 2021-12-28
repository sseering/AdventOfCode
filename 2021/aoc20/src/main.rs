use std::fmt;

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");
#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

const LIGHT: bool = true;
const DARK: bool = false;
const IMG_ENHANCEMENT_ALG_LEN: usize = 512;

fn char_2_lightness(c: char) -> bool {
    match c {
        '.' => DARK,
        '#' => LIGHT,
        _ => panic!(),
    }
}

fn lightness_2_debug_char(l: bool) -> char {
    if l == LIGHT {
        '█'
    } else {
        '.'
    }
}

#[derive(Eq)]
struct ScannerImage {
    pixel: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    filled_area_left: usize,   // inclusive index
    filled_area_top: usize,    // inclusive index
    filled_area_right: usize,  // exclusive index
    filled_area_bottom: usize, // exclusive index
    infinity_lightness: bool,
}

impl ScannerImage {
    fn new<'a>(
        num_image_enhancement_iterations: usize,
        mut image_str: impl Iterator<Item = &'a str>,
    ) -> Self {
        let line_a = image_str.next().unwrap();
        let line_len = line_a.len();
        let width = 2 * num_image_enhancement_iterations + line_len;

        let mut p: Vec<Vec<bool>> = Vec::with_capacity(width);
        for _ in 0..num_image_enhancement_iterations {
            p.push(vec![DARK; width]);
        }

        let left_right_padding = vec![DARK; num_image_enhancement_iterations];
        let mut pp: Vec<bool> = Vec::with_capacity(width);
        pp.extend(&left_right_padding);
        pp.extend(line_a.chars().map(char_2_lightness));
        pp.extend(&left_right_padding);
        p.push(pp);

        for line in image_str {
            if line.len() != line_len {
                panic!();
            }
            let mut pp: Vec<bool> = Vec::with_capacity(width);
            pp.extend(&left_right_padding);
            pp.extend(line.chars().map(char_2_lightness));
            pp.extend(&left_right_padding);
            p.push(pp);
        }

        for _ in 0..num_image_enhancement_iterations {
            p.push(vec![DARK; width]);
        }

        let height = p.len();

        if height == 0 || line_len == 0 {
            panic!();
        }

        Self {
            pixel: p,
            width,
            height,
            filled_area_left: num_image_enhancement_iterations,
            filled_area_top: num_image_enhancement_iterations,
            filled_area_right: width - num_image_enhancement_iterations,
            filled_area_bottom: height - num_image_enhancement_iterations,
            infinity_lightness: DARK,
        }
    }

    fn part_1_score(&self) -> u32 {
        let mut result = 0;
        for y in self.filled_area_top..self.filled_area_bottom {
            let sublist = &self.pixel[y];
            for x in self.filled_area_left..self.filled_area_right {
                if sublist[x] == LIGHT {
                    result += 1;
                }
            }
        }
        return result;
        // return self
        //     .pixel
        //     .iter()
        //     .map(|sublist| {
        //         sublist
        //             .iter()
        //             .map(|&c| if c == LIGHT { 1 } else { 0 })
        //             .sum::<u32>()
        //     })
        //     .sum();
    }

    fn get(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 {
            return self.infinity_lightness;
        }
        let x = x as usize;
        let y = y as usize;
        if x < self.filled_area_left
            || y < self.filled_area_top
            || x >= self.filled_area_right
            || y >= self.filled_area_bottom
        {
            return self.infinity_lightness;
        }

        return self.pixel[y][x];
    }

    fn get_bit(&self, x: isize, y: isize) -> usize {
        if self.get(x, y) == LIGHT {
            1
        } else {
            0
        }
    }

    fn part_1_enhancement(&mut self, img_enhancement_alg: &[bool; IMG_ENHANCEMENT_ALG_LEN]) {
        let mut new = self.pixel.clone();

        // println!("old {0:?}", self);

        let ylo: isize = (self.filled_area_top - 1) as isize;
        let yhi: isize = self.filled_area_bottom as isize;
        let xlo: isize = (self.filled_area_left - 1) as isize;
        let xhi: isize = self.filled_area_right as isize;
        for y in ylo..=yhi {
            for x in xlo..=xhi {
                let idx = (self.get_bit(x - 1, y - 1) << 8)
                    | (self.get_bit(x, y - 1) << 7)
                    | (self.get_bit(x + 1, y - 1) << 6)
                    | (self.get_bit(x - 1, y) << 5)
                    | (self.get_bit(x, y) << 4)
                    | (self.get_bit(x + 1, y) << 3)
                    | (self.get_bit(x - 1, y + 1) << 2)
                    | (self.get_bit(x, y + 1) << 1)
                    | (self.get_bit(x + 1, y + 1));
                // if x == 6 || y == 6 {
                //     println!(
                //         "x {} y {} idx {} new {}\n{}{}{}\n{}{}{}\n{}{}{}",
                //         x,
                //         y,
                //         idx,
                //         lightness_2_debug_char(img_enhancement_alg[idx]),
                //         lightness_2_debug_char(self.get(x - 1, y - 1)),
                //         lightness_2_debug_char(self.get(x, y - 1)),
                //         lightness_2_debug_char(self.get(x + 1, y - 1)),
                //         lightness_2_debug_char(self.get(x - 1, y)),
                //         lightness_2_debug_char(self.get(x, y)),
                //         lightness_2_debug_char(self.get(x + 1, y)),
                //         lightness_2_debug_char(self.get(x - 1, y + 1)),
                //         lightness_2_debug_char(self.get(x, y + 1)),
                //         lightness_2_debug_char(self.get(x + 1, y + 1)),
                //     );
                // }
                new[y as usize][x as usize] = img_enhancement_alg[idx];
            }
        }

        self.filled_area_left -= 1;
        self.filled_area_right += 1;
        self.filled_area_top -= 1;
        self.filled_area_bottom += 1;
        self.pixel = new;
        self.infinity_lightness = img_enhancement_alg[if self.infinity_lightness == DARK {
            0
        } else {
            IMG_ENHANCEMENT_ALG_LEN - 1
        }];
    }
}

impl PartialEq for ScannerImage {
    fn eq(&self, other: &Self) -> bool {
        if self.width != other.width
            || self.height != other.height
            || self.filled_area_left != other.filled_area_left
            || self.filled_area_right != other.filled_area_right
            || self.filled_area_top != other.filled_area_top
            || self.filled_area_bottom != other.filled_area_bottom
        {
            return false;
        }

        for y in self.filled_area_top..self.filled_area_bottom {
            let s_sublist = &self.pixel[y];
            let o_sublist = &other.pixel[y];
            for x in self.filled_area_left..self.filled_area_right {
                if s_sublist[x] != o_sublist[x] {
                    return false;
                }
            }
        }

        // let sp = &self.pixel;
        // let op = &other.pixel;
        // if sp.len() != op.len() || sp[0].len() != op[0].len() {
        //     return false;
        // }

        // for (ssp, osp) in sp.iter().zip(op) {
        //     for (&p1, &p2) in ssp.iter().zip(osp) {
        //         if p1 != p2 {
        //             false;
        //         }
        //     }
        // }

        return true;
    }
}

impl fmt::Debug for ScannerImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = self
            .pixel
            .iter()
            .map(|sublist| {
                sublist
                    .iter()
                    .map(|&l| lightness_2_debug_char(l))
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("x");
        f.debug_struct("ScannerImage")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("filled_area_left", &self.filled_area_left)
            .field("filled_area_top", &self.filled_area_top)
            .field("filled_area_right", &self.filled_area_right)
            .field("filled_area_bottom", &self.filled_area_bottom)
            .field("pixel", &p)
            .finish()
    }
}

fn parse_input(
    scanner_result: &str,
    img_enhancement_alg: &mut [bool; IMG_ENHANCEMENT_ALG_LEN],
    num_image_enhancement_iterations: usize,
) -> ScannerImage {
    let mut lines = scanner_result.lines();

    let img_enhancement_alg_str: &str = lines.next().unwrap();
    if img_enhancement_alg_str.len() != IMG_ENHANCEMENT_ALG_LEN {
        panic!();
    }
    for (idx, c) in img_enhancement_alg_str.chars().enumerate() {
        match c {
            '.' => {
                img_enhancement_alg[idx] = DARK;
            }
            '#' => {
                img_enhancement_alg[idx] = LIGHT;
            }
            _ => panic!(),
        }
    }
    // implementing this for non-infinite images only works
    // with test-input and not real input
    //
    // if img_enhancement_alg[0] != DARK {
    //     panic!("scanner image has infinite size");
    // }

    if lines.next().unwrap().len() != 0 {
        panic!();
    }

    return ScannerImage::new(num_image_enhancement_iterations, lines);
}

fn _part_1(scanner_result: &str, num_image_enhancement_iterations: usize) -> ScannerImage {
    let mut img_enhancement_alg: [bool; IMG_ENHANCEMENT_ALG_LEN] = [DARK; IMG_ENHANCEMENT_ALG_LEN];
    let mut img = parse_input(
        scanner_result,
        &mut img_enhancement_alg,
        num_image_enhancement_iterations,
    );

    for _ in 0..num_image_enhancement_iterations {
        img.part_1_enhancement(&img_enhancement_alg);
    }

    return img;
}

fn part_1(scanner_result: &str) -> u32 {
    return _part_1(scanner_result, 2).part_1_score();
}

fn part_2(scanner_result: &str) -> u32 {
    return _part_1(scanner_result, 50).part_1_score();
}

#[test]
fn test_a() {
    let left = _part_1(TEST_INPUT, 0);
    let mut img_enhancement_alg: [bool; IMG_ENHANCEMENT_ALG_LEN] = [DARK; IMG_ENHANCEMENT_ALG_LEN];
    let right_str = include_str!("../test-input.txt");
    let right = parse_input(right_str, &mut img_enhancement_alg, 0);
    assert_eq!(left, right);
}

#[test]
fn test_b() {
    let left = _part_1(TEST_INPUT, 1);
    let mut img_enhancement_alg: [bool; IMG_ENHANCEMENT_ALG_LEN] = [DARK; IMG_ENHANCEMENT_ALG_LEN];
    let right_str = include_str!("../test-b.txt");
    let right = parse_input(right_str, &mut img_enhancement_alg, 0);
    assert_eq!(left, right);
}

#[test]
fn test_c() {
    let left = _part_1(TEST_INPUT, 2);
    let mut img_enhancement_alg: [bool; IMG_ENHANCEMENT_ALG_LEN] = [DARK; IMG_ENHANCEMENT_ALG_LEN];
    let right_str = include_str!("../test-c.txt");
    let right = parse_input(right_str, &mut img_enhancement_alg, 0);
    assert_eq!(left, right);
}

#[test]
fn test_d() {
    let left = part_1(TEST_INPUT);
    assert_eq!(left, 35);
}

#[test]
fn test_e() {
    let left = part_2(TEST_INPUT);
    assert_eq!(left, 3351);
}

fn main() {
    println!("part 1: {}", part_1(INPUT));
    println!("part 1: {}", part_2(INPUT));
    println!("Done");
}
