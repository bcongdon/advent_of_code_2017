package main

import "fmt"

type Transition struct {
	newVal    int
	movement  int
	nextState byte
}

var turingMachine = [][]Transition{
	// a
	{
		Transition{1, +1, 'b'},
		Transition{0, -1, 'f'}},
	// b
	{
		Transition{0, +1, 'c'},
		Transition{0, +1, 'd'}},
	// c
	{
		Transition{1, -1, 'd'},
		Transition{1, +1, 'e'}},
	// d
	{
		Transition{0, -1, 'e'},
		Transition{0, -1, 'd'}},
	// e
	{
		Transition{0, +1, 'a'},
		Transition{1, +1, 'c'}},
	// f
	{
		Transition{1, -1, 'a'},
		Transition{1, +1, 'a'}},
}

func runTM(steps int) int {
	tape := make([]int, 15000)
	loc := len(tape) / 2
	state := byte('a')
	numOnes := 0

	for i := 0; i < steps; i++ {
		val := tape[loc]
		t := turingMachine[state-'a'][val]

		if t.newVal == 1 && val == 0 {
			numOnes++
		} else if t.newVal == 0 && val == 1 {
			numOnes--
		}

		tape[loc] = t.newVal
		loc += t.movement
		state = t.nextState
	}
	return numOnes
}

func main() {
	steps := 12994925
	fmt.Printf("Part 1: %d\n", runTM(steps))
	fmt.Printf("Part 2: ⭐\n")
}
