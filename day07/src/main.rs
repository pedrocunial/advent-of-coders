use model::Dir;

mod model;

pub fn parse(input: Vec<&str>) -> Box<Dir> {
    let root = Box::new(Dir::new(None));
    let objs = input.into_iter().map(start_of_line).collect::<Vec<_>>();
    dbg!(objs);
    root
}

fn start_of_line(line: &str) -> (usize, String) {
    let mut it = line.chars().into_iter();
    let mut depth = 0;
    let mut even = false;
    while let Some(c) = it.next() {
        match c {
            ' ' => {
                if even {
                    depth += 1;
                }
                even = !even;
            }
            _ => break,
        }
    }

    (depth, it.collect::<String>())
}
fn main() {
    let contents = std::fs::read_to_string("data/input.txt").unwrap();
    parse(contents.lines().collect());
}
