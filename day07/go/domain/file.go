package domain

import "fmt"

type (
	File struct {
		size   uint32
		name   string
		parent *Dir
	}
)

func NewFile(name string, size uint32) *File {
	return &File{size: size, name: name}
}

func (f *File) Size() uint32 {
	return f.size
}

func (f *File) String() string {
	return fmt.Sprintf("File{name: %s, size: %d}", f.name, f.size)
}

func (f *File) SetParent(parent *Dir) {
	f.parent = parent
	parent.children = append(parent.children, f)
}

func (f *File) Parent() *Dir {
	return f.parent
}
