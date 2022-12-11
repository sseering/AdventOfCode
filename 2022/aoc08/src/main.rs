#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

fn parse(height_map: &str) -> Option<(Vec<Vec<i8>>, usize)> {
    let height_map: Vec<Vec<i8>> = height_map
        .lines()
        .map(|line| -> Option<Vec<i8>> {
            return line
                .chars()
                .map(|c| {
                    let tmp: u32 = c.to_digit(10)?;
                    if tmp > 9 {
                        return None;
                    }
                    return Some(tmp as i8);
                })
                .collect();
        })
        .collect::<Option<Vec<Vec<i8>>>>()?;

    let wood_size = height_map.len();
    if wood_size != height_map.first()?.len() {
        return None;
    }

    return Some((height_map, wood_size));
}

fn part_1(height_map: &str) -> Option<u32> {
    let (height_map, wood_size) = parse(height_map)?;

    let mut seen = vec![vec![false; wood_size]; wood_size];

    for x in 0..wood_size {
        let mut sight_height: i8 = -1;
        let mut sight_height_reverse: i8 = -1;
        let mut sight_height_rot90: i8 = -1;
        let mut sight_height_reverse_rot90: i8 = -1;
        for y in 0..wood_size {
            let tree_height = height_map[x][y];
            if tree_height > sight_height {
                seen[x][y] = true;
                sight_height = tree_height;
            }

            let rev_y = wood_size - y - 1;
            let tree_height = height_map[x][rev_y];
            if tree_height > sight_height_reverse {
                seen[x][rev_y] = true;
                sight_height_reverse = tree_height;
            }

            let x_rot90 = y;
            let y_rot90 = x;

            let tree_height = height_map[x_rot90][y_rot90];
            if tree_height > sight_height_rot90 {
                seen[x_rot90][y_rot90] = true;
                sight_height_rot90 = tree_height;
            }

            let rev_x_rot90 = wood_size - x_rot90 - 1;
            let tree_height = height_map[rev_x_rot90][y_rot90];
            if tree_height > sight_height_reverse_rot90 {
                seen[rev_x_rot90][y_rot90] = true;
                sight_height_reverse_rot90 = tree_height;
            }
        }
    }

    return Some(
        seen.iter()
            .map(|sub| sub.iter().map(|&b| if b { 1 } else { 0 }).sum::<u32>())
            .sum(),
    );
}
fn part_2(height_map: &str) -> Option<u32> {
    let (height_map, wood_size) = parse(height_map)?;

    let mut best: u32 = 0;

    for x in 1..(wood_size - 1) {
        for y in 1..(wood_size - 1) {
            let treehouse_height = height_map[x][y];

            let mut walk_score_x_neg = 0;
            let mut x_walk = (x as i32) - 1;
            while x_walk >= 0 {
                walk_score_x_neg += 1;

                let tree_height = height_map[x_walk as usize][y];
                if tree_height >= treehouse_height {
                    break;
                }

                x_walk -= 1;
            }

            let mut walk_score_x_pos = 0;
            let mut x_walk = x + 1;
            while x_walk < wood_size {
                walk_score_x_pos += 1;

                let tree_height = height_map[x_walk][y];
                if tree_height >= treehouse_height {
                    break;
                }

                x_walk += 1;
            }

            let mut walk_score_y_neg = 0;
            let mut y_walk = (y as i32) - 1;
            while y_walk >= 0 {
                walk_score_y_neg += 1;

                let tree_height = height_map[x][y_walk as usize];
                if tree_height >= treehouse_height {
                    break;
                }

                y_walk -= 1;
            }

            let mut walk_score_y_pos = 0;
            let mut y_walk = y + 1;
            while y_walk < wood_size {
                walk_score_y_pos += 1;

                let tree_height = height_map[x][y_walk];
                if tree_height >= treehouse_height {
                    break;
                }

                y_walk += 1;
            }

            let score = walk_score_y_pos * walk_score_y_neg * walk_score_x_pos * walk_score_x_neg;
            if score > best {
                best = score;
            }
        }
    }

    return Some(best);
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), Some(21));
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), Some(8));
}

fn main() {
    println!("part 1: {}", part_1(INPUT).unwrap_or(99999));
    println!("part 2: {}", part_2(INPUT).unwrap_or(99999));
    println!("done.");
}
