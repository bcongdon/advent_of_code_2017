package main

import (
	"fmt"
	"io/ioutil"
	"strings"
	"unicode"
)

func traversePipeGrid(grid []string) (letters string, count int) {
	start := -1
	for i, c := range grid[0] {
		if c == '|' {
			start = i
		}
	}

	dx := 0
	dy := 1
	x := start
	y := 0
	letterBuf := make([]byte, 0)
	count = 0
	for !unicode.IsSpace(rune(grid[y][x])) {
		if grid[y][x] == '+' {
			if dx != 0 {
				dx = 0
				if y+1 < len(grid) && (grid[y+1][x] == '|' || unicode.IsLetter(rune(grid[y+1][x]))) {
					dy = 1
				} else {
					dy = -1
				}
			} else {
				dy = 0
				if x+1 < len(grid[0]) && (grid[y][x+1] == '-' || unicode.IsLetter(rune(grid[y][x+1]))) {
					dx = 1
				} else {
					dx = -1
				}
			}
		} else if unicode.IsLetter(rune(grid[y][x])) {
			letterBuf = append(letterBuf, grid[y][x])
		}
		x += dx
		y += dy
		count++
	}
	letters = string(letterBuf)
	return
}

func main() {
	dat, _ := ioutil.ReadFile("./19.txt")
	grid := strings.Split(string(dat), "\n")
	letters, count := traversePipeGrid(grid)
	fmt.Printf("Part 1: %s\n", letters)
	fmt.Printf("Part 2: %d\n", count)
}
