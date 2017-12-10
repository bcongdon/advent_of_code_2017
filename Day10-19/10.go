package main

import (
	"encoding/hex"
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

var magic_suffix = []byte{17, 31, 73, 47, 23}

type KnotHash struct {
	hash []byte
	curr int
	skip int
}

func NewKnotHash(size int) KnotHash {
	hash := KnotHash{
		hash: make([]byte, size),
		curr: 0,
		skip: 0,
	}

	for i := 0; i < size; i++ {
		hash.hash[i] = byte(i)
	}
	return hash
}

func knotHash(lengths []byte, state KnotHash) KnotHash {
	size := len(state.hash)
	for _, currLen := range lengths {
		chunkLen := int(currLen)
		var chunk []byte

		// Extract chunk
		if state.curr+chunkLen < size {
			chunk = state.hash[state.curr : state.curr+chunkLen]
		} else {
			chunk = state.hash[state.curr:]
			chunk = append(chunk, state.hash[:chunkLen-(size-state.curr)]...)
		}

		// Reverse chunk
		for i := 0; i < len(chunk)/2; i++ {
			tmp := chunk[i]
			chunk[i] = chunk[len(chunk)-i-1]
			chunk[len(chunk)-i-1] = tmp
		}

		// Reassign reversed chunk
		for i := 0; i < chunkLen; i++ {
			state.hash[(i+state.curr)%len(state.hash)] = chunk[i]
		}

		state.curr = (state.curr + (chunkLen + state.skip)) % len(state.hash)
		state.skip++
	}
	return state
}

func knotHashAscii(bytes []byte, rounds int) []byte {
	preparedBytes := append(bytes, magic_suffix...)
	hash := NewKnotHash(256)
	for i := 0; i < 64; i++ {
		hash = knotHash(preparedBytes, hash)
	}

	denseHash := make([]byte, 16)
	for i := 0; i < len(denseHash); i++ {
		xor := byte(0)
		for j := 16 * i; j < 16*i+16; j++ {
			xor ^= hash.hash[j]
		}
		denseHash[i] = xor
	}
	return denseHash
}

func main() {
	dat, _ := ioutil.ReadFile("./10.txt")
	digits := strings.Split(string(dat), ",")
	part1Input := make([]byte, len(digits))
	for idx, digit := range digits {
		intVal, _ := strconv.Atoi(digit)
		part1Input[idx] = byte(intVal)
	}

	part1Hash := NewKnotHash(256)
	knotHash(part1Input, part1Hash)
	part1Result := int(part1Hash.hash[0]) * int(part1Hash.hash[1])
	fmt.Printf("Part 1: %d\n", part1Result)

	part2DenseHash := knotHashAscii(dat, 64)
	fmt.Printf("Part 2: %s\n", hex.EncodeToString(part2DenseHash))
}
