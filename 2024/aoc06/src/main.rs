use png;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;
use std::ops::{Add, Index};

#[allow(unused)]
const TEST_INPUT: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord2D {
    row: i32,
    col: i32,
}

impl Coord2D {
    fn rotate_right(&mut self) {
        let r = self.col;
        let c = self.row * -1;
        self.row = r;
        self.col = c;
    }
}

impl Add<&Coord2D> for &Coord2D {
    type Output = Coord2D;

    fn add(self, other: &Coord2D) -> Coord2D {
        Coord2D {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

struct Laboratory {
    f: bool,
    width: usize,
    height: usize,
    positions: Vec<Vec<bool>>,
}

impl Index<&Coord2D> for Laboratory {
    type Output = bool;

    fn index(&self, coord: &Coord2D) -> &Self::Output {
        if coord.row < 0 || coord.col < 0 {
            return &self.f;
        }
        let r = coord.row as usize;
        let c = coord.col as usize;
        if r >= self.height || c >= self.width {
            return &self.f;
        }
        return &self.positions[r][c];
    }
}

impl Laboratory {
    fn new(width: usize, height: usize, positions: Vec<Vec<bool>>) -> Self {
        Self {
            f: false,
            width,
            height,
            positions,
        }
    }

    fn contains(&self, c: &Coord2D) -> bool {
        return c.row >= 0
            && c.col >= 0
            && (c.row as usize) < self.height
            && (c.col as usize) < self.width;
    }
}

fn parse_map_str(map: &str) -> Option<(Laboratory, Coord2D, Coord2D)> {
    let mut positions: Vec<Vec<bool>> = Vec::new();

    let mut width: usize = 0;
    let mut pos = Coord2D { row: -1, col: -1 };
    let mut direction = Coord2D { row: -1, col: -1 };
    for (row, line) in map.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        if width == 0 {
            width = line.len();
        } else {
            if width != line.len() {
                eprintln!("width contradiction");
                return None;
            }
        }

        let mut v: Vec<bool> = Vec::with_capacity(width);
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    v.push(true);
                }
                '.' => {
                    v.push(false);
                }
                '^' => {
                    if pos.col != -1 {
                        eprintln!("pos up contradiction");
                        return None;
                    }
                    pos = Coord2D {
                        row: row as i32,
                        col: col as i32,
                    };
                    direction = Coord2D { row: -1, col: 0 };
                    v.push(false);
                }
                '<' => {
                    if pos.col != -1 {
                        eprintln!("pos left contradiction");
                        return None;
                    }
                    pos = Coord2D {
                        row: row as i32,
                        col: col as i32,
                    };
                    direction = Coord2D { row: 0, col: -1 };
                    v.push(false);
                }
                '>' => {
                    if pos.col != -1 {
                        eprintln!("pos right contradiction");
                        return None;
                    }
                    pos = Coord2D {
                        row: row as i32,
                        col: col as i32,
                    };
                    direction = Coord2D { row: 0, col: 1 };
                    v.push(false);
                }
                'v' => {
                    if pos.col != -1 {
                        eprintln!("pos right contradiction");
                        return None;
                    }
                    pos = Coord2D {
                        row: row as i32,
                        col: col as i32,
                    };
                    direction = Coord2D { row: 1, col: 0 };
                    v.push(false);
                }
                x => {
                    eprintln!("unknown char |{0}|", x);
                    return None;
                }
            }
        }
        if v.len() != width {
            eprintln!("later width contradiction {0} {1}", v.len(), width);
            return None;
        }
        positions.push(v);
    }

    return Some((
        Laboratory::new(width, positions.len(), positions),
        pos,
        direction,
    ));
}

unsafe fn draw_part_1(
    lab: &Laboratory,
    pos: &Coord2D,
    next_pos: &Coord2D,
    stepped_on: &HashSet<Coord2D>,
) {
    // ffmpeg -f image2 -r 15 -pattern_type glob -i '*.png' -an -c:v libx264 -r 15 timelapse.mp4

    static mut IMG_COUNTER: u32 = 0;
    IMG_COUNTER += 1;

    let fname = format!("img{0:05}.png", IMG_COUNTER);
    let out = File::create(fname).unwrap();

    let zoom_factor = 16;

    let mut pixels: Vec<Vec<(u8, u8, u8)>> = Vec::with_capacity(lab.height);

    for row in 0..lab.height {
        let mut row_pixels: Vec<(u8, u8, u8)> = Vec::with_capacity(lab.width);
        for col in 0..lab.width {
            let c = Coord2D {
                row: row as i32,
                col: col as i32,
            };

            if c == *pos {
                row_pixels.push((255, 0, 0));
            } else if c == *next_pos {
                if lab[&c] {
                    row_pixels.push((204, 204, 204));
                } else {
                    row_pixels.push((0, 0, 255));
                }
            } else {
                if lab[&c] {
                    row_pixels.push((0, 0, 0));
                } else if stepped_on.contains(&c) {
                    row_pixels.push((0xFD, 0xEE, 0x73));
                } else {
                    row_pixels.push((255, 255, 255));
                };
            }
        }
        pixels.push(row_pixels);
    }

    let ref mut png_w = BufWriter::new(out);
    let mut png_encoder = png::Encoder::new(
        png_w,
        (lab.width * zoom_factor) as u32,
        (lab.height * zoom_factor) as u32,
    );
    png_encoder.set_color(png::ColorType::Rgb);
    png_encoder.set_depth(png::BitDepth::Eight);
    let mut png_w_2 = png_encoder.write_header().unwrap();

    let mut zoomed_pixels: Vec<u8> =
        Vec::with_capacity(lab.width * zoom_factor * lab.height * zoom_factor * 3);
    for row in 0..lab.height {
        for _z in 0..zoom_factor {
            for col in 0..lab.width {
                let (r, g, b) = pixels[row][col];
                for _ in 0..zoom_factor {
                    zoomed_pixels.extend_from_slice(&[r, g, b]);
                }
            }
        }
    }
    png_w_2.write_image_data(&zoomed_pixels).unwrap();
}

fn part_1_simple(map: &str) -> Option<usize> {
    let (lab, mut pos, mut direction) = parse_map_str(map)?;
    let mut stepped_on: HashSet<Coord2D> = HashSet::new();

    while lab.contains(&pos) {
        stepped_on.insert(pos.clone());

        let mut next_pos = &pos + &direction;

        // unsafe {
        //     draw_part_1(&lab, &pos, &next_pos, &stepped_on);
        // }

        let mut infinite_loop_check = 0;
        while lab[&next_pos] {
            infinite_loop_check += 1;
            if infinite_loop_check > 8 {
                eprintln!("infinite loop");
                return None;
            }
            direction.rotate_right();
            next_pos = &pos + &direction
        }

        pos = next_pos;
    }

    return Some(stepped_on.len());
}

fn part_1(map: &str) -> Option<usize> {
    part_1_simple(map)
}

fn part_2(map: &str) -> Option<usize> {
    None
}

#[test]
fn test_a() {
    assert_eq!(part_1_simple(TEST_INPUT), Some(41));
}

#[test]
fn test_b() {
    assert_eq!(part_1(TEST_INPUT), Some(41));
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
