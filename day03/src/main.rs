use std::collections::HashSet;

trait ToIndex {
    fn to_index(self) -> usize;
}

impl ToIndex for char {
    fn to_index(self) -> usize {
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .position(|v| v == self)
            .unwrap()
            + 1
    }
}

fn find_duplicated_item(rucksack: &str) -> Option<char> {
    let (first, second) = rucksack.split_at(rucksack.len() / 2);
    let map: HashSet<char> = first.chars().collect();
    for c in second.chars() {
        if map.contains(&c) {
            return Some(c);
        }
    }

    None
}

fn main() {
    let contents = std::fs::read_to_string("data/test.txt").unwrap();

    let response = contents
        .split("\n")
        .into_iter()
        .map(find_duplicated_item)
        .map(|c| c.unwrap())
        .map(|c| c.to_index())
        .sum::<usize>();

    println!("{}", response);
}
