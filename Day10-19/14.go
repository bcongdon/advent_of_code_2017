package main

import (
	"fmt"
)

var magic_suffix = []byte{17, 31, 73, 47, 23}
var grid_size = 128

type KnotHash struct {
	hash []byte
	curr int
	skip int
}

type Location struct {
	x int
	y int
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

		// Reverse chunk
		for i := 0; i < chunkLen/2; i++ {
			s := (state.curr + i) % size
			e := (state.curr + chunkLen - i - 1) % size
			tmp := state.hash[s]
			state.hash[s] = state.hash[e]
			state.hash[e] = tmp
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

func bitSetCount(v byte) int {
	v = (v & 0x55) + ((v >> 1) & 0x55)
	v = (v & 0x33) + ((v >> 2) & 0x33)
	return int((v + (v >> 4)) & 0xF)
}

func constructGrid(seed []byte) [][]byte {
	grid := make([][]byte, grid_size)
	for i := 0; i < grid_size; i++ {
		key := fmt.Sprintf("%s-%d", seed, i)
		row := knotHashAscii([]byte(key), 64)
		grid[i] = row
	}
	return grid
}

func countUsed(grid [][]byte) int {
	count := 0
	for i := 0; i < grid_size; i++ {
		for _, b := range grid[i] {
			count += bitSetCount(b)
		}
	}
	return count
}

func isSet(grid [][]byte, l Location) bool {
	row := grid[l.x]
	bit := (row[l.y/8] >> (7 - uint64(l.y)%8)) & 0x01
	return bit == 0x1
}

func dfs(grid [][]byte, l Location, visited map[Location]bool) {
	_, isVisited := visited[l]
	if !isSet(grid, l) || isVisited {
		return
	}

	visited[l] = true
	if l.x < grid_size-1 {
		dfs(grid, Location{x: l.x + 1, y: l.y}, visited)
	}
	if l.y < grid_size-1 {
		dfs(grid, Location{x: l.x, y: l.y + 1}, visited)
	}
	if l.x > 0 {
		dfs(grid, Location{x: l.x - 1, y: l.y}, visited)
	}
	if l.y > 0 {
		dfs(grid, Location{x: l.x, y: l.y - 1}, visited)
	}
}

func countGroups(grid [][]byte) int {
	visited := make(map[Location]bool)
	groupCount := 0
	for x := 0; x < grid_size; x++ {
		for y := 0; y < grid_size; y++ {
			l := Location{x: x, y: y}
			if _, isVisited := visited[l]; isVisited || !isSet(grid, l) {
				continue
			}
			dfs(grid, l, visited)
			groupCount++
		}
	}
	return groupCount
}

func main() {
	puzzle := []byte("uugsqrei")
	grid := constructGrid(puzzle)

	part1 := countUsed(grid)
	fmt.Printf("Part 1: %d\n", part1)

	part2 := countGroups(grid)
	fmt.Printf("Part 2: %d\n", part2)
}
