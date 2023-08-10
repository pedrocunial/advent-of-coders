use std::collections::{HashSet, VecDeque};

fn problem(payload: &str, threshold: usize) -> Option<usize> {
    let mut q = VecDeque::new();
    for (idx, c) in payload.chars().enumerate() {
        if q.len() == threshold {
            q.pop_front();
        }
        q.push_back(c);
        if q.len() == threshold && q.len() == q.clone().into_iter().collect::<HashSet<_>>().len() {
            return Some(idx + 1);
        }
    }
    None
}

fn main() {
    let contents = std::fs::read_to_string("data/test.txt").unwrap();
    contents.lines().for_each(|case| {
        dbg!(case);
        dbg!(problem(case, 4).unwrap());
        dbg!(problem(case, 14).unwrap());
    });
}
