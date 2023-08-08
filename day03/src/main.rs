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

fn find_badge(sacks: Vec<&str>) -> Option<char> {
    if let [first, second, third] = sacks[..] {
        let first_map = first.chars().collect::<HashSet<_>>();
        let second_map = second.chars().collect::<HashSet<_>>();

        for c in third.chars() {
            if first_map.contains(&c) && second_map.contains(&c) {
                return Some(c);
            }
        }
    }

    None
}

fn main() {
    let contents = std::fs::read_to_string("data/test.txt").unwrap();

    // problem 1
    let response = contents
        .split("\n")
        .into_iter()
        .map(find_duplicated_item)
        .map(|c| c.unwrap())
        .map(|c| c.to_index())
        .sum::<usize>();

    println!("{}", response);

    let problem2 = contents
        .split("\n")
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .into_iter()
        .map(|chars| Vec::from(chars))
        .map(find_badge)
        .map(|c| c.unwrap())
        .map(|c| c.to_index())
        .sum::<usize>();

    println!("{}", problem2)
}
