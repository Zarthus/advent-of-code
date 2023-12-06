package main

import (
	"fmt"
	"math"

	"liefland.net/aoc2023/helper"
)

type Data struct {
	Time     []int
	Distance []int
}

func main() {
	data := Data{
		Time:     []int{49, 78, 79, 80},
		Distance: []int{298, 1185, 1066, 1181},
	}

	// Problem 1
	pairs, total := data.Pairs(), 1

	for _, p := range pairs {
		total *= Calculate(p[0], p[1])
	}

	println("Problem 1:", total)

	// Problem 2
	println("Problem 2:", Calculate(data.Glue()))
}

func Calculate(time int, distance int) int {
	root := math.Sqrt(float64(time*time - 4.0*distance))
	upper := int(math.Ceil((float64(time) + root) / 2.0))
	lower := int(math.Floor((float64(time) - root) / 2.0))

	return upper - lower - 1
}

func (d Data) Glue() (int, int) {
	ti, da := "", ""

	for i := 0; i < len(d.Time); i++ {
		ti += fmt.Sprint(d.Time[i])
		da += fmt.Sprint(d.Distance[i])
	}

	return helper.EnsureAtoi(ti), helper.EnsureAtoi(da)
}

func (d Data) Pairs() [][]int {
	pairs := make([][]int, len(d.Time))
	for i := 0; i < len(d.Time); i++ {
		pairs[i] = []int{d.Time[i], d.Distance[i]}
	}
	return pairs
}
