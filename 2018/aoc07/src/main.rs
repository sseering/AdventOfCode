// --- Day 7: The Sum of Its Parts ---
//
// You find yourself standing on a snow-covered coastline; apparently, you landed a little off course. The region is too hilly to see the North Pole from here, but you do spot some Elves that seem to be trying to unpack something that washed ashore. It's quite cold out, so you decide to risk creating a paradox by asking them for directions.
//
// "Oh, are you the search party?" Somehow, you can understand whatever Elves from the year 1018 speak; you assume it's Ancient Nordic Elvish. Could the device on your wrist also be a translator? "Those clothes don't look very warm; take this." They hand you a heavy coat.
//
// "We do need to find our way back to the North Pole, but we have higher priorities at the moment. You see, believe it or not, this box contains something that will solve all of Santa's transportation problems - at least, that's what it looks like from the pictures in the instructions." It doesn't seem like they can read whatever language it's in, but you can: "Sleigh kit. Some assembly required."
//
// "'Sleigh'? What a wonderful name! You must help us assemble this 'sleigh' at once!" They start excitedly pulling more parts out of the box.
//
// The instructions specify a series of steps and requirements about which steps must be finished before others can begin (your puzzle input). Each step is designated by a single letter. For example, suppose you have the following instructions:
//
// Step C must be finished before step A can begin.
// Step C must be finished before step F can begin.
// Step A must be finished before step B can begin.
// Step A must be finished before step D can begin.
// Step B must be finished before step E can begin.
// Step D must be finished before step E can begin.
// Step F must be finished before step E can begin.
//
// Visually, these requirements look like this:
//
//   -->A--->B--
//  /    \      \
// C      -->D----->E
//  \           /
//   ---->F-----
//
// Your first goal is to determine the order in which the steps should be completed. If more than one step is ready, choose the step which is first alphabetically. In this example, the steps would be completed as follows:
//
//     Only C is available, and so it is done first.
//     Next, both A and F are available. A is first alphabetically, so it is done next.
//     Then, even though F was available earlier, steps B and D are now also available, and B is the first alphabetically of the three.
//     After that, only D and F are available. E is not available because only some of its prerequisites are complete. Therefore, D is completed next.
//     F is the only choice, so it is done next.
//     Finally, E is completed.
//
// So, in this example, the correct order is CABDFE.
//
// In what order should the steps in your instructions be completed?
//
// The first half of this puzzle is complete! It provides one gold star: *
//
// --- Part Two ---
//
// As you're about to begin construction, four of the Elves offer to help. "The sun will set soon; it'll go faster if we work together." Now, you need to account for multiple people working on steps simultaneously. If multiple steps are available, workers should still begin them in alphabetical order.
//
// Each step takes 60 seconds plus an amount corresponding to its letter: A=1, B=2, C=3, and so on. So, step A takes 60+1=61 seconds, while step Z takes 60+26=86 seconds. No time is required between steps.
//
// To simplify things for the example, however, suppose you only have help from one Elf (a total of two workers) and that each step takes 60 fewer seconds (so that step A takes 1 second and step Z takes 26 seconds). Then, using the same instructions as above, this is how each second would be spent:
//
// Second   Worker 1   Worker 2   Done
//    0        C          .
//    1        C          .
//    2        C          .
//    3        A          F       C
//    4        B          F       CA
//    5        B          F       CA
//    6        D          F       CAB
//    7        D          F       CAB
//    8        D          F       CAB
//    9        D          .       CABF
//   10        E          .       CABFD
//   11        E          .       CABFD
//   12        E          .       CABFD
//   13        E          .       CABFD
//   14        E          .       CABFD
//   15        .          .       CABFDE
//
// Each row represents one second of time. The Second column identifies how many seconds have passed as of the beginning of that second. Each worker column shows the step that worker is currently doing (or . if they are idle). The Done column shows completed steps.
//
// Note that the order of the steps has changed; this is because steps now take time to finish and multiple workers can begin multiple steps simultaneously.
//
// In this example, it would take 15 seconds for two workers to complete these steps.
//
// With 5 workers and the 60+ second step durations described above, how long will it take to complete all of the steps?

