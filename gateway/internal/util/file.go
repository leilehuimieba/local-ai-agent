package util

import (
	"bufio"
	"os"
)

func CountJSONLLines(path string) int {
	file, err := os.Open(path)
	if err != nil {
		return 0
	}
	defer file.Close()
	count := 0
	for scanner := bufio.NewScanner(file); scanner.Scan(); count++ {
	}
	return count
}

func CountDirEntries(path string) int {
	items, err := os.ReadDir(path)
	if err != nil {
		return 0
	}
	return len(items)
}
