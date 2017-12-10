def first_jump_outside(jumps, mutator):
    c = 0
    steps = 0
    while c >= 0 and c < len(jumps):
        j = jumps[c]
        jumps[c] = mutator(j)
        steps += 1
        c += j
    return steps

if __name__ == '__main__':
    with open('5.txt') as f:
        jumps = [int(i) for i in f.readlines()]
    part1 = (lambda x: x+1)
    part2 = (lambda x: x-1 if x >= 3 else x+1)
    # example = [0, 3, 0, 1, -3]
    # assert first_jump_outside(example[:], part1) == 5
    # assert first_jump_outside(example[:], part2) == 10
    print("Part 1: {}".format(
        first_jump_outside(jumps[:], lambda x: x+1)
    ))
    print("Part 2: {}".format(
        first_jump_outside(jumps[:], lambda x: x-1 if x >= 3 else x+1)
    ))
