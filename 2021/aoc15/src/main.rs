#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

fn neighbors(current: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let (x, y) = current;
    let mut result: Vec<(usize, usize)> = Vec::new();

    if x > 0 {
        result.push((x - 1, y));
    }

    if y > 0 {
        result.push((x, y - 1));
    }

    let nx = x + 1;
    if nx < width {
        result.push((nx, y));
    }

    let ny = y + 1;
    if ny < height {
        result.push((x, ny));
    }

    return result;
}

fn part_1(risk_map: &str) -> u32 {
    // Solve with Dijkstra's algorithm, simple variant without Heaps

    let mut risk_levels: Vec<Vec<u32>> = Vec::new();
    let mut distances: Vec<Vec<u32>> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();

    let mut width: usize = 0;
    for line in risk_map.lines() {
        width = line.len();
        risk_levels.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
        distances.push(vec![u32::MAX; width]);
        visited.push(vec![false; width]);
    }
    let height = risk_levels.len();
    distances[0][0] = 0;

    let mut current: (usize, usize) = (0, 0);

    while !visited[height - 1][width - 1] {
        for neigh in neighbors(current, width, height) {
            if visited[neigh.1][neigh.0] {
                continue;
            }
            let new_dist = distances[current.1][current.0] + risk_levels[neigh.1][neigh.0];
            if new_dist < distances[neigh.1][neigh.0] {
                distances[neigh.1][neigh.0] = new_dist;
            }
        }

        visited[current.1][current.0] = true;

        // In a more complicated implementation one could use a better way to find the next node.
        let mut next_dist: u32 = u32::MAX;
        for y in 0..height {
            for x in 0..width {
                if !visited[y][x] && distances[y][x] < next_dist {
                    next_dist = distances[y][x];
                    current = (x, y);
                }
            }
        }
        if next_dist == u32::MAX {
            //panic!("no next node");
            break;
        }
    }

    // for y in 0..height {
    //     for x in 0..width {
    //         if distances[y][x] == u32::MAX {
    //             print!("max ");
    //         } else {
    //             print!("{0:03} ", distances[y][x]);
    //         }
    //     }
    //     println!("");
    // }

    return distances[height - 1][width - 1];
}

fn part_2(_risk_map: &str) -> u32 {
    0
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), 40);
}

fn main() {
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
    println!("Done");
}
