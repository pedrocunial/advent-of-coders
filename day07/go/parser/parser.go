package parser

import "day07/domain"

type (
	Node interface {
		Size() uint32
		SetParent(*domain.Dir)
		Parent() *domain.Dir
	}

	parsedLine struct {
		content string
		depth   uint32
	}
)

func Parse(contents string) ([]Node, error) {
	parsedLines, err := parsedLines(contents)
	if err != nil {
		return nil, err
	}

	return buildTree(parsedLines)
}
