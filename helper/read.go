package helper

import (
	"bufio"
	"os"
	"strconv"
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

func EnsureAtoi(s string) int {
	i, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return i
}

func Abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}
