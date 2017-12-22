from collections import defaultdict
from copy import copy


def part1_new_direction(val, direction):
    if val == 'I':
        return direction + 1
    else:
        return direction + 3


def part2_new_direction(val, direction):
    if val == 'C':
        return direction + 3
    elif val == 'W':
        return direction
    elif val == 'I':
        return direction + 1
    elif val == 'F':
        return direction + 2

part1_transitions = {
    'C': 'I',
    'I': 'C'
}

part_2_transitions = {
    'C': 'W',
    'W': 'I',
    'I': 'F',
    'F': 'C'
}


def run_iterations(grid, start, iterations, new_direction, transitions):
    coord = start
    direction = 0
    newly_infected = 0
    for _ in range(iterations):
        direction = new_direction(grid[coord], direction)
        direction %= 4

        grid[coord] = transitions[grid[coord]]

        if grid[coord] == 'I':
            newly_infected += 1

        x, y = coord
        if direction == 0:
            coord = (x, y-1)
        elif direction == 1:
            coord = (x+1, y)
        elif direction == 2:
            coord = (x, y+1)
        elif direction == 3:
            coord = (x-1, y)
        else:
            raise ValueError("Invalid direction")
    return newly_infected


if __name__ == '__main__':
    with open('22.txt') as f:
        lines = f.readlines()
    grid = defaultdict(lambda: 'C')
    for row_idx, row in enumerate(lines):
        for col_idx, symbol in enumerate(row.strip()):
            if symbol == '#':
                grid[(col_idx, row_idx)] = 'I'
            else:
                grid[(col_idx, row_idx)] = 'C'
    middle = len(lines) // 2
    start = (middle, middle)

    part1 = run_iterations(
        copy(grid),
        start,
        10000,
        part1_new_direction,
        part1_transitions
    )
    print("Part 1: {}".format(part1))

    part1 = run_iterations(
        grid,
        start,
        10000000,
        part2_new_direction,
        part_2_transitions
    )
    print("Part 1: {}".format(part1))
