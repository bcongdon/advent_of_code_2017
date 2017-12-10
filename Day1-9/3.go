package main

import (
    "fmt"
)

func abs(x int) int {
    if x < 0 {
        return -x
    }
    return x
}

func nextLocation(x int, y int) (int, int) {
    det := 0
    if abs(x) > abs(y) {
        det = abs(x)
    } else {
        det = abs(y)
    }

    if det == x && det == -y {
        return x+1, y
    } else if det == x && det != y {
        return x, y+1
    } else if det == y && det != -x {
        return x-1, y
    } else if det == -x && det != -y {
        return x, y-1
    } else {
        return x+1, y
    }
}

func spiralCost(n int) int {
    x := 1
    y := 0

    for curr := 2; curr < n; curr++ {
        x, y = nextLocation(x, y)
    }

    return abs(x) + abs(y)
}

type Location struct {
    x int
    y int
}

func spiral2(n int) int {
    memMap := make(map[Location]int)
    memMap[Location{x: 0, y: 0}] = 1
    loc := Location{x: 1, y: 0}

    cost := 0
    for cost = 0; cost <= n; {
        cost = 0
        for i := -1; i <= 1; i++ {
            for j := -1; j <= 1; j++ {
                potLoc := Location{x: loc.x + i, y: loc.y + j}
                if _, ok := memMap[potLoc]; ok {
                    cost += memMap[potLoc]
                }
            }
        }
        memMap[loc] = cost
        nX, nY := nextLocation(loc.x, loc.y)
        loc.x = nX
        loc.y = nY
    }
    return cost
}

func main() {
    fmt.Printf("Part 1: %d\n", spiralCost(312051))
    fmt.Printf("Part 2: %d\n", spiral2(312051))
}