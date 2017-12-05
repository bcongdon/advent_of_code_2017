package main

import (
    "fmt"
    "strings"
    "strconv"
    "io/ioutil"
)

type offsetUpdater func (int) int

func stepsToBreakout(input []int, update offsetUpdater) int {
    offsets := make([]int, len(input))
    copy(offsets, input)
    curr := 0
    steps := 0
    for ; curr >= 0 && curr < len(offsets); steps ++ {
        j := offsets[curr]
        offsets[curr] = update(j)
        curr += j
    }
    return steps
}

func part1(o int) int {
    return o + 1
}

func part2(o int) int {
    if o >= 3 {
        return o - 1
    } else {
        return o + 1
    }
}

func parseInput(in string) []int {
    lines := strings.Split(in, "\n")
    offsets := make([]int, len(lines))
    for i := 0; i < len(lines); i++ {
        offsets[i], _ = strconv.Atoi(lines[i])
    }
    return offsets
}

func main() {
    dat, _ := ioutil.ReadFile("./5.txt")
    offsets := parseInput(string(dat))
    fmt.Printf("Part 1: %d\n", stepsToBreakout(offsets, part1))
    fmt.Printf("Part 2: %d\n", stepsToBreakout(offsets, part2))
}