use std::collections::{HashSet, VecDeque};

const THRESHOLD: usize = 4;

fn problem1(payload: &str) -> Option<usize> {
    let mut q = VecDeque::new();
    for (idx, c) in payload.chars().enumerate() {
        if q.len() == THRESHOLD {
            q.pop_front();
        }
        q.push_back(c);
        if q.len() == THRESHOLD && q.len() == q.clone().into_iter().collect::<HashSet<_>>().len() {
            return Some(idx + 1);
        }
    }
    None
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").unwrap();
    contents.lines().for_each(|case| {
        dbg!(case);
        dbg!(problem1(case).unwrap());
    });
}
