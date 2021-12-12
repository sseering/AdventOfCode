use std::collections::HashMap;

#[allow(unused)]
const TEST_INPUT_A: &str = include_str!("../test-input-a.txt");

#[allow(unused)]
const TEST_INPUT_B: &str = include_str!("../test-input-b.txt");

#[allow(unused)]
const TEST_INPUT_C: &str = include_str!("../test-input-c.txt");

#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

struct CaveWalkState {
    current_cave: usize,
    caves_visited: Vec<bool>,
    num_steps_taken: usize,
    // path_taken: Vec<usize>,
    did_duplicate_visit: bool,
}

impl CaveWalkState {
    fn new(current_cave: usize, num_caves: usize) -> Self {
        let mut new_caves_visited = vec![false; num_caves];
        new_caves_visited[current_cave] = true;
        Self {
            current_cave,
            caves_visited: new_caves_visited,
            num_steps_taken: 0,
            // path_taken: vec![current_cave],
            did_duplicate_visit: false,
        }
    }

    fn clone_after_step(&self, new_cave: usize, is_small_cave: &Vec<bool>) -> Self {
        let mut new_caves_visited = self.caves_visited.clone();
        let mut new_did_duplicate_visit: bool = self.did_duplicate_visit;
        if new_caves_visited[new_cave] && is_small_cave[new_cave] {
            new_did_duplicate_visit = true;
        }
        new_caves_visited[new_cave] = true;
        // let mut new_path_taken = self.path_taken.clone();
        // new_path_taken.push(new_cave);
        return Self {
            current_cave: new_cave,
            caves_visited: new_caves_visited,
            num_steps_taken: self.num_steps_taken + 1,
            // path_taken: new_path_taken,
            did_duplicate_visit: new_did_duplicate_visit,
        };
    }

    // fn debug_print(&self) {
    //     println!(
    //         "{}",
    //         self.path_taken
    //             .iter()
    //             .map(|i| i.to_string())
    //             .fold(String::new(), |a, b| a + "," + &b)
    //     );
    // }
}

fn part_1_2_walk(
    part_1: bool,
    start_idx: usize,
    end_idx: usize,
    num_caves: usize,
    adjacancy_matrix: &Vec<Vec<bool>>,
    is_small_cave: &Vec<bool>,
    state: CaveWalkState,
) -> u32 {
    if state.num_steps_taken > 1_000_000 {
        panic!("infinite loop");
    }
    if state.current_cave == end_idx {
        // state.debug_print();
        return 1;
    }

    let mut further_walks_num_paths = 0;
    for neighbor_idx in 0..num_caves {
        if !adjacancy_matrix[state.current_cave][neighbor_idx] {
            continue;
        }
        if part_1 {
            if is_small_cave[neighbor_idx] && state.caves_visited[neighbor_idx] {
                continue;
            }
        } else {
            if neighbor_idx == start_idx {
                continue;
            }
            if state.did_duplicate_visit
                && is_small_cave[neighbor_idx]
                && state.caves_visited[neighbor_idx]
            {
                continue;
            }
        }
        further_walks_num_paths += part_1_2_walk(
            part_1,
            start_idx,
            end_idx,
            num_caves,
            adjacancy_matrix,
            is_small_cave,
            state.clone_after_step(neighbor_idx, is_small_cave),
        );
    }
    return further_walks_num_paths;
}

fn part_1_2(cave_system_str: &str, part_1: bool) -> u32 {
    let lines: Vec<&str> = cave_system_str.lines().collect();

    let mut cave_indices: HashMap<&str, usize> = HashMap::new();

    let mut start_idx: usize = usize::MAX;
    let mut end_idx: usize = usize::MAX;
    let mut idx = 0;
    for line in &lines {
        if let [a, b] = line.split('-').collect::<Vec<&str>>()[..] {
            if !cave_indices.contains_key(a) {
                cave_indices.insert(a, idx);

                if a == "start" {
                    start_idx = idx;
                }
                if a == "end" {
                    end_idx = idx;
                }

                idx += 1;
            }
            if !cave_indices.contains_key(b) {
                cave_indices.insert(b, idx);

                if b == "start" {
                    start_idx = idx;
                }
                if b == "end" {
                    end_idx = idx;
                }

                idx += 1;
            }
        } else {
            panic!("parsing failed");
        }
    }
    if start_idx == usize::MAX {
        panic!("start not found");
    }
    if end_idx == usize::MAX {
        panic!("end not found");
    }
    let num_caves: usize = idx;

    // println!("num_caves {}", num_caves);
    // println!("{0:?}", cave_indices);

    let mut is_small_cave: Vec<bool> = vec![false; num_caves];
    let mut adjacancy_matrix: Vec<Vec<bool>> = Vec::new();
    for _ in 0..num_caves {
        adjacancy_matrix.push(vec![false; num_caves]);
    }

    for line in &lines {
        if let [a, b] = line.split('-').collect::<Vec<&str>>()[..] {
            let a_idx: usize = *cave_indices.get(a).unwrap();
            let b_idx: usize = *cave_indices.get(b).unwrap();
            adjacancy_matrix[a_idx][b_idx] = true;
            adjacancy_matrix[b_idx][a_idx] = true;
            if a == a.to_ascii_lowercase() {
                is_small_cave[a_idx] = true;
            }
            if b == b.to_ascii_lowercase() {
                is_small_cave[b_idx] = true;
            }
        }
    }

    return part_1_2_walk(
        part_1,
        start_idx,
        end_idx,
        num_caves,
        &adjacancy_matrix,
        &is_small_cave,
        CaveWalkState::new(start_idx, num_caves),
    );
}

fn part_1(cave_system_str: &str) -> u32 {
    return part_1_2(cave_system_str, true);
}

fn part_2(cave_system_str: &str) -> u32 {
    return part_1_2(cave_system_str, false);
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT_A), 10);
}

#[test]
fn test_b() {
    assert_eq!(part_1(TEST_INPUT_B), 19);
}

#[test]
fn test_c() {
    assert_eq!(part_1(TEST_INPUT_C), 226);
}

#[test]
fn test_d() {
    assert_eq!(part_2(TEST_INPUT_A), 36);
}

#[test]
fn test_e() {
    assert_eq!(part_2(TEST_INPUT_B), 103);
}

#[test]
fn test_f() {
    assert_eq!(part_2(TEST_INPUT_C), 3509);
}

fn main() {
    println!("part 1: {0}", part_1(INPUT));
    println!("part 2: {0}", part_2(INPUT));
    println!("Done.");
}
