package main

import (
	"math"
	"os"

	"liefland.net/aoc2023/helper"
)

type position struct {
	row int
	col int
}

func main() {
	lines := helper.ReadInput(os.Open("d11/input.txt"))
	galaxies := parseGalaxies(lines)

	// Problem 1
	println("Problem 1:", sum(expand(galaxies, 2)))

	// Problem 2
	println("Problem 2:", sum(expand(galaxies, 1_000_000)))
}

func sum(galaxies []position) int {
	var sum int
	for i := range galaxies {
		for j := i + 1; j < len(galaxies); j++ {
			sum += distance(galaxies[i], galaxies[j])
		}
	}

	return sum
}

func distance(a position, b position) int {
	return helper.Abs(a.row-b.row) + helper.Abs(a.col-b.col)
}

func expand(galaxies []position, factor int) []position {
	cols, rows := simplify(galaxies)

	var expanded []position
	for _, g := range galaxies {
		exp := g
		for _, row := range rows {
			if row > g.row {
				break
			}
			exp.row += factor - 1
		}
		for _, col := range cols {
			if col > g.col {
				break
			}
			exp.col += factor - 1
		}
		expanded = append(expanded, exp)
	}

	return expanded
}

func simplify(galaxies []position) ([]int, []int) {
	cols := make(map[int]bool)
	rows := make(map[int]bool)

	for _, galaxy := range galaxies {
		cols[galaxy.col] = true
		rows[galaxy.row] = true
	}

	handle := func(item map[int]bool) []int {
		min, max := minmax(item)

		var data []int

		for ptr := min; ptr <= max; ptr++ {
			if !item[ptr] {
				data = append(data, ptr)
			}
		}

		return data
	}

	return handle(cols), handle(rows)
}

func minmax(m map[int]bool) (int, int) {
	min, max := math.MaxInt, math.MinInt
	for key := range m {
		if key < min {
			min = key
		}
		if key > max {
			max = key
		}
	}

	return min, max
}

func parseGalaxies(lines []string) []position {
	var galaxies []position
	for row, line := range lines {
		for col, char := range line {
			if char == '#' {
				galaxies = append(galaxies, position{row, col})
			}
		}
	}

	return galaxies
}
