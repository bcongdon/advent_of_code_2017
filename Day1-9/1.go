package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func captcha(input string) (part1 int, part2 int) {
	part1 = 0
	part2 = 0
	n := len(input)
	for i := 0; i < n; i++ {
		cval, _ := strconv.Atoi(string(input[i]))
		if input[(i+1)%n] == input[i] {
			part1 += cval
		}
		if input[(i+n/2)%n] == input[i] {
			part2 += cval
		}
	}
	return
}

func main() {
	dat, _ := ioutil.ReadFile("./1.txt")
	part1, part2 := captcha(strings.TrimSpace(string(dat)))
	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}
