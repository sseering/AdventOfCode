use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::OpenOptions;
use std::io::Write;
use std::mem::replace;

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_a() {
        let c = part_1_e(TEST_INPUT, 10);
        assert!(c.is_some());
        let c = c.unwrap();

        let counter = c.part_1_count();

        let mut expected = HashMap::new();
        expected.insert(5, 1);
        expected.insert(4, 1);
        expected.insert(2, 2);
        expected.insert(1, 7);
        expected.insert(0, 9);

        assert_eq!(counter, expected);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_1(TEST_INPUT, 10), Some(40));
    }

    #[test]
    fn test_c() {
        assert_eq!(part_2(TEST_INPUT), Some(25272));
    }
}

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Coord3d {
    x: u64,
    y: u64,
    z: u64,
}

impl TryFrom<&str> for Coord3d {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut spl = value.trim().split(',');

        let x: u64 = spl.next().ok_or(())?.parse().map_err(|_| ())?;
        let y: u64 = spl.next().ok_or(())?.parse().map_err(|_| ())?;
        let z: u64 = spl.next().ok_or(())?.parse().map_err(|_| ())?;

        Ok(Self { x, y, z })
    }
}

impl Coord3d {
    fn straight_line_distance_squared(&self, other: &Self) -> u64 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);
        return dx * dx + dy * dy + dz * dz;
    }
}

fn parse_1_2(input: &str) -> Option<Vec<Coord3d>> {
    let res: Option<Vec<Coord3d>> = input
        .lines()
        .filter_map(|s| {
            let t = s.trim();
            return if t.is_empty() { None } else { Some(t) };
        })
        .map(|l| {
            let c: Result<Coord3d, _> = l.try_into();
            return c.ok();
        })
        .collect();

    return res;
}

#[derive(Debug)]
struct Circuits {
    // A list of circuits.
    // c[circuit_idx] is a circuit (which is a list of junction boxes).
    // c[circuit_idx][n] gives an index into the list of all junction boxes.
    // junction_boxes[c[circuit_idx][0]] will give the 3D coords of the first junction box in this circuit.
    c: Vec<Vec<usize>>,

    // In which circuit can I find a specific junction box?
    // idx2c[junction_idx] will return the index of the cicuit that contains this junction box.
    // c[idx2c[junction_idx]].contains(junction_idx) is always true.
    idx2c: HashMap<usize, usize>,

    // How many non empty circuits are there?
    // How many sublists of c are not empty?
    num_circuits: usize,
}

impl From<&Vec<Coord3d>> for Circuits {
    fn from(value: &Vec<Coord3d>) -> Self {
        let l = value.len();
        let mut c = Vec::with_capacity(l);
        let mut idx2c = HashMap::with_capacity(l);
        for idx in 0..l {
            c.push(vec![idx]);
            idx2c.insert(idx, idx);
        }
        Self {
            c,
            idx2c,
            num_circuits: l,
        }
    }
}

impl Circuits {
    fn connect(&mut self, junction_idx_a: usize, junction_idx_b: usize) {
        let ca: usize = self.idx2c[&junction_idx_a];
        let cb: usize = self.idx2c[&junction_idx_b];
        if ca == cb {
            // Already in the same circuit. Nothing to do.
            return;
        }

        let mut moving = replace(&mut self.c[cb], Vec::new());
        if moving.len() > 0 {
            self.num_circuits -= 1;
        }
        for &jidx in &moving {
            self.idx2c.insert(jidx, ca);
        }
        self.c[ca].append(&mut moving);
    }

    fn part_1_count(&self) -> HashMap<usize, u64> {
        let mut counter = HashMap::new();
        for l in &self.c {
            *counter.entry(l.len()).or_insert(0) += 1;
        }

        return counter;
    }
}

#[derive(PartialEq, Eq)]
struct JunctionBoxPair {
    idx_a: usize,
    idx_b: usize,
    dist: u64,
}

impl Ord for JunctionBoxPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.dist.cmp(&other.dist);
    }
}

