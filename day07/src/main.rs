use std::{cell::RefCell, rc::Rc};

use model::Dir;

use crate::model::OSObject;

mod model;

pub fn parse(input: Vec<&str>) -> Rc<RefCell<Dir>> {
    let objs = input
        .into_iter()
        .map(start_of_line)
        .map(|(depth, line)| {
            let obj = line.parse::<OSObject>().unwrap();
            (depth, obj)
        })
        .collect::<Vec<_>>();
    dbg!(objs);
    Rc::new(RefCell::new(Dir::new("some".to_string()))) // to compile
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
