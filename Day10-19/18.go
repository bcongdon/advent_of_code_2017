package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type VM struct {
	instructions *[][]string
	pc           int64
	registers    map[string]int64

	sendFlag bool
	sendVal  int64

	recvQueue []int64

	part1             bool
	firstReceivedFlag bool
	firstReceived     int64
	lastPlayed        int64
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
	case "snd":
		vm.lastPlayed = vm.getValue(inst[1])
		vm.sendVal = vm.getValue(inst[1])
		vm.sendFlag = true
	case "set":
		vm.registers[inst[1]] = vm.getValue(inst[2])
	case "add":
		prev := vm.getValue(inst[1])
		vm.registers[inst[1]] = prev + vm.getValue(inst[2])
	case "mul":
		prev := vm.getValue(inst[1])
		vm.registers[inst[1]] = prev * vm.getValue(inst[2])
	case "mod":
		prev := vm.getValue(inst[1])
		if vm.getValue(inst[2]) == 0 {
			break
		}
		if prev < 0 {
			prev = -prev
		}
		vm.registers[inst[1]] = prev % vm.getValue(inst[2])
	case "rcv":
		if vm.part1 {
			if vm.getValue(inst[1]) != 0 {
				vm.firstReceived = vm.lastPlayed
				vm.firstReceivedFlag = true
			}
		} else if len(vm.recvQueue) > 0 {
			vm.registers[inst[1]] = vm.recvQueue[0]
			vm.recvQueue = vm.recvQueue[1:]
		} else {
			return
		}
	case "jgz":
		if vm.getValue(inst[1]) > 0 {
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

		sendFlag: false,
		sendVal:  0,

		recvQueue: make([]int64, 0),

		part1:             part1,
		firstReceivedFlag: false,
		firstReceived:     0,
		lastPlayed:        0,
	}
}

func makeProcessRegister(pid int64) map[string]int64 {
	reg := make(map[string]int64)
	reg["p"] = pid
	return reg
}

func runVMPair(instructions *[][]string) int {
	P0 := NewVM(instructions, makeProcessRegister(0), false)
	P1 := NewVM(instructions, makeProcessRegister(1), false)

	part2 := 0
	pc0 := int64(-1)
	pc1 := int64(-1)
	for P0.pc != pc0 || P1.pc != pc1 {
		pc0 = P0.pc
		pc1 = P1.pc

		if P0.sendFlag {
			P1.recvQueue = append(P1.recvQueue, P0.sendVal)
			P0.sendFlag = false
		}
		if P1.sendFlag {
			P0.recvQueue = append(P0.recvQueue, P1.sendVal)
			P1.sendFlag = false
			part2++
		}

		P0.runInstruction()
		P1.runInstruction()
	}
	return part2
}

func main() {
	dat, _ := ioutil.ReadFile("./18.txt")
	lines := strings.Split(string(dat), "\n")

	instructions := make([][]string, len(lines))
	for i := 0; i < len(lines); i++ {
		instructions[i] = strings.Fields(lines[i])
	}

	P1 := NewVM(&instructions, make(map[string]int64), true)
	for !P1.firstReceivedFlag || P1.firstReceived == 0 {
		P1.runInstruction()
	}
	fmt.Printf("Part 1: %d\n", P1.firstReceived)

	part2 := runVMPair(&instructions)
	fmt.Printf("Part 2: %d\n", part2)
}
