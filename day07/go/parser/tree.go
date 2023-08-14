package parser

import (
	"day07/domain"
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

func buildTree(lines []parsedLine) ([]Node, error) {
	toDomain, err := lineToDomain(lines[0].content)
	if err != nil {
		return nil, err
	}

	curr := toDomain
	depth := uint32(0)
	nodes := []Node{curr}
	for _, line := range lines[1:] {
		toDomain, err := lineToDomain(line.content)
		if err != nil {
			return nil, err
		}

		updateNodeFamily(line, depth, toDomain, curr)
		curr = toDomain
		depth = line.depth
		nodes = append(nodes, curr)
	}

	return nodes, nil
}

func updateNodeFamily(line parsedLine, depth uint32, toDomain Node, curr Node) {
	if line.depth == depth { // sibbling
		toDomain.SetParent(curr.Parent())
	} else if line.depth >= depth { // child
		toDomain.SetParent(curr.(*domain.Dir))
	} else { // uncle?
		toDomain.SetParent(curr.Parent().Parent())
	}
}

func lineToDomain(line string) (Node, error) {
	var name, typ, size string
	re := regexp.MustCompile(`(?P<name>\S+) \((?P<type>dir|file, size=(?P<size>\d+))\)`)
	match := re.FindStringSubmatch(line)
	for i, n := range re.SubexpNames() {
		if i != 0 && n != "" {
			switch n {
			case "name":
				name = match[i]
			case "type":
				typ = match[i]
			case "size":
				size = match[i]
			default:
				return nil, fmt.Errorf("impossible match %s", n)
			}
		}
	}

	if typ == "dir" {
		return domain.NewDir(name), nil
	} else if strings.Contains(typ, "file") {
		s, err := strconv.ParseUint(size, 10, 0)
		if err != nil {
			return nil, err
		}

		return domain.NewFile(name, uint32(s)), nil
	}

	return nil, fmt.Errorf("invalid type %s", typ)
}
