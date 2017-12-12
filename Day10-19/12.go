package main

import (
    "io/ioutil"
    "fmt"
    "github.com/golang-collections/collections/stack"
    "strings"
)

func graphReach(graph map[string][]string, start string) []string{
    toVisit := stack.New()
    toVisit.Push(start)
    reachable := make(map[string]bool)
    for ; toVisit.Len() > 0; {
        curr := toVisit.Pop().(string)
        for _, neighbor := range graph[curr] {
            if _, ok := reachable[neighbor]; !ok {
                reachable[neighbor] = true
                toVisit.Push(neighbor)
            }
        }
    }

    keys := []string{}
    for k := range reachable {
        keys = append(keys, k)
    }
    return keys
}

func numConnectedComponents(graph map[string][]string) int {
    marked := make(map[string]bool)
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

func parseGraph(edges []string) map[string][]string {
    graph := make(map[string][]string)
    replacer := strings.NewReplacer(",", "", "\n", "")
    for _, e := range edges {
        split := strings.Split(e, " ")
        adjacent := make([]string, len(split) - 2)
        for i, v := range split[2:] {
            adjacent[i] = replacer.Replace(v)
        }
        graph[split[0]] = adjacent
    }
    return graph
}

func main() {
    dat, _ := ioutil.ReadFile("./12.txt")
    graph := parseGraph(strings.Split(string(dat), "\n"))
    part1 := graphReach(graph, "0")
    fmt.Printf("Part 1: %d\n", len(part1))
    part2 := numConnectedComponents(graph)
    fmt.Printf("Part 2: %d\n", part2)
}