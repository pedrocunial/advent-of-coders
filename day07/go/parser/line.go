package parser

import (
	"errors"
	"strings"
)

func parsedLines(contents string) ([]parsedLine, error) {
	lines := strings.Split(contents, "\n")
	parsedLines := []parsedLine{}
	for _, line := range lines {
		parsed, depth := stripPrefix(line)
		parsedLines = append(parsedLines, parsedLine{parsed, depth})
	}

	if len(parsedLines) != len(lines) {
		return nil, errors.New("mismatched len between file and parsed contents")
	}

	return parsedLines, nil
}

func stripPrefix(line string) (string, uint32) {
	depth := uint32(0)
	result := line
	for result[0] != '-' {
		result = strings.TrimPrefix(result, "  ")
		depth += 1
	}

	return result, depth

}
