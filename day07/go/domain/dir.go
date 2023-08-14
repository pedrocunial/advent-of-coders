package domain

import "fmt"

type (
	Sizeable interface {
		Size() uint32
	}

	Dir struct {
		parent   *Dir
		children []Sizeable
		name     string
		size     *uint32
	}
)

func NewDir(name string) *Dir {
	return &Dir{name: name, size: nil}
}

func (d *Dir) Size() uint32 {
	if d.size != nil {
		return *d.size
	}

	size := uint32(0)
	for _, c := range d.children {
		size += c.Size()
	}
	d.size = &size

	return size
}

func (d *Dir) String() string {
	return fmt.Sprintf("Dir{name: %s, children: %v}", d.name, d.children)
}

func (d *Dir) SetParent(parent *Dir) {
	d.parent = parent
	parent.children = append(parent.children, d)
}

func (d *Dir) Parent() *Dir {
	return d.parent
}
