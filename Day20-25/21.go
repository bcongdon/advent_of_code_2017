package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

type Config []string

type RuleMatcher struct {
	rules map[string]string
}

// flipChunk returns a flipped image of the provided chunk with respect
// to the horizontal axis.
func flipChunk(chunk Config) Config {
	for i := 0; i < len(chunk)/2; i++ {
		tmp := chunk[i]
		chunk[i] = chunk[len(chunk)-i-1]
		chunk[len(chunk)-i-1] = tmp
	}
	return chunk
}

// transposeChunk returns the matrix transpose of the provided chunk.
func transposeChunk(chunk Config) Config {
	tmpChunk := make([][]byte, len(chunk))
	for i := 0; i < len(chunk); i++ {
		tmpChunk[i] = []byte(chunk[i])
	}

	transposed := make([][]byte, len(chunk))
	for i := 0; i < len(chunk); i++ {
		transposed[i] = make([]byte, len(chunk))
		for j := 0; j < len(chunk); j++ {
			transposed[i][j] = tmpChunk[j][i]
		}
	}

	finalChunk := make([]string, len(chunk))
	for i, row := range transposed {
		finalChunk[i] = string(row)
	}
	return finalChunk
}

// FindMatch returns the configuration string of the output of the
// production rule associated with the input configuration string.
func (r *RuleMatcher) FindMatch(cfgString string) string {
	if match, ok := r.rules[cfgString]; ok {
		return match
	}

	chunk := cfgToChunk(cfgString)
	for i := 0; i < 3; i++ {
		chunk = flipChunk(chunk)
		flippedCfg := chunkToCfgString(chunk)
		if match, ok := r.rules[flippedCfg]; ok {
			r.AddRule(cfgString, match)
			return match
		}

		chunk = transposeChunk(chunk)
		transposed := chunkToCfgString(chunk)
		if match, ok := r.rules[transposed]; ok {
			r.AddRule(cfgString, match)
			return match
		}
	}
	panic("No match found")
}

// AddRule creates a new production rule which will set oldCfg to
// produce newCfg.
func (r *RuleMatcher) AddRule(oldCfg, newCfg string) {
	r.rules[oldCfg] = newCfg
}


// NewMatcher creates and initializes a new RuleMatcher.
func NewMatcher() RuleMatcher {
	return RuleMatcher{
		rules: make(map[string]string),
	}
}

// chunktoCfgString takes a configuration chunk and returns a representation
// of that chunk as a configuration string.
func chunkToCfgString(cfg Config) string {
	cfgStr := make([]byte, 0)
	for x := 0; x < len(cfg); x++ {
		for y := 0; y < len(cfg); y++ {
			cfgStr = append(cfgStr, cfg[x][y])
		}
		if x+1 != len(cfg) {
			cfgStr = append(cfgStr, '/')
		}
	}
	return string(cfgStr)
}

// extractChunkCfgStr returns a section of a larger configuration as a
// configuration string. The string represents the configuration of the chunk
// in the x range [cx, cx+chunkSize] and the y range [cy, cy+chunkSize].
func extractChunkCfgStr(cfg Config, cx int, cy int, chunkSize int) string {
	cfgStr := make([]byte, 0)
	for x := cx; x < cx+chunkSize; x++ {
		for y := cy; y < cy+chunkSize; y++ {
			cfgStr = append(cfgStr, cfg[x][y])
		}
		if x+1 != cx+chunkSize {
			cfgStr = append(cfgStr, '/')
		}
	}
	return string(cfgStr)
}

// cfgToChunk takes a configuration string, and returns a Configuration chunk.
func cfgToChunk(cfg string) Config {
	rows := strings.Split(cfg, "/")
	chunk := make([]string, len(rows))
	for i, row := range rows {
		chunk[i] = row
	}
	return chunk
}

// iterateProduction takes a configuration and a rule matcher, and generates
// the next configuration by splitting the current configuration into chunks
// and using the production rules in the provided matcher to generate the next
// configuration.
func iterateProduction(cfg Config, matcher RuleMatcher) Config {
	var chunkSize int

	switch len(cfg) % 2 {
	case 0:
		chunkSize = 2
	case 1:
		chunkSize = 3
	}

	newSize := (chunkSize + 1) * len(cfg) / chunkSize

	tmpCfg := make([][]byte, newSize)
	for i := 0; i < newSize; i++ {
		tmpCfg[i] = make([]byte, newSize)
	}

	for x := 0; x < len(cfg); x += chunkSize {
		cx := (x / chunkSize) * (chunkSize + 1)
		for y := 0; y < len(cfg); y += chunkSize {
			cy := (y / chunkSize) * (chunkSize + 1)
			chunk := extractChunkCfgStr(cfg, x, y, chunkSize)
			match := matcher.FindMatch(chunk)
			newChunk := cfgToChunk(match)

			for nx := 0; nx < chunkSize+1; nx++ {
				for ny := 0; ny < chunkSize+1; ny++ {
					tmpCfg[nx+cx][ny+cy] = newChunk[nx][ny]
				}
			}
		}
	}

	newCfg := make([]string, newSize)
	for i, row := range tmpCfg {
		newCfg[i] = string(row)
	}

	return newCfg
}


// NumActive returns the number of active areas ('#' occurrences) in the
// Config.
func (c Config) NumActive() int {
	active := 0
	for i := 0; i < len(c); i++ {
		for j := 0; j < len(c); j++ {
			if c[i][j] == '#' {
				active++
			}
		}
	}
	return active
}

func main() {
	dat, _ := ioutil.ReadFile("21.txt")

	matcher := NewMatcher()
	for _, line := range strings.Split(string(dat), "\n") {
		rule := strings.Fields(line)
		matcher.AddRule(rule[0], rule[2])
	}

	config := cfgToChunk(".#./..#/###")
	for i := 1; i <= 18; i++ {
		config = iterateProduction(config, matcher)
		if i == 5 {
			fmt.Printf("Part 1: %d\n", config.NumActive())
		}
	}
	fmt.Printf("Part 2: %d\n", config.NumActive())
}
