package main

import (
    "fmt"
)

const factorA uint64 = 16807
const factorB uint64 = 48271
const generatorMod uint64 = 2147483647
const checkMask = 0xffff
const part1Iterations = 40000000
const part2Iterations = 5000000

func part1(seedA uint64, seedB uint64, c chan int) {
    judge := 0
    a := seedA
    b := seedB
    for i := 0; i < part1Iterations; i++ {
        a = (a*factorA) % generatorMod
        b = (b*factorB) % generatorMod

        if a & checkMask == b & checkMask {
            judge++
        }
    }
    c <- judge
}

func part2(seedA uint64, seedB uint64, c chan int) {
    judge := 0
    a := seedA
    b := seedB
    for i := 0; i < part2Iterations; i++ {
        for a = (a*factorA) % generatorMod; a % 4 != 0; {
            a = (a*factorA) % generatorMod
        }

        for b = (b*factorB) % generatorMod; b % 4 != 0; {
            b = (b*factorB) % generatorMod
        }

        if a & checkMask == b & checkMask {
            judge++
        }
    }
    c <- judge
}

func main() {
    seedA := uint64(783)
    seedB := uint64(325)

    p1 := make(chan int)
    p2 := make(chan int)

    go part1(seedA, seedB, p1)
    go part2(seedA, seedB, p2)

    fmt.Printf("Part 1: %d\n", <-p1)
    fmt.Printf("Part 2: %d\n", <-p2)
}