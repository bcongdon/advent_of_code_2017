package main

import (
	"fmt"
	"io/ioutil"
)

func parseStream(stream []byte) (streamScore int, garbageChars int) {
	streamScore = 0
	garbageChars = 0
	currLevel := 0

	for idx := 0; idx < len(stream); {
		switch stream[idx] {
		case '{':
			streamScore += currLevel + 1
			currLevel++
			idx++
		case '}':
			currLevel--
			idx++
		case '<':
			gLen, gChars := parseGarbage(stream[idx:])
			idx += gLen
			garbageChars += gChars
		default:
			idx++
		}
	}
	return
}

func parseGarbage(stream []byte) (garbageLen int, numChars int) {
	numChars = 0

parseLoop:
	for garbageLen = 1; garbageLen < len(stream); {
		switch stream[garbageLen] {
		case '>':
			break parseLoop
		case '!':
			garbageLen += 2
		default:
			garbageLen += 1
			numChars++
		}
	}
	return
}

func main() {
	dat, _ := ioutil.ReadFile("./9.txt")
	streamScore, garbageChars := parseStream(dat)
	fmt.Printf("Part 1: %d\n", streamScore)
	fmt.Printf("Part 2: %d\n", garbageChars)
}
