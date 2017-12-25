package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type Component struct {
	pins [2]int
	used bool
}

func parseComponents(lines []string) map[int][]*Component {
	c := make(map[int][]*Component, len(lines))

	for _, line := range lines {
		nums := strings.Split(line, "/")
		a, _ := strconv.Atoi(nums[0])
		b, _ := strconv.Atoi(nums[1])

		comp := Component{
			pins: [2]int{a, b},
			used: false,
		}

		arr, ok := c[a]
		if !ok {
			c[a] = []*Component{&comp}
		} else {
			c[a] = append(arr, &comp)
		}

		arr, ok = c[b]
		if !ok {
			c[b] = []*Component{&comp}
		} else {
			c[b] = append(arr, &comp)
		}
	}

	return c
}

func strongestBridgeIter(components *map[int][]*Component, nextComp int) int {
	maxStrength := 0
	for i, comp := range (*components)[nextComp] {
		if comp.used {
			continue
		}

		var currNextComp int
		if comp.pins[0] == nextComp {
			currNextComp = comp.pins[1]
		} else {
			currNextComp = comp.pins[0]
		}

		(*components)[nextComp][i].used = true
		strength := strongestBridgeIter(components, currNextComp)
		(*components)[nextComp][i].used = false

		if strength+nextComp > maxStrength {
			maxStrength = strength + nextComp
		}
	}
	return maxStrength + nextComp
}

func findStrongestBridge(components map[int][]*Component, c chan int) {
	c <- strongestBridgeIter(&components, 0)
}

func longestBridgeIter(components *map[int][]*Component, nextComp int) (int, int) {
	maxStrength := 0
	maxLength := 0
	for i, comp := range (*components)[nextComp] {
		if comp.used {
			continue
		}

		var currNextComp int
		if comp.pins[0] == nextComp {
			currNextComp = comp.pins[1]
		} else {
			currNextComp = comp.pins[0]
		}

		(*components)[nextComp][i].used = true
		strength, length := longestBridgeIter(components, currNextComp)
		(*components)[nextComp][i].used = false

		if length > maxLength || (length == maxLength && strength+nextComp > maxStrength) {
			maxStrength = strength + nextComp
			maxLength = length
		}
	}
	return maxStrength + nextComp, maxLength + 1
}

func findLongestBridge(components map[int][]*Component, c chan int) {
	strength, _ := longestBridgeIter(&components, 0)
	c <- strength
}

func main() {
	dat, _ := ioutil.ReadFile("24.txt")
	lines := strings.Split(string(dat), "\n")

	c1 := make(chan int)
	c2 := make(chan int)

	go findStrongestBridge(parseComponents(lines), c1)
	go findLongestBridge(parseComponents(lines), c2)

	fmt.Printf("Part 1: %d\n", <-c1)
	fmt.Printf("Part 2: %d\n", <-c2)
}
