def cycles_until_reseen(bins):
    configs = set()

    curr = bins
    while tuple(curr) not in configs:
        configs.add(tuple(curr))
        min_idx = 0
        for i in range(1, len(curr)):
            if curr[i] > curr[min_idx]:
                min_idx = i
        redistribute = curr[min_idx]
        curr[min_idx] = 0
        all_gains = redistribute // len(bins)
        partial_gain_bins = redistribute % len(bins)
        for i in range(len(bins)):
            curr[i] += all_gains
            if i < partial_gain_bins:
                curr[(min_idx+i+1) % len(bins)] += 1
    return len(configs), curr

if __name__ == '__main__':
    with open('6.txt') as f:
        bins = [int(i) for i in f.read().split()]
    num_cycles, last_seen = cycles_until_reseen(bins)
    print("Part 1: {}".format(num_cycles))
    loop_cycles, _ = cycles_until_reseen(last_seen)
    print("Part 2: {}".format(loop_cycles))
