// --- Day 3: Toboggan Trajectory ---
//
// With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan might be easy, it's certainly not safe: there's very minimal steering and the area is covered in trees. You'll need to see which angles will take you near the fewest trees.
//
// Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You make a map (your puzzle input) of the open squares (.) and trees (#) you can see. For example:
//
// ..##.......
// #...#...#..
// .#....#..#.
// ..#.#...#.#
// .#...##..#.
// ..#.##.....
// .#.#.#....#
// .#........#
// #.##...#...
// #...##....#
// .#..#...#.#
//
// These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability, the same pattern repeats to the right many times:
//
// ..##.........##.........##.........##.........##.........##.......  --->
// #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
// .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
// ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
// .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
// ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
// .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
// .#........#.#........#.#........#.#........#.#........#.#........#
// #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
// #...##....##...##....##...##....##...##....##...##....##...##....#
// .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//
// You start on the open square (.) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).
//
// The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start by counting all the trees you would encounter for the slope right 3, down 1:
//
// From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.
//
// The locations you'd check in the above example are marked here with O where there was an open square and X where there was a tree:
//
// ..##.........##.........##.........##.........##.........##.......  --->
// #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
// .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
// ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
// .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
// ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
// .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
// .#........#.#........X.#........#.#........#.#........#.#........#
// #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
// #...##....##...##....##...#X....##...##....##...##....##...##....#
// .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//
// In this example, traversing the map using this slope would cause you to encounter 7 trees.
//
// Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?
//
// To begin, get your puzzle input.
//
// The first half of this puzzle is complete! It provides one gold star: *
// --- Part Two ---
//
// Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.
//
// Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:
//
//     Right 1, down 1.
//     Right 3, down 1. (This is the slope you already checked.)
//     Right 5, down 1.
//     Right 7, down 1.
//     Right 1, down 2.
//
// In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.
//
// What do you get if you multiply together the number of trees encountered on each of the listed slopes?

#[allow(unused)]
const TEST_INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

#[allow(unused)]
const INPUT: &str = include_str!("input");

fn part_1_2(map: &str, slopes: Vec<(usize, usize)>) -> usize {
    // map has width and height
    // x is a horizontal dimension and grows to the right
    // y is a vertival dimension and grows to the bottom

    let trees: Vec<Vec<bool>> = map // dimensions: [height][width]
        .lines()
        .map(|line| -> Vec<bool> {
            return line.chars().map(|c| c == '#').collect();
        })
        .collect();

    let map_height = trees.len();
    let map_width = trees[0].len();

    let mut trees_met_product = 1;
    for movement_vec in slopes {
        let mut trees_met = 0;
        let mut pos = (0, 0); // [x][y]

        pos = (pos.0 + movement_vec.0, pos.1 + movement_vec.1);
        while pos.1 < map_height {
            if trees[pos.1][pos.0 % map_width] {
                trees_met += 1;
            }
            pos = (pos.0 + movement_vec.0, pos.1 + movement_vec.1);
        }

        trees_met_product *= trees_met;
    }

    return trees_met_product;
}

fn part_1(map: &str) -> usize {
    return part_1_2(map, vec![(3, 1)]);
}

fn part_2(map: &str) -> usize {
    return part_1_2(map, vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]);
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), 7);
}

#[test]
fn test_b() {
    assert_eq!(part_2(TEST_INPUT), 336);
}

fn main() {
    println!("Part 1 {}", part_1(INPUT));
    println!("Part 2 {}", part_2(INPUT));
    println!("done");
}
