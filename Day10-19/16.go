package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

const numDancers = 16

func runDance(steps []string, dancers []byte, repetitions int, result chan []byte) {
	seenStates := make(map[string]bool)
	orderedStates := make([][]byte, 0)

	for i := 0; i < repetitions; i++ {
		if _, ok := seenStates[string(dancers)]; ok {
			result <- orderedStates[repetitions%i]
			return
		} else {
			seenStates[string(dancers)] = true
			cpy := make([]byte, len(dancers))
			copy(cpy, dancers)
			orderedStates = append(orderedStates, cpy)
		}

		for _, step := range steps {
			switch step[0] {
			case 's':
				shift, _ := strconv.Atoi(step[1:])
				dancers = append(dancers[len(dancers)-shift:], dancers[:len(dancers)-shift]...)
			case 'x':
				positions := strings.Split(step[1:], "/")
				p1, _ := strconv.Atoi(positions[0])
				p2, _ := strconv.Atoi(positions[1])
				tmp := dancers[p1]
				dancers[p1] = dancers[p2]
				dancers[p2] = tmp
			case 'p':
				partners := strings.Split(step[1:], "/")
				p1 := -1
				p2 := -1

				for j := 0; j < len(dancers); j++ {
					if partners[0][0] == dancers[j] {
						p1 = j
					}
					if partners[1][0] == dancers[j] {
						p2 = j
					}
				}
				tmp := dancers[p1]
				dancers[p1] = dancers[p2]
				dancers[p2] = tmp
			default:
				panic("Invalid step")
			}
		}
	}
	result <- dancers
}

func makeDancers(n int) []byte {
	dancers := make([]byte, n)
	for i := 0; i < n; i++ {
		dancers[i] = 'a' + byte(i)
	}
	return dancers
}

func main() {
	dat, _ := ioutil.ReadFile("./16.txt")
	steps := strings.Split(string(dat), ",")

	p1 := make(chan []byte)
	p2 := make(chan []byte)

	dancers := makeDancers(numDancers)
	go runDance(steps, dancers, 1, p1)

	dancers = makeDancers(numDancers)
	go runDance(steps, dancers, 1000000000, p2)

	fmt.Printf("Part 1: %s\n", <-p1)
	fmt.Printf("Part 2: %s\n", <-p2)
}
