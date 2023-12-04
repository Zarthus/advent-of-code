package main

import (
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"

	"liefland.net/aoc2023/helper"
)

type Card struct {
	Id             int
	WinningNumbers []int
	Numbers        []int
}

func main() {
	lines := helper.ReadInput(os.Open("d04/input.txt"))

	cards := parseCards(lines)

	// Problem 1
	var points int
	for _, card := range cards {
		p := card.Points()
		// log.Printf("Card %d is worth %d points", card.Id, p)
		points += p
	}

	println("Problem 1:", points)

	// Problem 2
	card_copies := make(map[int]int)

	for _, c := range cards {
		card_copies[c.Id] = 1
	}

	for _, c := range cards {
		for matches := c.Matches(); matches > 0; matches-- {
			card_copies[c.Id+matches] += card_copies[c.Id]
		}
	}

	sum := 0
	for _, copies := range card_copies {
		sum += copies
	}

	println("Problem 2:", sum)
}

func parseCards(lines []string) []Card {
	parse_cards_regexp := regexp.MustCompile(`Card \s*(\d+): ([\d ]+) \| ([\d ]+)`)
	var cards []Card

	for _, line := range lines {
		matches := parse_cards_regexp.FindStringSubmatch(line)

		if matches == nil {
			log.Fatalf("Could not parse line: %s", line)
			continue
		}

		id, _ := strconv.Atoi(matches[1])
		winning_numbers_sa := strings.Split(matches[2], " ")
		numbers_sa := strings.Split(matches[3], " ")

		var winning_numbers []int
		var numbers []int

		for _, winning_number := range winning_numbers_sa {
			if winning_number == "" {
				continue
			}

			n, err := strconv.Atoi(winning_number)

			if err != nil {
				log.Printf("Could not parse winning number: %v", winning_number)
				continue
			}

			winning_numbers = append(winning_numbers, n)
		}
		for _, number := range numbers_sa {
			if number == "" {
				continue
			}

			n, err := strconv.Atoi(number)

			if err != nil {
				log.Printf("Could not parse number: %v", number)
				continue
			}

			numbers = append(numbers, n)
		}

		cards = append(cards, Card{
			Id:             id,
			WinningNumbers: winning_numbers,
			Numbers:        numbers,
		})
	}

	return cards
}

func (c Card) Points() int {
	var points int
	for _, winning_number := range c.WinningNumbers {
		for _, number := range c.Numbers {
			if number == winning_number {
				if points == 0 {
					points++
				} else {
					points *= 2
				}
			}
		}
	}
	return points
}

func (c Card) Matches() int {
	var matches int

	for _, winning_number := range c.WinningNumbers {
		for _, number := range c.Numbers {
			if number == winning_number {
				matches++
			}
		}
	}

	return matches
}
