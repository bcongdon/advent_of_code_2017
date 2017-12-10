import numpy as np

magic_suffix = [17, 31, 73, 47, 23]


def knot_hash(inputs, size=256, curr=0, skip=0, knot=None):
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


def part1(inputs, size=256):
    lengths, _, _ = knot_hash(inputs, size=size)
    return lengths[0] * lengths[1]


def part2(input_str):
    lengths = [ord(i) for i in input_str] + magic_suffix
    sparse_hash = None
    curr, skip = 0, 0
    for _ in range(64):
        sparse_hash, curr, skip = knot_hash(
            lengths, curr=curr, skip=skip, knot=sparse_hash
        )
    dense_hash = [0 for _ in range(16)]
    for i in range(16):
        dense_hash[i] = np.bitwise_xor.reduce(sparse_hash[16*i:16*i+16])
    return(bytes(dense_hash).hex())

if __name__ == '__main__':
    # assert part1([3, 4, 1, 5], size=5) == 12
    # assert part2('') == 'a2582a3a0e66e6e86e3812dcb672a272'
    # assert part2('AoC 2017') == '33efeb34ea91902bb2f59c9920caa6cd'

    with open("10.txt") as f:
        raw = f.read().strip()
    lengths = [int(i) for i in raw.split(',')]
    product = part1(lengths)
    print("Part 1: {}".format(product))

    dense_hash = part2(raw)
    print("Part 2: {}".format(dense_hash))
