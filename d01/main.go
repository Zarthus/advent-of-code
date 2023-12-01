package main

import (
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"

	"liefland.net/aoc2023/helper"
)


func main() {
	lines := helper.ReadInput(os.Open("d01/input.txt"))

	var nums []int
	pattern, err := regexp.Compile(`\d+`)

	if err != nil {
		panic(err)
	}

	for _, line := range lines {
		ln := numberConvert(line) // part 2
		p := pattern.FindAllString(ln, -1)

		first := strings.Split(p[0], "")[0]
		last_nums := strings.Split(p[len(p)-1], "")
		last := last_nums[len(last_nums)-1]

		num, err := strconv.Atoi(first + "" + last)
		// log.Println(line, ":", first, last, "=", num)

		if err != nil {
			log.Println(err)
			continue
		}

		nums = append(nums, num)
	}

	sum := 0
	for _, num := range nums {
		sum += num
	}

	println("Answer", sum)
}

func numberConvert(inp string) string {
	// disable part 2:
	// return inp

	replacements := map[string]string{
		"one": "o1e",
		"two": "tw2wo",
		"three": "thre3ree",
		"four": "fou4our",
		"five": "fiv5ive",
		"six": "si6ix",
		"seven": "seve7even",
		"eight": "eigh8ight",
		"nine": "nin9ine",
	}

	for k, v := range replacements {
		inp = strings.ReplaceAll(inp, k, v)
	}

	return inp
}