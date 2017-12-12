package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func nextLoc(x int, y int, direction string) (int, int) {
	switch direction {
	case "n":
		return x, y + 1
	case "ne":
		return x + 1, y
	case "se":
		return x + 1, y - 1
	case "s":
		return x, y - 1
	case "sw":
		return x - 1, y
	case "nw":
		return x - 1, y + 1
	}
	panic("Invalid direction")
}

func hexManhattenDist(x0 int, y0 int, x1 int, y1 int) int {
	distX := x1 - x0
	distY := y1 - y0

	if (distX > 0) == (distY > 0) {
		return abs(distX + distY)
	} else if abs(distX) > abs(distY) {
		return abs(distX)
	} else {
		return abs(distY)
	}
}

func calculatePathDistances(directions []string) (endDist int, maxDist int) {
	x := 0
	y := 0
	maxDist = 0
	for _, direction := range directions {
		x, y = nextLoc(x, y, direction)
		dist := hexManhattenDist(0, 0, x, y)
		if dist > maxDist {
			maxDist = dist
		}
	}
	endDist = hexManhattenDist(0, 0, x, y)
	return
}

func main() {
	dat, _ := ioutil.ReadFile("./11.txt")
	directions := strings.Split(string(dat), ",")
	part1, part2 := calculatePathDistances(directions)
	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}