#[allow(unused)]
use std::cmp::Ordering;
#[allow(unused)]
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[allow(unused)]
const TEST_INPUT: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

#[allow(unused)]
const INPUT: &str = include_str!("input");

#[derive(Debug, PartialEq, Eq, Hash)]
struct StepOrder {
    earlier: char,
    later: char,
}

impl StepOrder {
    fn new(earlier: char, later: char) -> Self {
        Self { earlier, later }
    }
}

impl FromStr for StepOrder {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if &input[0..5] != "Step "
            || &input[6..36] != " must be finished before step "
            || &input[37..] != " can begin."
        {
            return Err(());
        }

        let earlier: char = input[5..=5].chars().next().ok_or(())?;
        let later: char = input[36..=36].chars().next().ok_or(())?;
        return Ok(StepOrder::new(earlier, later));
    }
}

fn instructions_2_step_orders(instructions: &str) -> (char, char, Vec<StepOrder>) {
    let mut min = std::char::MAX;
    let mut max = '\0';
    let step_orders = instructions
        .lines()
        .filter_map(|line| -> Option<StepOrder> {
            let o_so = StepOrder::from_str(line).ok();
            if let Some(so) = &o_so {
                min = std::cmp::min(min, so.earlier);
                min = std::cmp::min(min, so.later);
                max = std::cmp::max(max, so.earlier);
                max = std::cmp::max(max, so.later);
            }
            return o_so;
        })
        .collect();
    return (min, max, step_orders);
}

fn get_available_steps(
    all_steps_sorted: &Vec<char>,
    step_orders: &Vec<StepOrder>,
    already_done_steps: &HashSet<char>,
) -> Vec<char> {
    let mut res = Vec::new();
    'nextchar: for &c in all_steps_sorted.iter() {
        if already_done_steps.contains(&c) {
            continue;
        }
        // This for loop could be faster with a multi hash map. But there is none in the std-lib and I don't want to use external dependencies.
        for so in step_orders.iter() {
            if so.later != c {
                continue;
            }
            if !already_done_steps.contains(&so.earlier) {
                continue 'nextchar;
            }
        }

        res.push(c);
    }

    return res;
}

#[allow(unused)]
fn part_1(instructions: &str) -> String {
    let (min, max, step_orders) = instructions_2_step_orders(instructions);

    let mut done_steps: HashSet<char> = HashSet::new();

    let mut res: Vec<char> = Vec::new();
    let mut all_steps_sorted: Vec<char> = (min..=max).collect();
    all_steps_sorted.sort();

    let mut endless_loop_check = 0;
    while all_steps_sorted.len() > res.len() && endless_loop_check < 0xFFF0 {
        endless_loop_check += 1;
        let available_steps = get_available_steps(&all_steps_sorted, &step_orders, &done_steps);
        let &next_step = available_steps.first().unwrap();
        res.push(next_step);
        done_steps.insert(next_step);
    }

    return res
        .into_iter()
        .map(|c| String::from(c))
        .collect::<Vec<String>>()
        .concat();
}

fn step_dutaion(step: char, short_work: bool) -> u32 {
    return u32::from(step) - u32::from('A') + if short_work { 1 } else { 61 };
}

struct Part2Event {
    step_done: char,
    time_done: u32,
}

impl Part2Event {
    fn new(step_done: char, time_done: u32) -> Self {
        Self {
            step_done,
            time_done,
        }
    }
}

struct Part2EventQeue {
    q: VecDeque<Part2Event>,
    steps_in_progress: HashSet<char>,
}

