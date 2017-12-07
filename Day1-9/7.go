package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"regexp"
	"strconv"
	"strings"
)

type Program struct {
	pid      string
	weight   int
	children []string
}

func parseLine(line string) Program {
	re := regexp.MustCompile("^(.+) \\((\\d+)\\)( -> )?(.+)?")
	matches := re.FindStringSubmatch(line)
	p := Program{}
	p.pid = matches[1]
	p.weight, _ = strconv.Atoi(matches[2])

	if len(matches[4]) > 0 {
		p.children = strings.Split(matches[4], ", ")
	} else {
		p.children = make([]string, 0)
	}

	return p
}

func findSource(programs []Program) Program {
	nonSources := make(map[string]bool)
	for _, p := range programs {
		for _, c := range p.children {
			nonSources[c] = true
		}
	}
	for _, p := range programs {
		if _, ok := nonSources[p.pid]; !ok {
			return p
		}
	}
	panic("Unreachable")
}

func balance(root Program, vertices map[string]Program, weights map[string]int) int {
	for _, c := range root.children {
		fixed := balance(vertices[c], vertices, weights)
		if fixed >= 0 {
			return fixed
		}
	}
	cost := math.MaxInt64
	for _, c := range root.children {
		if weights[c] < cost {
			cost = weights[c]
		}
	}
	for _, c := range root.children {
		if weights[c] != cost {
			c_cost := 0
			for _, c_child := range vertices[c].children {
				c_cost += weights[c_child]
			}
			fixed := cost - c_cost
			return fixed
		}
	}
	weights[root.pid] = root.weight + len(root.children)*cost
	return -1
}

func balanceTree(programs []Program, root Program) int {
	vertices := make(map[string]Program)
	weights := make(map[string]int)

	for _, p := range programs {
		vertices[p.pid] = p
		weights[p.pid] = p.weight
	}
	return balance(root, vertices, weights)
}

func main() {
	dat, _ := ioutil.ReadFile("./7.txt")
	lines := strings.Split(string(dat), "\n")

	programs := make([]Program, len(lines))
	for idx, line := range lines {
		programs[idx] = parseLine(line)
	}

	source := findSource(programs)
	fmt.Printf("Part 1: %s\n", source.pid)

	fixed := balanceTree(programs, source)
	fmt.Printf("Part 2: %d\n", fixed)
}
