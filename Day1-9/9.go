package main

import (
	"fmt"
	"github.com/golang-collections/collections/stack"
	"io/ioutil"
)

func parseStream(stream []byte) (streamScore int, garbageChars int) {
	streamScore = 0
	garbageChars = 0
	scoreStack := stack.New()
	scoreStack.Push(0)

	for idx := 0; idx < len(stream); {
		switch stream[idx] {
		case '{':
			streamScore += scoreStack.Peek().(int) + 1
			scoreStack.Push(scoreStack.Len())
			idx++
		case '}':
			scoreStack.Pop()
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
