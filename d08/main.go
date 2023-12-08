package main

import (
	"log"
	"os"
	"regexp"
	"strings"

	"liefland.net/aoc2023/helper"
)

type NavElementCollection struct {
	Elements []NavElement

	index map[string]int
}

type NavElement struct {
	Action string
	Op     []string
}

func main() {
	lines := helper.ReadInput(os.Open("d08/input.txt"))

	var instructions []int
	ins := strings.Split(lines[0], "")
	for _, i := range ins {
		instructions = append(instructions, helper.EnsureAtoi(strings.ReplaceAll(strings.ReplaceAll(i, "L", "0"), "R", "1")))
	}
	collection := parseNavElements(lines[2:])

	// Problem 1
	println("Problem 1:", pathing(instructions, collection, collection.getNavElement("AAA"), "ZZZ"))
}

func pathing(instructions []int, collection NavElementCollection, start NavElement, end string) int {
	instruction_ptr := 0
	instruction_len := len(instructions)
	icount := 0
	var current NavElement = start

	for {
		instruction := instructions[instruction_ptr]
		instruction_ptr++

		if instruction_ptr >= instruction_len {
			instruction_ptr = 0
		}

		current = collection.getNavElement(current.Op[instruction])
		icount++

		if strings.HasSuffix(current.Action, end) {
			break
		}
	}

	return icount
}

func parseNavElements(lines []string) NavElementCollection {
	var collection NavElementCollection
	matcher := regexp.MustCompile(`^(\w+) = \((\w+), (\w+)\)$`)

	for _, line := range lines {
		match := matcher.FindStringSubmatch(line)

		if len(match) == 0 {
			log.Fatal("No match found", line)
		}

		el := NavElement{
			Action: match[1],
			Op:     []string{match[2], match[3]},
		}

		collection.Elements = append(collection.Elements, el)
	}

	return collection
}

func (c *NavElementCollection) getNavElement(name string) NavElement {
	if len(c.index) == 0 {
		c.BuildIndex()
	}

	return c.Elements[c.index[name]]
}

func (c *NavElementCollection) getNavElements(name string) []NavElement {
	if len(c.index) == 0 {
		c.BuildIndex()
	}

	var elements []NavElement
	for _, v := range c.Elements {
		if strings.HasSuffix(v.Action, name) {
			elements = append(elements, v)
		}
	}

	return elements
}

func (c *NavElementCollection) BuildIndex() {
	c.index = make(map[string]int)
	for i, v := range c.Elements {
		c.index[v.Action] = i
	}
}
