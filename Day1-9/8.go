package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type Instruction struct {
	assignmentReg   string
	assignmentOp    string
	assignmentAmt   int
	compareReg      string
	compareOperator string
	compareAmt      int
}

func assignmentOp(operator string, a int, b int) int {
	switch operator {
	case "inc":
		return a + b
	case "dec":
		return a - b
	}
	panic("Invalid assignment operator")
}

func compareOp(operator string, a int, b int) bool {
	switch operator {
	case ">":
		return a > b
	case "<":
		return a < b
	case "<=":
		return a <= b
	case ">=":
		return a >= b
	case "==":
		return a == b
	case "!=":
		return a != b
	}
	panic("Invalid assignment operator")
}

func parseLine(line string) Instruction {
	fields := strings.Fields(line)
	inst := Instruction{}
	inst.assignmentReg = fields[0]
	inst.assignmentOp = fields[1]
	inst.assignmentAmt, _ = strconv.Atoi(fields[2])
	inst.compareReg = fields[4]
	inst.compareOperator = fields[5]
	inst.compareAmt, _ = strconv.Atoi(fields[6])
	return inst
}

func runInterpreter(instructions []Instruction) (maxAtEnd int, maxAtRuntime int) {
	registers := make(map[string]int)
	maxAtRuntime = 0
	maxAtEnd = 0
	for _, inst := range instructions {
		// Check that assignment register has been initialized
		if _, ok := registers[inst.assignmentReg]; !ok {
			registers[inst.assignmentReg] = 0
		}
		// Check that comparing register has been initialized
		if _, ok := registers[inst.compareReg]; !ok {
			registers[inst.compareReg] = 0
		}

		compareRegVal := registers[inst.compareReg]
		if compareOp(inst.compareOperator, compareRegVal, inst.compareAmt) {
			assignmentRegVal := registers[inst.assignmentReg]
			assignmentVal := assignmentOp(inst.assignmentOp, assignmentRegVal, inst.assignmentAmt)
			registers[inst.assignmentReg] = assignmentVal

			// Update max assignment value during runtime
			if assignmentVal > maxAtRuntime {
				maxAtRuntime = assignmentVal
			}
		}
	}

	for _, val := range registers {
		if val > maxAtEnd {
			maxAtEnd = val
		}
	}

	return
}

func main() {
	dat, _ := ioutil.ReadFile("./8.txt")
	lines := strings.Split(string(dat), "\n")
	instructions := make([]Instruction, len(lines))

	for i, line := range lines {
		instructions[i] = parseLine(line)
	}

	maxAtEnd, maxAtRuntime := runInterpreter(instructions)
	fmt.Printf("Part 1: %d\n", maxAtEnd)
	fmt.Printf("Part 2: %d\n", maxAtRuntime)
}
