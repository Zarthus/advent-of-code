package main

import (
	"os"
	"regexp"
	"strconv"
	"strings"

	"liefland.net/aoc2023/helper"
)

type Game struct {
	Id int
	Sets []Set
};

type Set struct {
	Red int
	Green int
	Blue int
}


func main() {
	lines := helper.ReadInput(os.Open("d02/input.txt"))

	games := parseGames(lines)

	config := Set {
		Red: 12,
		Green: 13,
		Blue: 14,
	}

	// Problem 1
	allowed_games := filterGames(games, config)

	sum_game_ids := 0
	for _, game := range allowed_games {
		sum_game_ids += game.Id
	}

	println("Problem 1:", sum_game_ids)

	// Problem 2
	var minimum_sets []Set

	for _, game := range games {
		minimum_sets = append(minimum_sets, game.MinimumRequiredCubes())
	}

	total_sum := 0
	for _, set := range minimum_sets {
		total_sum += set.Multiply()
	}

	println("Problem 2:", total_sum)

}

func parseGames(lines []string) []Game {
	var games []Game

	sets_regexp := regexp.MustCompile(`\d+ \w+`)

	for _, line := range lines {
		split := strings.SplitN(line, ":", 2)
		game_id, _ := strconv.Atoi(strings.Split(split[0], " ")[1])
		sets_str := strings.Split(split[1], ";")
		var sets []Set

		for _, current_set := range sets_str {
			sets = append(sets, parseSet(*sets_regexp, current_set))
		}


		games = append(games, Game {
			Id: game_id,
			Sets: sets,
		})
	}

	return games
}

func parseSet(matcher regexp.Regexp, current_set string) Set {
	red := 0
	green := 0
	blue := 0

	found_sets := matcher.FindAllString(current_set, -1)
	for _, set := range found_sets {
		amount, _ := strconv.Atoi(strings.Split(set, " ")[0])
		color := strings.Split(set, " ")[1]

		switch color {
		case "red":
			red = amount
		case "green":
			green = amount
		case "blue":
			blue = amount
		}
	}

	return Set {
		Blue: blue,
		Red: red,
		Green: green,
	}
}

func isAllowed(set Set, config Set) bool {
	return set.Red <= config.Red && set.Green <= config.Green && set.Blue <= config.Blue
}

func filterGames(games []Game, config Set) []Game {
	var allowed_games []Game

	for _, game := range games {
		valid := true
		for _, set := range game.Sets {
			if !isAllowed(set, config) {
				valid = false
				break
			}
		}

		if valid {
			allowed_games = append(allowed_games, game)
		}
	}

	return allowed_games
}

func (g Game) MinimumRequiredCubes() Set {
	red := 0
	green := 0
	blue := 0

	for _, set := range g.Sets {
		red = max(set.Red, red)
		green = max(set.Green, green)
		blue = max(set.Blue, blue)
	}

	return Set {
		Red: red,
		Green: green,
		Blue: blue,
	}
}

func (s Set) Multiply() int {
	return s.Red * s.Green * s.Blue
}