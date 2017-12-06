def serialize(bins):
    return ','.join(map(str, bins))


def cycles_until_reseen(bins):
    configs = set()

    curr = bins
    while serialize(curr) not in configs:
        configs.add(serialize(curr))
        min_idx = 0
        for i in range(1, len(curr)):
            if curr[i] > curr[min_idx]:
                min_idx = i
        redistribute = curr[min_idx]
        curr[min_idx] = 0
        for i in range(redistribute):
            curr[(min_idx+i+1) % len(curr)] += 1
    return len(configs), curr

if __name__ == '__main__':
    with open('6.txt') as f:
        bins = [int(i) for i in f.read().split()]
    num_cycles, last_seen = cycles_until_reseen(bins)
    print("Part 1: {}".format(num_cycles))
    loop_cycles, _ = cycles_until_reseen(last_seen)
    print("Part 2: {}".format(loop_cycles))
