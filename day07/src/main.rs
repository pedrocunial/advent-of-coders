use model::{parse, Dir, Sizeable};

mod model;

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").unwrap();
    parse(contents.lines().collect());
}
