// --- Day 6: Chronal Coordinates ---
// 
// The device on your wrist beeps several times, and once again you feel like you're falling.
// 
// "Situation critical," the device announces. "Destination indeterminate. Chronal interference detected. Please specify new target coordinates."
// 
// The device then produces a list of coordinates (your puzzle input). Are they places it thinks are safe or dangerous? It recommends you check manual page 729. The Elves did not give you a manual.
// 
// If they're dangerous, maybe you can minimize the danger by finding the coordinate that gives the largest distance from the other points.
// 
// Using only the Manhattan distance, determine the area around each coordinate by counting the number of integer X,Y locations that are closest to that coordinate (and aren't tied in distance to any other coordinate).
// 
// Your goal is to find the size of the largest area that isn't infinite. For example, consider the following list of coordinates:
// 
// 1, 1
// 1, 6
// 8, 3
// 3, 4
// 5, 5
// 8, 9
// 
// If we name these coordinates A through F, we can draw them on a grid, putting 0,0 at the top left:
// 
// ..........
// .A........
// ..........
// ........C.
// ...D......
// .....E....
// .B........
// ..........
// ..........
// ........F.
// 
// This view is partial - the actual grid extends infinitely in all directions. Using the Manhattan distance, each location's closest coordinate can be determined, shown here in lowercase:
// 
// aaaaa.cccc
// aAaaa.cccc
// aaaddecccc
// aadddeccCc
// ..dDdeeccc
// bb.deEeecc
// bBb.eeee..
// bbb.eeefff
// bbb.eeffff
// bbb.ffffFf
// 
// Locations shown as . are equally far from two or more coordinates, and so they don't count as being closest to any.
// 
// In this example, the areas of coordinates A, B, C, and F are infinite - while not shown here, their areas extend forever outside the visible grid. However, the areas of coordinates D and E are finite: D is closest to 9 locations, and E is closest to 17 (both including the coordinate's location itself). Therefore, in this example, the size of the largest area is 17.
// 
// What is the size of the largest area that isn't infinite?
// 
// To begin, get your puzzle input.

use std::fs;

struct Koordinate {
    x: u32,
    y: u32,
}

impl Koordinate {
    pub fn from_str(s: &String) -> Koordinate {
        let split: Vec<&str> = s.split(',').collect();
        return Koordinate {
            x: split[0].trim().parse().unwrap(),
            y: split[1].trim().parse().unwrap(),
        };
    }
}

fn read_file() -> Vec<String> {
    // return fs::read_to_string("input.txt").expect("bad file read").trim().lines().map(|s| String::from(s)).collect();
    return String::from("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9").lines().map(|s| String::from(s)).collect();
}

fn read_input() -> Vec<Koordinate> {
    read_file().iter().map(|line| Koordinate::from_str(line)).collect()
}

fn main() {
    // const X: usize = 12;
    // const Y: usize = 8;
    // let a = [[0u32; X]; Y];

    let mut a = vec![vec![0; 12]; 8];

    for x in 0..8 {
        // let mut s = String::from("");
        // let s = "";
        for y in 0..12 {
            // s.push_str(&a[x][y].to_string());
            // s.push_str(", ");
            print!("{}, ", a[x][y]);
        }
        println!("");
    }

    a[1][1] = 3;
    println!("");

    for x in 0..8 {
        // let mut s = String::from("");
        // let s = "";
        for y in 0..12 {
            // s.push_str(&a[x][y].to_string());
            // s.push_str(", ");
            print!("{}, ", a[x][y]);
        }
        println!("");
    }

    let input = read_input();
    for k in input {
        println!("{} <=> {}", k.x, k.y);
    }
    println!("Hello, world!");
}
