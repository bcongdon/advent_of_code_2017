import numpy as np
magic_suffix = [17, 31, 73, 47, 23]


def knot_hash_iter(inputs, size=256, curr=0, skip=0, knot=None):
    if knot is None:
        knot = np.arange(0, size)
    for i in inputs:
        if curr + i < size:
            chunk = knot[curr:curr+i]
        else:
            chunk = np.append(knot[curr:], knot[:i-(size-curr)])
        chunk = chunk[::-1]
        if curr + i < size:
            knot[curr:curr+i] = chunk
        else:
            knot[curr:] = chunk[:size-curr]
            knot[:i-(size-curr)] = chunk[size-curr:]
        curr = (curr + (i + skip)) % size
        skip += 1
    return knot, curr, skip


def knot_hash(input_str):
    lengths = [ord(i) for i in input_str] + magic_suffix
    sparse_hash = None
    curr, skip = 0, 0
    for _ in range(64):
        sparse_hash, curr, skip = knot_hash_iter(
            lengths, curr=curr, skip=skip, knot=sparse_hash
        )
    dense_hash = [0 for _ in range(16)]
    for i in range(16):
        dense_hash[i] = np.bitwise_xor.reduce(sparse_hash[16*i:16*i+16])
    return dense_hash


def construct_grid(seed):
    for i in range(128):
        key = "{}-{}".format(seed, i)
        row_hash = knot_hash(key)
        row = ''
        for b in row_hash:
            b_str = bin(b)
            b_str = (10 - len(b_str))*'0' + b_str[2:]
            row += b_str
        yield row


def part1(grid):
    return sum(row.count('1') for row in grid)


def part2(grid):
    visited = set()

    def dfs(x, y):
        if grid[x][y] != '1' or (x, y) in visited:
            return
        visited.add((x, y))
        if x < len(grid)-1:
            dfs(x+1, y)
        if y < len(grid[0])-1:
            dfs(x, y+1)
        if x > 0:
            dfs(x-1, y)
        if y > 0:
            dfs(x, y-1)

    group_count = 0
    for x in range(len(grid)):
        for y in range(len(grid[0])):
            if (x, y) not in visited and grid[x][y] == '1':
                dfs(x, y)
                group_count += 1
    return group_count


if __name__ == '__main__':
    puzzle_input = 'uugsqrei'
    grid = [row for row in construct_grid(puzzle_input)]
    p1 = part1(grid)
    print("Part 1: {}".format(p1))

    p2 = part2(grid)
    print("Part 2: {}".format(p2))
