package main

import (
	"fmt"
)

func runSimlock(offset int, iterations int) (nextVal int) {
	var spinlock []int
	spinlock = append(spinlock, 0)
	loc := 0

	for i := 1; i <= iterations; i++ {
		loc = (loc+offset)%i + 1
		spinlock = append(spinlock, 0)
		copy(spinlock[loc+1:], spinlock[loc:])
		spinlock[loc] = i
	}
	return spinlock[loc+1]
}

func simSpinlock(offset int, iterations int) (afterZero int) {
	loc := 0
	target_val := -1
	for i := 1; i <= iterations; i++ {
		loc = (loc+offset)%i + 1
		if loc == 1 {
			target_val = i
		}
	}
	return target_val
}

func main() {
	p1 := runSimlock(354, 2017)
	fmt.Printf("Part 1: %d\n", p1)

	p2 := simSpinlock(354, 50000000)
	fmt.Printf("Part 2: %d\n", p2)
}
