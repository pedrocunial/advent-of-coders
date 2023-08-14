package main

import (
	"day07/domain"
	"day07/parser"
	"fmt"
	"os"
)

func problem1(nodes []parser.Node, threshold uint32) uint32 {
	sum := uint32(0)
	for _, node := range nodes {
		switch node.(type) {
		case *domain.Dir:
			if node.Size() <= threshold {
				sum += node.Size()
			}
		}
	}
	return sum
}

func main() {
	data, err := os.ReadFile("data/input.txt")
	if err != nil {
		panic(err)
	}
	contents := string(data)
	nodes, err := parser.Parse(contents)
	if err != nil {
		panic(err)
	}

	fmt.Println(problem1(nodes, 100_000))
}