impl PartialOrd<JunctionBoxPair> for JunctionBoxPair {
    fn partial_cmp(&self, other: &JunctionBoxPair) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

fn all_distances(junction_boxes: &Vec<Coord3d>) -> BinaryHeap<Reverse<JunctionBoxPair>> {
    let l = junction_boxes.len();
    let mut res = BinaryHeap::with_capacity(l * l);

    for idx_a in 0..(l - 1) {
        for idx_b in (idx_a + 1)..l {
            let dist = junction_boxes[idx_a].straight_line_distance_squared(&junction_boxes[idx_b]);
            res.push(Reverse(JunctionBoxPair { idx_a, idx_b, dist }));
        }
    }
    return res;
}

fn part_1_e(input: &str, num_steps: usize) -> Option<Circuits> {
    let junction_boxes = parse_1_2(input)?;
    let mut dists = all_distances(&junction_boxes);

    let mut circuits: Circuits = (&junction_boxes).into();

    for _ in 0..num_steps {
        let lowest_dist = dists.pop()?.0;
        // println!(
        //     "combining dist {} idx {} {} coords {:?} {:?}",
        //     lowest_dist.dist,
        //     lowest_dist.idx_a,
        //     lowest_dist.idx_b,
        //     junction_boxes[lowest_dist.idx_a],
        //     junction_boxes[lowest_dist.idx_b]
        // );

        circuits.connect(lowest_dist.idx_a, lowest_dist.idx_b);
    }

    return Some(circuits);
}

fn part_1(input: &str, num_steps: usize) -> Option<usize> {
    let c = part_1_e(input, num_steps)?;

    let mut l1: usize = 0;
    let mut l2: usize = 0;
    let mut l3: usize = 0;

    for k in c.part_1_count().keys() {
        let mut v = *k;
        if v >= l1 {
            (l1, v) = (v, l1);
        }
        if v >= l2 {
            (l2, v) = (v, l2);
        }
        if v > l3 {
            l3 = v;
        }
    }

    return Some(l1 * l2 * l3);
}

fn part_2_e<F>(input: &str, mut connection_callback: F) -> Option<u64>
where
    F: FnMut(&Coord3d, &Coord3d),
{
    let junction_boxes = parse_1_2(input)?;
    let mut dists = all_distances(&junction_boxes);

    let mut circuits: Circuits = (&junction_boxes).into();

    loop {
        let lowest_dist = dists.pop()?.0;

        connection_callback(
            &junction_boxes[lowest_dist.idx_a],
            &junction_boxes[lowest_dist.idx_b],
        );

        circuits.connect(lowest_dist.idx_a, lowest_dist.idx_b);

        if circuits.num_circuits == 1 {
            return Some(junction_boxes[lowest_dist.idx_a].x * junction_boxes[lowest_dist.idx_b].x);
        }
    }
}

fn part_2(input: &str) -> Option<u64> {
    return part_2_e(input, |_, _| { /* nothing */ });
}

#[allow(unused)]
fn junction_boxes_to_javascript() {
    // for visualization with three.js

    let junction_boxes = parse_1_2(INPUT).unwrap();

    let mut xs = String::from("const xs=[");
    let mut ys = String::from("const ys=[");
    let mut zs = String::from("const zs=[");

    let mut minx: u64 = u64::MAX;
    let mut miny: u64 = u64::MAX;
    let mut minz: u64 = u64::MAX;
    let mut maxx: u64 = 0;
    let mut maxy: u64 = 0;
    let mut maxz: u64 = 0;

    for jb in &junction_boxes {
        minx = u64::min(jb.x, minx);
        miny = u64::min(jb.y, miny);
        minz = u64::min(jb.z, minz);
        maxx = u64::max(jb.x, maxx);
        maxy = u64::max(jb.y, maxy);
        maxz = u64::max(jb.z, maxz);
    }

    println!("min x {} y {} z{}", minx, miny, minz);
    println!("max x {} y {} z{}", maxx, maxy, maxz);

    let xnums: String = junction_boxes
        .iter()
        .map(|jb| jb.x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let ynums: String = junction_boxes
        .iter()
        .map(|jb| jb.y.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let znums: String = junction_boxes
        .iter()
        .map(|jb| jb.z.to_string())
        .collect::<Vec<String>>()
        .join(",");

    xs.push_str(&xnums);
    ys.push_str(&ynums);
    zs.push_str(&znums);
    xs.push_str("];");
    ys.push_str("];");
    zs.push_str("];");

    let mut connections = String::from("const connections = [");
    let mut got_first_connection = false;
    let connection_callback = |a: &Coord3d, b: &Coord3d| {
        if got_first_connection {
            connections.push_str(",");
        } else {
            got_first_connection = true;
        }
        connections.push_str(&format!(
            "[{},{},{},{},{},{}]",
            a.x, a.y, a.z, b.x, b.y, b.z
        ));
    };
    part_2_e(INPUT, connection_callback);
    connections.push_str("];");

    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("/tmp/junction_boxes_hardcodede.js")
        .unwrap();
    f.write_all(xs.as_bytes()).unwrap();
    f.write_all(ys.as_bytes()).unwrap();
    f.write_all(zs.as_bytes()).unwrap();
    f.write_all(connections.as_bytes()).unwrap();
    f.write_all(format!("const num_junction_boxes = {};", junction_boxes.len()).as_bytes())
        .unwrap();
    f.write_all(b"export {xs,ys,zs,num_junction_boxes,connections};")
        .unwrap();
}

fn main() {
    junction_boxes_to_javascript();

    match part_1(INPUT, 1000) {
        Some(answer) => {
            println!("part 1: {0}", answer);
        }
        None => {
            println!("part 1 failed");
        }
    }
    match part_2(INPUT) {
        Some(answer) => {
            println!("part 2: {0}", answer);
        }
        None => {
            println!("part 2 failed");
        }
    }

    println!("Done.");
}
