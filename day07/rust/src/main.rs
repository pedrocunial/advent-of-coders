use std::{cell::RefCell, rc::Rc};

use model::Dir;

use crate::model::OSObject;

mod model;

fn wire_objs(mut objs: Vec<(usize, Rc<RefCell<OSObject>>)>) {
    let mut depth = 0usize;
    let mut parent: &RefCell<OSObject> = objs[0].1.as_ref();
    for (current_depth, obj) in objs {
        while current_depth > depth {
            if let Some(node) = parent.borrow().parent() {
                parent = node.as_ref();
            };
        }
    }
}

fn parse(input: Vec<&str>) -> Rc<RefCell<Dir>> {
    let objs = input
        .into_iter()
        .map(start_of_line)
        .map(|(depth, line)| {
            let obj = line.parse::<OSObject>().unwrap();
            (depth, Rc::new(RefCell::new(obj)))
        })
        .collect::<Vec<_>>();
    let root = wire_objs(objs);
    dbg!(root);
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
