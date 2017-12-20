import re
from collections import defaultdict
from functools import reduce
import copy


def parse_particles(lines):
    particles = []
    pattern = re.compile('[p,v,a,=,<,>,\s]+')
    for l in lines:
        particles.append([int(i) for i in pattern.split(l)[1:-1]])
    return particles


def particle_dist(particle):
    return sum(abs(p)**2 for p in particle[:3])


def closest_particle(particles):
    time = 500
    for p in particles:
        for dim in range(3):
            l, v, a = p[dim], p[3+dim], p[6+dim]
            final_l = l + v*time + 0.5*a*time**2
            p[dim] = final_l

    m_idx = 0
    for idx, d in enumerate(particles):
        if particle_dist(d) < particle_dist(particles[m_idx]):
            m_idx = idx
    return m_idx


def num_noncolliding_particles(particles):
    for _ in range(40):
        locations = defaultdict(list)
        for p in particles:
            for dim in range(3):
                l, v, a = p[dim], p[3+dim], p[6+dim]
                v += a
                l += v
                p[dim], p[3+dim] = l, v
            locations[tuple(p[:3])].append(p)
        mapped = [
            p_list for k, p_list in locations.items() if len(p_list) == 1
        ]
        particles = reduce(lambda x, y: x+y, mapped)
    return len(particles)

if __name__ == '__main__':
    with open('20.txt') as f:
        lines = f.readlines()
    particles = parse_particles(lines)

    part1 = closest_particle(copy.deepcopy(particles))
    print("Part 1: {}".format(part1))

    part2 = num_noncolliding_particles(particles)
    print("Part 2: {}".format(part2))
