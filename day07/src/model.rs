#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Dir {
    size: Option<usize>,
    children: Vec<Dir>,
    parent: Option<Box<Dir>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct File {
    size: usize,
    parent: Box<Dir>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum OSObject {
    File(File),
    Dir(Dir),
}

impl Dir {
    pub fn new(parent: Option<Box<Dir>>) -> Dir {
        Dir {
            size: None,
            children: vec![],
            parent: parent,
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
    pub fn new(parent: Box<Dir>, size: usize) -> File {
        File {
            size: size,
            parent: parent,
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
