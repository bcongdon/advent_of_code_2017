package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type Segment struct {
	rng   int
	depth int
}

func parseSegment(line string) Segment {
	fields := strings.Fields(line)
	depth, _ := strconv.Atoi(fields[0][:len(fields[0])-1])
	rng, _ := strconv.Atoi(fields[1])
	return Segment{rng: rng, depth: depth}
}

func parseLines(lines []string) []Segment {
	segments := make([]Segment, 0)
	for _, line := range lines {
		segments = append(segments, parseSegment(line))
	}
	return segments
}

func tripSeverity(segments []Segment) int {
	severity := 0
	for _, segment := range segments {
		if (2*segment.rng-2) == 0 || segment.depth%(2*segment.rng-2) == 0 {
			severity += segment.rng * segment.depth
		}
	}
	return severity
}

func getsCaught(segments []Segment, delay int) bool {
	for _, segment := range segments {
		if (delay+segment.depth)%(2*segment.rng-2) == 0 {
			return true
		}
	}
	return false
}

func minDelay(segments []Segment) int {
	delay := 0
	for delay = 0; getsCaught(segments, delay); delay++ {
	}
	return delay
}

func main() {
	dat, _ := ioutil.ReadFile("./13.txt")
	lines := strings.Split(string(dat), "\n")
	segments := parseLines(lines)

	part1 := tripSeverity(segments)
	fmt.Printf("Part 1: %d\n", part1)

	part2 := minDelay(segments)
	fmt.Printf("Part 2: %d\n", part2)
}
