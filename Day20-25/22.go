package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

type Direction int

const (
	Up Direction = iota
	Right
	Down
	Left
)

type State int

const (
	Clean State = iota
	Infected
	Flagged
	Weakened
)

func (d Direction) TurnLeft() Direction {
	return (d + 3) % 4
}

func (d Direction) TurnRight() Direction {
	return (d + 1) % 4
}

func (d Direction) TurnAround() Direction {
	return (d + 2) % 4
}

type Location struct {
	x int32
	y int32
}

type Grid map[int64]State

func (l Location) Serialize() int64 {
	return (int64(l.x) << 31) | (int64(l.y) & 0xffff)
}

func (g *Grid) GetState(l Location) State {
	repr := l.Serialize()
	if state, ok := (*g)[repr]; ok {
		return state
	}
	(*g)[repr] = Clean
	return (*g)[repr]
}

func (g *Grid) SetState(l Location, s State) {
	(*g)[l.Serialize()] = s
}

type InfectionTransition map[State]State
type DirectionTransition func(State, Direction) Direction

func runInfection(grid Grid, start Location, iterations int, inf InfectionTransition, dir DirectionTransition) int {
	coord := start
	direction := Up
	newlyInfected := 0

	for i := 0; i < iterations; i++ {
		direction = dir(grid.GetState(coord), direction)

		grid.SetState(coord, inf[grid.GetState(coord)])

		if grid.GetState(coord) == Infected {
			newlyInfected++
		}

		switch direction {
		case Up:
			coord.y--
		case Down:
			coord.y++
		case Right:
			coord.x++
		case Left:
			coord.x--
		default:
			panic("Invalid direction")
		}
	}
	return newlyInfected
}

var part1InfectionTransition = map[State]State{
	Clean:    Infected,
	Infected: Clean,
}

var part2InfectionTransition = map[State]State{
	Clean:    Weakened,
	Weakened: Infected,
	Infected: Flagged,
	Flagged:  Clean,
}

func part1DirectionTransition(s State, d Direction) Direction {
	switch s {
	case Infected:
		return d.TurnRight()
	case Clean:
		return d.TurnLeft()
	default:
		panic("Invalid state")
	}
}

func part2DirectionTransition(s State, d Direction) Direction {
	switch s {
	case Clean:
		return d.TurnLeft()
	case Weakened:
		return d
	case Infected:
		return d.TurnRight()
	case Flagged:
		return d.TurnAround()
	default:
		panic("Invalid state")
	}
}

func main() {
	dat, _ := ioutil.ReadFile("22.txt")
	lines := strings.Split(string(dat), "\n")

	grid := Grid(make(map[int64]State, len(lines)*len(lines)))
	for i, line := range lines {
		for j, symbol := range []byte(line) {
			l := Location{x: int32(j), y: int32(i)}
			switch symbol {
			case '#':
				grid.SetState(l, Infected)
			case '.':
				grid.SetState(l, Clean)
			}
		}
	}

	start := Location{x: int32(len(lines) / 2), y: int32(len(lines) / 2)}

	part1Grid := make(map[int64]State, 10000)
	for k, v := range grid {
		part1Grid[k] = v
	}
	part1 := runInfection(part1Grid, start, 10000, part1InfectionTransition, part1DirectionTransition)
	fmt.Printf("Part 1: %d\n", part1)

	part2Grid := make(map[int64]State, 1000000)
	for k, v := range grid {
		part2Grid[k] = v
	}
	part2 := runInfection(part2Grid, start, 10000000, part2InfectionTransition, part2DirectionTransition)
	fmt.Printf("Part 2: %d\n", part2)
}
