package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strings"
)

type validator func(string) bool

func part1Validator(input string) bool {
	words := strings.Split(input, " ")
	wordSet := make(map[string]bool)
	for _, w := range words {
		wordSet[w] = true
	}
	return len(wordSet) == len(words)
}

func part2Validator(input string) bool {
	words := strings.Split(input, " ")
	wordSet := make(map[string]bool)
	for _, w := range words {
		s := strings.Split(w, "")
		sort.Strings(s)
		wordSet[strings.Join(s, "")] = true
	}
	return len(wordSet) == len(words)
}

func countValidPasswords(candidates []string, isValid validator) int {
	count := 0
	for _, pw := range candidates {
		if isValid(pw) {
			count++
		}
	}
	return count
}

func main() {
	dat, _ := ioutil.ReadFile("./4.txt")
	lines := strings.Split(string(dat), "\n")
	fmt.Printf("Part 1: %d\n", countValidPasswords(lines, part1Validator))
	fmt.Printf("Part 2: %d\n", countValidPasswords(lines, part2Validator))
}
