package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func captcha1(numbers [][]int) int {
	result := 0
	for _, line := range numbers {
		min := int(^uint(0) >> 1)
		max := -1
		for _, num := range line {
			if num > max {
				max = num
			}
			if num < min {
				min = num
			}
		}
		result += max - min
	}
	return result
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func captcha2(numbers [][]int) int {
	result := 0
	for _, line := range numbers {
		for i := 0; i < len(line); i++ {
			for j := i + 1; j < len(line); j++ {
				if line[i]%line[j] == 0 || line[j]%line[i] == 0 {
					result += max(line[i], line[j]) / min(line[i], line[j])
				}
			}
		}
	}
	return result
}

func main() {
	dat, _ := ioutil.ReadFile("2.txt")
	lines := strings.Split(string(dat), "\n")

	numbers := make([][]int, len(lines))
	for i, line := range lines {
		splitNums := strings.Fields(line)
		row := make([]int, len(splitNums))
		for j, numStr := range splitNums {
			row[j], _ = strconv.Atoi(numStr)
		}
		numbers[i] = row
	}

	part1 := captcha1(numbers)
	fmt.Printf("Part 1: %d\n", part1)

	part2 := captcha2(numbers)
	fmt.Printf("Part 2: %d\n", part2)
}
