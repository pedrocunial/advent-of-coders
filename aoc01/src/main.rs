use std::fs;

fn sum_elf_bag(elf_bag: &str) -> i32 {
    elf_bag
        .split("\n")
        .into_iter()
        .map(|n| n.parse::<i32>().unwrap())
        .sum()
}

fn main() {
    let contents = fs::read_to_string("data/test.txt").expect("file should exist");

    let result = contents
        .split("\n\n")
        .into_iter()
        .map(sum_elf_bag)
        .max()
        .unwrap();
    println!("{}", result);
}
