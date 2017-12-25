package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"strconv"
	"strings"
)

type VM struct {
	instructions *[][]string
	pc           int64
	registers    map[string]int64

	mulOperations int
}

func (vm *VM) getValue(token string) int64 {
	val, err := strconv.Atoi(token)

	if err == nil {
		return int64(val)
	}

	if _, ok := vm.registers[token]; !ok {
		vm.registers[token] = 0
	}
	return vm.registers[token]
}

func (vm *VM) runInstruction() {
	inst := (*vm.instructions)[vm.pc]

	switch inst[0] {
	case "set":
		vm.registers[inst[1]] = vm.getValue(inst[2])
	case "sub":
		prev := vm.getValue(inst[1])
		vm.registers[inst[1]] = prev - vm.getValue(inst[2])
	case "mul":
		prev := vm.getValue(inst[1])
		vm.registers[inst[1]] = prev * vm.getValue(inst[2])
		vm.mulOperations++
	case "jnz":
		if vm.getValue(inst[1]) != 0 {
			vm.pc += vm.getValue(inst[2])
			return
		}
	default:
		panic("Invalid operation")
	}
	vm.pc++
}

func NewVM(instructions *[][]string, registers map[string]int64, part1 bool) VM {
	return VM{
		instructions: instructions,
		pc:           0,
		registers:    registers,

		mulOperations: 0,
	}
}

func part2(instructions *[][]string) int {
	var start, end, skip int
	for _, inst := range *instructions {
		if inst[0] == "set" && inst[1] == "b" {
			val, _ := strconv.Atoi(inst[2])
			start = val*100 + 100000
		} else if inst[0] == "sub" && inst[1] == "c" {
			val, _ := strconv.Atoi(inst[2])
			end = start - val
		} else if inst[0] == "sub" && inst[1] == "b" && end != 0 {
			val, _ := strconv.Atoi(inst[2])
			skip = -val
		}
	}

	h := 0
	for x := start; x <= end; x += skip {
		sqrtX := int(math.Sqrt(float64(x)))
		for j := 2; j <= sqrtX; j++ {
			if x%j == 0 {
				h++
			}
		}
	}
	return h
}

func main() {
	dat, _ := ioutil.ReadFile("./23.txt")
	lines := strings.Split(string(dat), "\n")

	instructions := make([][]string, len(lines))
	for i := 0; i < len(lines); i++ {
		instructions[i] = strings.Fields(lines[i])
	}

	P1 := NewVM(&instructions, make(map[string]int64), true)
	for P1.pc < int64(len(instructions)) {
		P1.runInstruction()
	}
	fmt.Printf("Part 1: %d\n", P1.mulOperations)

	fmt.Printf("Part 2: %d\n", part2(&instructions))
}
