package main

import (
	"os"
	"strings"

	"liefland.net/aoc2023/helper"
)

type Oasis struct {
    History [][]int
}

func main() {
    lines := helper.ReadInput(os.Open("d09/input.txt"))

    oasis := Oasis{[][]int{}}
    for _, line := range lines {
        pushLine(&oasis, line)
    }

    // Problem 1
    sum := 0
    for _, numbers := range oasis.History {
        sum += predict(numbers, false)
    }

    println("Problem 1:", sum)

    // Problem 2
    sum = 0
    for _, numbers := range oasis.History {
        sum += predict(numbers, true)
    }

    println("Problem 2:", sum)
}

func predict(sequence []int, part2 bool) int {
    if allZeroes(sequence) {
        return 0
    }

    diffs := differences(sequence)

    if part2 {
        return sequence[0] - predict(diffs, part2)
    }

    return sequence[len(sequence) - 1] + predict(diffs, part2)
}

func allZeroes(i []int) bool {
    for _, n := range i {
        if n != 0 {
            return false
        }
    }
    return true
}

func differences(sequence []int) []int {
    diffs := make([]int, len(sequence) - 1)

    for i := 0; i < len(sequence) - 1; i++ {
        diffs[i] = sequence[i + 1] - sequence[i]
    }

    return diffs
}

func pushLine(o *Oasis, line string) {
    var numbers []int

    snums := strings.Split(line, " ")
    for _, snum := range snums {
        numbers = append(numbers, helper.EnsureAtoi(snum))
    }

    o.Push(numbers)
}

func (o *Oasis) Push(numbers []int) {
    o.History = append(o.History, numbers)
}