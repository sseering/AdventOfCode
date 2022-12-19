#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

#[allow(unused)]
const TEST_INPUT: &str = include_str!("../test-input.txt");

pub mod filesystem {
    use std::collections::VecDeque;

    pub struct Node {
        first_child: Option<Box<Node>>,
        next_sibling: Option<Box<Node>>,
        name: String,
        size: usize,
    }

    impl Node {
        pub fn create_root() -> Self {
            Self {
                first_child: None,
                next_sibling: None,
                name: String::from("/"),
                size: 0,
            }
        }

        fn add_sibling(&mut self, name: String, size: usize) {
            match &mut self.next_sibling {
                Some(ns) => {
                    ns.add_sibling(name, size);
                }
                None => {
                    let new = Self {
                        first_child: None,
                        next_sibling: None,
                        name,
                        size,
                    };

                    self.next_sibling = Some(Box::new(new));
                }
            }
        }

        fn add_child(&mut self, name: String, size: usize) {
            match &mut self.first_child {
                Some(fs) => {
                    fs.add_sibling(name, size);
                }
                None => {
                    let new = Self {
                        first_child: None,
                        next_sibling: None,
                        name,
                        size,
                    };

                    self.first_child = Some(Box::new(new));
                }
            }
        }

        pub fn add_directory(&mut self, name: &str) {
            self.add_child(String::from(name), 0);
        }

        pub fn add_file(&mut self, name: &str, size: usize) {
            self.add_child(String::from(name), size);
        }

        pub fn cd(&mut self, path: &Vec<String>) -> Option<&mut Self> {
            return self.internal_cd(path, 0);
        }

        fn internal_cd(&mut self, path: &Vec<String>, path_idx: usize) -> Option<&mut Self> {
            if path[path_idx] == self.name {
                let idx_p1 = path_idx + 1;
                if idx_p1 >= path.len() {
                    return Some(self);
                }

                match &mut self.first_child {
                    Some(fc) => {
                        return fc.internal_cd(path, idx_p1);
                    }
                    None => {
                        return None;
                    }
                }
            }

            match &mut self.next_sibling {
                Some(ns) => {
                    return ns.internal_cd(path, path_idx);
                }
                None => {
                    return None;
                }
            }
        }

        pub fn is_file(&self) -> bool {
            return self.size > 0;
        }

        pub fn is_directory(&self) -> bool {
            return self.size == 0;
        }

        pub fn disk_usage(&self) -> usize {
            TreeIterator::new(self).map(|n| n.size).sum()
        }
    }

    pub struct TreeIterator<'a> {
        tree_nodes: VecDeque<&'a Node>,
    }

    impl<'a> TreeIterator<'a> {
        pub fn new(root: &'a Node) -> Self {
            let mut tree_nodes = VecDeque::from([root]);
            let mut child = &root.first_child;
            while let Some(c) = child {
                tree_nodes.push_back(c);
                child = &c.first_child;
            }
            Self { tree_nodes }
        }
    }

    impl<'a> Iterator for TreeIterator<'a> {
        type Item = &'a Node;

        fn next(&mut self) -> Option<Self::Item> {
            let current = self.tree_nodes.pop_back()?;
            if !self.tree_nodes.is_empty() {
                if let Some(ns) = &current.next_sibling {
                    self.tree_nodes.push_back(ns);
                    let mut child = &ns.first_child;
                    while let Some(c) = child {
                        self.tree_nodes.push_back(c);
                        child = &c.first_child;
                    }
                }
            }
            return Some(current);
        }
    }
}

fn parse(cmdline_output: &str) -> Option<filesystem::Node> {
    let mut root = filesystem::Node::create_root();
    let mut cwd: Vec<String> = Vec::new();
    for line in cmdline_output.lines() {
        if line.trim().len() <= 0 {
            continue;
        }

        if line == "$ cd /" {
            cwd = vec![String::from("/")];
        } else if line == "$ ls" {
            // nothing
        } else if line == "$ cd .." {
            if cwd.len() < 2 {
                return None;
            }
            cwd.pop();
        } else if line.starts_with("$ cd ") {
            let dir = line.split_whitespace().nth(2)?;
            cwd.push(String::from(dir));
        } else if line.starts_with("dir ") {
            let dir = line.split_whitespace().nth(1)?;
            root.cd(&cwd)?.add_directory(dir);
        } else {
            let mut split = line.split_whitespace();
            let size: usize = split.next()?.parse().ok()?;
            let fname = split.next()?;
            root.cd(&cwd)?.add_file(fname, size);
        }
    }

    return Some(root);
}

fn part_1(cmdline_output: &str) -> Option<usize> {
    let root = parse(cmdline_output)?;

    return Some(
        filesystem::TreeIterator::new(&root)
            .filter_map(|n| {
                if n.is_directory() {
                    Some(n.disk_usage())
                } else {
                    None
                }
            })
            .filter(|&s| s < 100000)
            .sum(),
    );
}

fn part_2(cmdline_output: &str) -> Option<usize> {
    let root = parse(cmdline_output)?;

    let disk_size: usize = 70000000;
    let needed: usize = 30000000;
    let free = disk_size - root.disk_usage();
    let missing = needed - free;

    let mut found = usize::MAX;

    filesystem::TreeIterator::new(&root).for_each(|n| {
        if n.is_file() {
            return;
        }
        let siz = n.disk_usage();
        if siz >= missing && siz < found {
            found = siz;
        }
    });

    return if found < usize::MAX {
        Some(found)
    } else {
        None
    };
}

#[test]
fn test_a() {
    assert_eq!(part_1(TEST_INPUT), Some(95437));
}

#[test]
fn test_ba() {
    let root = parse(TEST_INPUT);
    assert!(root.is_some());
    assert_eq!(root.unwrap().disk_usage(), 48381165);
}

#[test]
fn test_bb() {
    assert_eq!(part_2(TEST_INPUT), Some(24933642));
}

fn main() {
    match part_1(INPUT) {
        Some(r) => {
            println!("part 1: {}", r);
        }
        None => {
            println!("part 1 failed.")
        }
    }
    match part_2(INPUT) {
        Some(r) => {
            println!("part 2: {}", r);
        }
        None => {
            println!("part 2 failed.")
        }
    }
    println!("done.");
}
