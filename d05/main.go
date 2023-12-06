package main

import (
	"os"
	"strings"

	"liefland.net/aoc2023/helper"
)

type SeedMap struct {
    Name string
    Reading []MapReading
}

type MapReading struct {
    DestinationStart int
    SourceStart int
    Length int
}

func main() {
    lines := helper.ReadInput(os.Open("d05/input.txt"))

    seeds := parseSeeds(lines[0])
    seedMap := parseSeedMap(lines[1:])

    // Problem 1
    println("Problem 1:", FindLocation(seeds, seedMap))

    // Problem 2
    println("Problem 2:", FindLocation(seeds, seedMap))
}

func FindLocation(seeds []int, maps []SeedMap) int {
	loc := -1

	for i := range seeds {
	    Maps:

		for _, m := range maps {
			for _, reading := range m.Reading {
				if reading.Between(seeds[i]) {
					seeds[i] = seeds[i] - reading.SourceStart + reading.DestinationStart
					continue Maps
				}
			}
		}

		if loc == -1 || seeds[i] < loc {
			loc = seeds[i]
		}
	}

	return loc
}

func parseSeeds(line string) []int {
    ln := strings.Replace(line, "seeds: ", "", 1)

    sdata := strings.Split(ln, " ")
    var seeds []int

    for _, s := range sdata {
        seeds = append(seeds, helper.EnsureAtoi(s))
    }

    return seeds
}

func parseSeedMap(lines []string) []SeedMap {
    var seedMap []SeedMap
    var currentMap SeedMap

    for _, line := range lines {
        if strings.HasSuffix(line, "map:") {
            if !currentMap.Empty() {
                seedMap = append(seedMap, currentMap)
            }
            currentMap = SeedMap{Name: strings.Replace(line, " map:", "", 1), Reading: nil}
            continue
        }

        if line == "" {
            continue
        }

        scoords := strings.Split(line, " ")
        coords := MapReading{DestinationStart: helper.EnsureAtoi(scoords[0]), SourceStart: helper.EnsureAtoi(scoords[1]), Length: helper.EnsureAtoi(scoords[2])}
        currentMap.AddCoords(coords)
    }

    seedMap = append(seedMap, currentMap)

    return seedMap
}

func (s *SeedMap) AddCoords(coords MapReading) {
    s.Reading = append(s.Reading, coords)
}

func (s SeedMap) Empty() bool {
    return len(s.Reading) == 0
}

func (s SeedMap) LookupSource(seed int) int {
    for _, r := range s.Reading {
        if r.SourceStart <= seed && seed <= (r.DestinationStart + r.Length) {
            seed = seed - r.SourceStart + r.DestinationStart
            break
        }
    }
    return seed
}

func (r MapReading) Between(n int) bool {
    return n >= r.SourceStart && n <= r.SourceStart + r.Length - 1
}
