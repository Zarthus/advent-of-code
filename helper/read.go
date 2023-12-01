package helper

import (
	"bufio"
	"os"
)

func ReadInput(file *os.File, err error) []string {
	defer file.Close()

	if err != nil {
		panic(err)
	}

	var lines []string
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        lines = append(lines, scanner.Text())
    }
	return lines
}