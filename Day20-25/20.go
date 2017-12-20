package main

import (
    "strconv"
    "fmt"
    "regexp"
    "io/ioutil"
    "strings"
    "math"
)

type Particle struct {
    x int64
    y int64
    z int64

    vx int64
    vy int64
    vz int64

    ax int64
    ay int64
    az int64

    annihilated bool
}

func (p Particle) Distance() float64 {
    dist := math.Pow(float64(p.x), 2)
    dist += math.Pow(float64(p.y), 2)
    dist += math.Pow(float64(p.z), 2)
    return dist
}

func closestParticle(particles []Particle) int {
    time := float64(1000)
    tSquared := math.Pow(time, 2)

    distances := make([]float64, len(particles))
    for i, p := range(particles) {
        p.x += int64(time*float64(p.vx)) + int64(0.5*tSquared*float64(p.ax))
        p.y += int64(time*float64(p.vy)) + int64(0.5*tSquared*float64(p.ay))
        p.z += int64(time*float64(p.vz)) + int64(0.5*tSquared*float64(p.az))
        distances[i] = p.Distance()
    }

    minParticle := 0
    for i, d := range(distances) {
        if d < distances[minParticle] {
            minParticle = i
        }
    }
    return minParticle
}

func parseParticle(line string) Particle {
    coordsStr := regexp.MustCompile("[p,v,a,=,<,>,\\s]+").Split(line, 11)
    
    coords := make([]int64, len(coordsStr))
    for i, cStr := range(coordsStr[1:]) {
        val, _ := strconv.Atoi(cStr)
        coords[i] = int64(val)
    }

    return Particle{
        x: coords[0],
        y: coords[1],
        z: coords[2],
        vx: coords[3],
        vy: coords[4],
        vz: coords[5],
        ax: coords[6],
        ay: coords[7],
        az: coords[8],
        annihilated: false,
    }
}

func numUncollidedParticles(particles []Particle) int {
    rounds := 40

    for i := 0; i < rounds; i++ {
        locations := make(map[string]int)
        for i, p := range(particles) {
            if p.annihilated {
                continue
            }

            p.vx += p.ax
            p.x += p.vx
            p.vy += p.ay
            p.y += p.vy
            p.vz += p.az
            p.z += p.vz

            pLoc := fmt.Sprintf("%d,%d,%d", p.x, p.y, p.z)

            if _, ok := locations[pLoc]; ok {
                particles[locations[pLoc]].annihilated = true
                p.annihilated = true
            } else {
                locations[pLoc] = i
            }
            particles[i] = p
        }
    }

    remaining := 0
    for _, p := range(particles) {
        if !p.annihilated {
            remaining++
        }
    }
    return remaining
}

func main() {
    dat, _ := ioutil.ReadFile("20.txt")
    lines := strings.Split(string(dat), "\n")

    particles := make([]Particle, len(lines))
    for i, line := range(lines) {
        particles[i] = parseParticle(line)
    }

    part1Particles := make([]Particle, len(particles))
    copy(part1Particles, particles)
    part1 := closestParticle(part1Particles)
    fmt.Printf("Part 1: %d\n", part1)

    part2 := numUncollidedParticles(particles)
    fmt.Printf("Part 2: %d\n", part2)
}