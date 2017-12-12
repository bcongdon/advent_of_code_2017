package main

import (
	"fmt"
	"github.com/golang-collections/collections/stack"
	"io/ioutil"
	"strconv"
	"strings"
)

func graphReach(graph map[int][]int, start int) []int {
	toVisit := stack.New()
	toVisit.Push(start)
	
	reachable := make(map[int]bool)
	for toVisit.Len() > 0 {
		curr := toVisit.Pop().(int)
		for _, neighbor := range graph[curr] {
			if _, ok := reachable[neighbor]; !ok {
				reachable[neighbor] = true
				toVisit.Push(neighbor)
			}
		}
	}

	keys := []int{}
	for k := range reachable {
		keys = append(keys, k)
	}
	
	return keys
}

func numConnectedComponents(graph map[int][]int) int {
	marked := make(map[int]bool)
	numComponents := 0
	for vertex := range graph {
		if _, ok := marked[vertex]; ok {
			continue
		}

		component := graphReach(graph, vertex)
		for _, c := range component {
			marked[c] = true
		}
		numComponents++
	}
	return numComponents
}

func parseGraph(edges []string) map[int][]int {
	graph := make(map[int][]int)
	replacer := strings.NewReplacer(",", "", "\n", "")
	for _, e := range edges {
		split := strings.Split(e, " ")
		adjacent := make([]int, len(split)-2)
		
		for i, v := range split[2:] {
			cleanVStr := replacer.Replace(v)
			adjVal, _ := strconv.Atoi(cleanVStr)
			adjacent[i] = adjVal
		}

		vertexVal, _ := strconv.Atoi(split[0])
		graph[vertexVal] = adjacent
	}
	return graph
}

func main() {
	dat, _ := ioutil.ReadFile("./12.txt")
	graph := parseGraph(strings.Split(string(dat), "\n"))
	
	part1 := graphReach(graph, 0)
	fmt.Printf("Part 1: %d\n", len(part1))
	
	part2 := numConnectedComponents(graph)
	fmt.Printf("Part 2: %d\n", part2)
}
