use std::{cell::RefCell, rc::Rc, str::FromStr};

use regex::Regex;

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

impl FromStr for OSObject {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = Regex::new(r"(?<name>\S+) \((?<type>dir|file, size=(?<size>\d+))\)")
            .map_err(|_| "invalid regex string".to_string())?
            .captures(s)
            .ok_or(format!("couldn't match string {}", s))?;

        let name = cap.name("name").map(|m| m.as_str()).unwrap_or("");
        let typ = cap.name("type").map(|m| m.as_str()).unwrap_or("");

        match typ {
            "dir" => Ok(Self::from_dir(Dir::new(name.to_string()))),
            t if t.contains("file") => {
                let size = cap
                    .name("size")
                    .map(|m| m.as_str())
                    .unwrap_or("")
                    .parse::<usize>()
                    .map_err(|_| format!("invalid size in string {}", s))?;

                Ok(Self::from_file(File::new(name.to_string(), size)))
            }
            format => Err(format!("invalid object type {} in string {}", format, s)),
        }
    }
}
