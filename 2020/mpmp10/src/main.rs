use std::collections::HashSet;

const D: usize = 4;
const EMPTY: u64 = 0;
const RED: u64 = 1;
const BLUE: u64 = 2;

struct Board {
    cells: [u64; D * D],
}

impl Board {
    fn empty() -> Board {
        Board {
            cells: [EMPTY; D * D],
        }
    }

    fn full() -> Board {
        Board {
            cells: [BLUE; D * D],
        }
    }

    fn my_hash(&self) -> u64 {
        self.cells
            .iter()
            .enumerate()
            .map(|(idx, c)| c << (idx * 2))
            .sum()
    }

    fn next(&mut self) -> bool {
        let mut idx = 0;
        while idx < D * D {
            let n = self.cells[idx] + 1;
            self.cells[idx] = n % 3;
            let overflow = (n / 3) > 0;
            if overflow {
                idx += 1;
            } else {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let mut checker = HashSet::new();
    let mut board = Board::empty();
    checker.insert(board.my_hash());

    loop {
        let overflow = board.next();
        let h = board.my_hash2();
        if checker.contains(&h) {
            println!("collision {}", h);
            break;
        }
        checker.insert(h);
        if overflow {
            break;
        }
    }

    println!("num hashes {}", checker.len());

    println!("done");
}
