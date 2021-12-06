#[allow(unused)]
const TEST_INPUT_A: &str = "3";
#[allow(unused)]
const TEST_INPUT_B: &str = "3,4,3,1,2";
#[allow(unused)]
const INPUT: &str = "1,1,3,5,1,1,1,4,1,5,1,1,1,1,1,1,1,3,1,1,1,1,2,5,1,1,1,1,1,2,1,4,1,4,1,1,1,1,1,3,1,1,5,1,1,1,4,1,1,1,4,1,1,3,5,1,1,1,1,4,1,5,4,1,1,2,3,2,1,1,1,1,1,1,1,1,1,1,1,1,1,5,1,1,1,1,1,1,1,1,2,2,1,1,1,1,1,5,1,1,1,3,4,1,1,1,1,3,1,1,1,1,1,4,1,1,3,1,1,3,1,1,1,1,1,3,1,5,2,3,1,2,3,1,1,2,1,2,4,5,1,5,1,4,1,1,1,1,2,1,5,1,1,1,1,1,5,1,1,3,1,1,1,1,1,1,4,1,2,1,1,1,1,1,1,1,1,1,1,1,1,1,3,2,1,1,1,1,2,2,1,2,1,1,1,5,5,1,1,1,1,1,1,1,1,1,1,1,1,2,2,1,1,4,2,1,4,1,1,1,1,1,1,1,2,1,2,1,1,1,1,1,1,1,1,1,1,1,1,1,2,2,1,5,1,1,1,1,1,1,1,1,3,1,1,3,3,1,1,1,3,5,1,1,4,1,1,1,1,1,4,1,1,3,1,1,1,1,1,1,1,1,2,1,5,1,1,1,1,1,1,1,1,1,1,4,1,1,1,1";

#[test]
fn test_aa() {
    assert_eq!(part_1_2(TEST_INPUT_A, 1), 1);
}

#[test]
fn test_ab() {
    assert_eq!(part_1_2(TEST_INPUT_A, 2), 1);
}

#[test]
fn test_ac() {
    assert_eq!(part_1_2(TEST_INPUT_A, 3), 1);
}

#[test]
fn test_ad() {
    assert_eq!(part_1_2(TEST_INPUT_A, 4), 2);
}

#[test]
fn test_ae() {
    assert_eq!(part_1_2(TEST_INPUT_A, 5), 2);
}

#[test]
fn test_ba() {
    assert_eq!(part_1_2(TEST_INPUT_B, 1), 5);
}

#[test]
fn test_bb() {
    assert_eq!(part_1_2(TEST_INPUT_B, 2), 6);
}

#[test]
fn test_bc() {
    assert_eq!(part_1_2(TEST_INPUT_B, 3), 7);
}

#[test]
fn test_bd() {
    assert_eq!(part_1_2(TEST_INPUT_B, 4), 9);
}

#[test]
fn test_be() {
    assert_eq!(part_1_2(TEST_INPUT_B, 5), 10);
}

#[test]
fn test_bf() {
    assert_eq!(part_1_2(TEST_INPUT_B, 6), 10);
}

#[test]
fn test_bg() {
    assert_eq!(part_1_2(TEST_INPUT_B, 7), 10);
}

#[test]
fn test_bh() {
    assert_eq!(part_1_2(TEST_INPUT_B, 8), 10);
}

#[test]
fn test_bi() {
    assert_eq!(part_1_2(TEST_INPUT_B, 9), 11);
}

#[test]
fn test_bj() {
    assert_eq!(part_1_2(TEST_INPUT_B, 10), 12);
}

#[test]
fn test_bk() {
    assert_eq!(part_1_2(TEST_INPUT_B, 11), 15);
}

#[test]
fn test_bl() {
    assert_eq!(part_1_2(TEST_INPUT_B, 12), 17);
}

#[test]
fn test_bm() {
    assert_eq!(part_1_2(TEST_INPUT_B, 13), 19);
}

#[test]
fn test_bn() {
    assert_eq!(part_1_2(TEST_INPUT_B, 14), 20);
}

#[test]
fn test_bo() {
    assert_eq!(part_1_2(TEST_INPUT_B, 15), 20);
}

#[test]
fn test_bp() {
    assert_eq!(part_1_2(TEST_INPUT_B, 16), 21);
}

#[test]
fn test_bq() {
    assert_eq!(part_1_2(TEST_INPUT_B, 17), 22);
}

#[test]
fn test_br() {
    assert_eq!(part_1_2(TEST_INPUT_B, 18), 26);
}

#[test]
fn test_bs() {
    assert_eq!(part_1_2(TEST_INPUT_B, 80), 5934);
}

#[test]
fn test_ca() {
    assert_eq!(part_1_2(TEST_INPUT_B, 256), 26984457539);
}

fn part_1_2(starting_populatin: &str, days: u64) -> usize {
    let mut pop: Vec<usize> = vec![0; 9];

    for fish in starting_populatin
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
    {
        pop[fish] += 1;
    }

    for _ in 0..days {
        let births = pop[0];

        for idx in 0..8 {
            pop[idx] = pop[idx + 1];
        }

        pop[6] += births;
        pop[8] = births;
    }
    return pop.iter().sum();
}

fn main() {
    println!("part 1: {0}", part_1_2(INPUT, 80));
    println!("part 2: {0}", part_1_2(INPUT, 256));
    println!("Done.");
}
