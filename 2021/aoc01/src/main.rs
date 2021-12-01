#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

fn parse_ints(depth_measurements: &str) -> Vec<i32> {
    depth_measurements
        .lines()
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn part_1(depth_measurements: Vec<i32>) -> u32 {
    let aa = depth_measurements.iter();
    let mut bb = depth_measurements.iter();
    bb.next();

    let mut result = 0;
    for (a, b) in aa.zip(bb) {
        if a < b {
            result += 1;
        }
    }
    return result;
}

fn part_2(depth_measurements: Vec<i32>) -> u32 {
    // I purposefully don't use .windows() here as that trivializes the challenge.

    let mut cum_sum = vec![0];
    let mut sum = 0;
    for dm in depth_measurements {
        sum += dm;
        cum_sum.push(sum);
    }

    let mut result = 0;
    for idx in 4..cum_sum.len() {
        let a = cum_sum[idx - 1] - cum_sum[idx - 4];
        let b = cum_sum[idx] - cum_sum[idx - 3];
        if a < b {
            result += 1;
        }
    }
    return result;
}

#[test]
fn test_a() {
    assert_eq!(part_1(parse_ints(TEST_INPUT)), 7);
}

#[test]
fn test_b() {
    assert_eq!(part_2(parse_ints(TEST_INPUT)), 5);
}

fn main() {
    println!("part 1: {0}", part_1(parse_ints(INPUT)));
    println!("part 2: {0}", part_2(parse_ints(INPUT)));
    println!("done");
}
