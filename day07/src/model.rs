use std::{cell::RefCell, rc::Rc, str::FromStr};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Dir {
    size: Option<usize>,
    children: Vec<Dir>,
    parent: Option<Rc<RefCell<Dir>>>,
    name: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct File {
    size: usize,
    parent: Option<Rc<RefCell<Dir>>>,
    name: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum OSObject {
    File(File),
    Dir(Dir),
}

impl Dir {
    pub fn new(name: String) -> Dir {
        Dir {
            size: None,
            children: vec![],
            parent: None,
            name: name,
        }
    }

    fn size(&mut self) -> usize {
        if let Some(size) = self.size {
            return size;
        }

        let size = self
            .children
            .clone()
            .into_iter()
            .map(|mut child| child.size())
            .sum();
        self.size = Some(size);
        size
    }
}

impl File {
    pub fn new(name: String, size: usize) -> File {
        File {
            size: size,
            name: name,
            parent: None,
        }
    }
}

impl OSObject {
    pub fn size(&mut self) -> usize {
        match self {
            OSObject::File(file) => file.size,
            OSObject::Dir(ref mut dir) => dir.size(),
        }
    }

    pub fn from_dir(dir: Dir) -> Self {
        Self::Dir(dir)
    }

    pub fn from_file(file: File) -> Self {
        Self::File(file)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct OSObjectParseErr {}

impl FromStr for OSObject {
    type Err = OSObjectParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ref mut it = s.chars().into_iter();
        if let Some(c) = it.next() {
            if c == ' ' {
                return Err(OSObjectParseErr {});
            }
        }

        let name = it.take_while(|&c| c != ' ').collect::<String>();

        if let Some(c) = it.next() {
            if c == '(' {
                return Err(OSObjectParseErr {});
            }
        }

        let typ = it.take_while(|&c| c != ' ').collect::<String>();
        match typ.as_str() {
            "dir" => return Ok(Self::from_dir(Dir::new(name))),
            "file" => (),
            _ => return Err(OSObjectParseErr {}),
        }

        if let (Some(c1), Some(c2)) = (it.next(), it.next()) {
            if c1 != ',' || c2 != ' ' {
                return Err(OSObjectParseErr {});
            }
        }
        if "size=" != it.take(5).collect::<String>().as_str() {
            return Err(OSObjectParseErr {});
        }
        let size = it.take_while(|&c| c != ')').collect::<String>();

        return Err(OSObjectParseErr {});
    }
}
