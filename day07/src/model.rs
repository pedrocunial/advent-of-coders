pub trait Sizeable {
    fn size(&mut self) -> usize;
}

impl PartialEq for dyn Sizeable + '_ {
    fn eq(&self, that: &dyn Sizeable) -> bool {
        <dyn Sizeable>::eq(self, that)
    }
}

impl PartialEq<dyn Sizeable> for Box<dyn Sizeable + '_> {
    fn eq(&self, that: &dyn Sizeable) -> bool {
        <dyn Sizeable>::eq(&**self, that)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Dir {
    pub contents: Vec<Box<dyn Sizeable>>,
    size: Option<usize>,
    parent: Option<Box<dyn Sizeable>>,
}

impl Dir {
    fn new(parent: Option<Box<dyn Sizeable>>) -> Self {
        Dir {
            contents: vec![],
            size: None,
            parent: parent,
        }
    }
}

impl Sizeable for Dir {
    fn size(&mut self) -> usize {
        if let Some(size) = self.size {
            return size;
        }
        let size = self.contents.into_iter().map(|mut it| it.size()).sum();
        self.size = Some(size);
        size
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct File {
    size: usize,
    parent: Option<Box<dyn Sizeable>>,
}

impl File {
    fn new(size: usize, parent: Option<Box<dyn Sizeable>>) -> File {
        File {
            size: size,
            parent: parent,
        }
    }
}

impl Sizeable for File {
    fn size(&mut self) -> usize {
        self.size
    }
}

pub fn parse<T>(input: Vec<&str>) -> Box<dyn Sizeable>
where
    T: Sizeable,
{
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
