// --- Day 7: Handy Haversacks ---
//
// You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.
//
// Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!
//
// For example, consider the following rules:
//
// light red bags contain 1 bright white bag, 2 muted yellow bags.
// dark orange bags contain 3 bright white bags, 4 muted yellow bags.
// bright white bags contain 1 shiny gold bag.
// muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
// dark olive bags contain 3 faded blue bags, 4 dotted black bags.
// vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
// faded blue bags contain no other bags.
// dotted black bags contain no other bags.
//
// These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.
//
// You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)
//
// In the above rules, the following options would be available to you:
//
//     A bright white bag, which can hold your shiny gold bag directly.
//     A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
//     A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
//     A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
//
// So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.
//
// How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)
//
// To begin, get your puzzle input.
//
// --- Part Two ---
//
// It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!
//
// Consider again your shiny gold bag and the rules from the above example:
//
//     faded blue bags contain 0 other bags.
//     dotted black bags contain 0 other bags.
//     vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
//     dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.
//
// So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!
//
// Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!
//
// Here's another example:
//
// shiny gold bags contain 2 dark red bags.
// dark red bags contain 2 dark orange bags.
// dark orange bags contain 2 dark yellow bags.
// dark yellow bags contain 2 dark green bags.
// dark green bags contain 2 dark blue bags.
// dark blue bags contain 2 dark violet bags.
// dark violet bags contain no other bags.
//
// In this example, a single shiny gold bag must contain 126 other bags.
//
// How many individual bags are required inside your single shiny gold bag?

use regex::Regex;
use std::collections::{HashMap, HashSet};

#[allow(unused)]
const TEST_INPUT_A: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

#[allow(unused)]
const INPUT: &str = include_str!("input");

struct FloydWarshallInput {
    vertices: Vec<String>,
    edges: Vec<(String, String)>,
}

impl FloydWarshallInput {
    fn new(vertices: Vec<String>, edges: Vec<(String, String)>) -> Self {
        Self { vertices, edges }
    }
}

fn parse(regulations: &str) -> Option<FloydWarshallInput> {
    let not_contains_regex: Regex =
        Regex::new(r"(\w+\s+\w+)\s+bags\s+contain\s+no\s+other\s+bags.").ok()?;
    let start_contains_regex: Regex = Regex::new(r"(\w+\s+\w+)\s+bags\s+contain\s+").ok()?;
    let continue_contains_regex: Regex = Regex::new(r"(\d+)\s+(\w+\s+\w+)\s+bag").ok()?;

    let mut vertices: HashSet<String> = HashSet::new();
    let mut edges: Vec<(String, String)> = Vec::new();

    for line in regulations.lines() {
        // println!("{}", line);
        if not_contains_regex.is_match(line) {
            // println!("no bags in bags");
            continue;
        }
        let captures = start_contains_regex.captures(line)?;
        let edge_start: &str = captures.get(1)?.as_str();
        vertices.insert(edge_start.to_string());
        //println!("s {}", captures.get(1)?.as_str());

        for captures in continue_contains_regex.captures_iter(line) {
            let edge_end: &str = captures.get(2)?.as_str();
            vertices.insert(edge_end.to_string());
            edges.push((edge_start.to_string(), edge_end.to_string()));

            // println!(
            //     "c {}|{}",
            //     captures.get(1)?.as_str(),
            //     captures.get(2)?.as_str()
            // );
        }
    }

    return Some(FloydWarshallInput::new(
        vertices.into_iter().collect(),
        edges,
    ));
}

#[allow(unused)]
fn part_1(regulations: &str) -> Option<usize> {
    // We solve this by treating bag regulations like an directed graph.
    // Then we find all distances in the graph using the floyd warshall algorithm.
    // This tells us which vertices of the graph can reach others.
    // Which allows us to count the solution.
    let graph = parse(regulations)?;
    let num_vertices = graph.vertices.len();
    let vertice_2_idx: HashMap<String, usize> = graph
        .vertices
        .iter()
        .enumerate()
        .map(|(idx, s)| (s.clone(), idx))
        .collect();

    let mut distances: Vec<Vec<usize>> = Vec::new();
    for idx in 0..num_vertices {
        distances.push(vec![std::usize::MAX; num_vertices]);
        distances[idx][idx] = 0;
    }

    let edge_distance: usize = 1;
    for (start, end) in graph.edges.iter() {
        let start: usize = *vertice_2_idx.get(start)?;
        let end: usize = *vertice_2_idx.get(end)?;
        distances[start][end] = edge_distance;
    }

    for k in 0..num_vertices {
        for i in 0..num_vertices {
            for j in 0..num_vertices {
                if distances[i][k] != std::usize::MAX
                    && distances[k][j] != std::usize::MAX
                    && distances[i][j] > distances[i][k] + distances[k][j]
                {
                    distances[i][j] = distances[i][k] + distances[k][j];
                }
            }
        }
    }

    let target_vertices = "shiny gold";
    let target_vertice_idx = *vertice_2_idx.get(target_vertices)?;
    let mut res = 0;
    for start_idx in 0..num_vertices {
        if start_idx == target_vertice_idx {
            continue;
        }
        if distances[start_idx][target_vertice_idx] < std::usize::MAX {
            res += 1;
        }
    }

    return Some(res);
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT_A), Some(4))
}

fn main() {
    match part_1(INPUT) {
        Some(p) => {
            println!("Part 1: {}", p);
        }
        None => {
            println!("Part 1 parse error");
        }
    }
    println!("done");
}