impl Part2EventQeue {
    fn new() -> Self {
        Self {
            q: VecDeque::new(),
            steps_in_progress: HashSet::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.q.is_empty()
    }

    fn enqueue_keep_sort_order(&mut self, insert_this: Part2Event) {
        let mut insert_idx = self.q.len();
        for (idx, event) in self.q.iter().enumerate() {
            if event.time_done > insert_this.time_done {
                insert_idx = idx;
            }
        }
        self.steps_in_progress.insert(insert_this.step_done);
        self.q.insert(insert_idx, insert_this);
    }

    fn pop_front(&mut self) -> Part2Event {
        let res = self.q.pop_front().unwrap();
        self.steps_in_progress.remove(&res.step_done);
        return res;
    }

    fn is_in_progress(&self, step: &char) -> bool {
        self.steps_in_progress.contains(step)
    }
}

#[allow(unused)]
fn part_2(instructions: &str, mut num_available_workers: u32, short_work: bool) -> u32 {
    let (min, max, step_orders) = instructions_2_step_orders(instructions);

    let mut all_steps_sorted: Vec<char> = (min..=max).collect();
    all_steps_sorted.sort();

    let mut done_steps: HashSet<char> = HashSet::new();
    let mut event_q = Part2EventQeue::new();

    // find inital steps
    let mut possible_initial_steps =
        get_available_steps(&all_steps_sorted, &step_orders, &done_steps);
    for step in possible_initial_steps.into_iter() {
        if num_available_workers <= 0 {
            break;
        }
        event_q.enqueue_keep_sort_order(Part2Event::new(step, step_dutaion(step, short_work)));
        num_available_workers -= 1;
    }

    let mut latest_finish_seen = 0;

    let mut infinite_loop_check = 0;
    while !event_q.is_empty() && infinite_loop_check < 0xFFF0 {
        infinite_loop_check += 1;

        let done = event_q.pop_front();
        done_steps.insert(done.step_done);
        num_available_workers += 1;
        latest_finish_seen = std::cmp::max(latest_finish_seen, done.time_done);

        let mut possible_next_steps =
            get_available_steps(&all_steps_sorted, &step_orders, &done_steps);

        for step in possible_next_steps.into_iter() {
            if num_available_workers <= 0 {
                break;
            }
            if event_q.is_in_progress(&step) {
                continue;
            }

            event_q.enqueue_keep_sort_order(Part2Event::new(
                step,
                done.time_done + step_dutaion(step, short_work),
            ));
            num_available_workers -= 1;
        }
    }

    return latest_finish_seen;
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), "CABDFE");
}

#[test]
fn test_b() {
    let (min, max, sos) = instructions_2_step_orders(TEST_INPUT);
    assert_eq!(min, 'A');
    assert_eq!(max, 'F');
    let mut iter = sos.into_iter();
    assert_eq!(iter.next(), Some(StepOrder::new('C', 'A')));
    assert_eq!(iter.next(), Some(StepOrder::new('C', 'F')));
    assert_eq!(iter.next(), Some(StepOrder::new('A', 'B')));
    assert_eq!(iter.next(), Some(StepOrder::new('A', 'D')));
    assert_eq!(iter.next(), Some(StepOrder::new('B', 'E')));
    assert_eq!(iter.next(), Some(StepOrder::new('D', 'E')));
    assert_eq!(iter.next(), Some(StepOrder::new('F', 'E')));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_c() {
    assert_eq!(part_2(TEST_INPUT, 2, true), 15);
}

#[test]
fn test_d() {
    assert_eq!(step_dutaion('A', false), 61);
    assert_eq!(step_dutaion('B', false), 62);
    assert_eq!(step_dutaion('C', false), 63);
    assert_eq!(step_dutaion('Z', false), 86);
    assert_eq!(step_dutaion('A', true), 1);
    assert_eq!(step_dutaion('B', true), 2);
    assert_eq!(step_dutaion('C', true), 3);
    assert_eq!(step_dutaion('Z', true), 26);
}

fn main() {
    let p1 = part_1(INPUT);
    println!("Part 1: {}", p1);
    let p2 = part_2(INPUT, 5, false);
    println!("Part 2: {}", p2);
    println!("done");
}
