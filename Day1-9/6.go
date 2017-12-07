package main

import (
	"encoding/binary"
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func serializeBins(bins []int64) string {
	bytes := make([]byte, len(bins)*8)
	for i := 0; i < len(bins); i++ {
		binary.LittleEndian.PutUint64(bytes[i*8:], uint64(bins[i]))
	}
	return string(bytes)
}

func cyclesUntilReseen(bins []int64) (int, []int64) {
	configs := make(map[string]bool)

	for {
		_, contains := configs[serializeBins(bins)]
		if contains {
			break
		}

		configs[serializeBins(bins)] = true
		minIdx := 0
		for i := 1; i < len(bins); i++ {
			if bins[i] > bins[minIdx] {
				minIdx = i
			}
		}

		redistribute := bins[minIdx]
		bins[minIdx] = 0
		allGains := redistribute / int64(len(bins))
		partialGainsBins := int(redistribute % int64(len(bins)))
		for i := 0; i < len(bins); i++ {
			bins[i] += allGains
			if i < partialGainsBins {
				bins[(minIdx+i+1)%len(bins)]++
			}
		}
	}

	return len(configs), bins
}

func main() {
	dat, _ := ioutil.ReadFile("./6.txt")
	strNums := strings.Fields(string(dat))
	bins := make([]int64, len(strNums))
	for i := 0; i < len(strNums); i++ {
		num, _ := strconv.Atoi(strNums[i])
		bins[i] = int64(num)
	}

	cycles, lastSeen := cyclesUntilReseen(bins)
	fmt.Printf("Part 1: %d\n", cycles)
	loopCycles, _ := cyclesUntilReseen(lastSeen)
	fmt.Printf("Part 2: %d\n", loopCycles)
}
