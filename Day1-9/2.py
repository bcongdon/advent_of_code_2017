def line_difference(line):
    nums = [int(i) for i in line.split()]
    return max(nums) - min(nums)


def checksum(data):
    lines = data.split('\n')
    return sum(map(line_difference, lines))


def line_even_score(line):
    nums = [int(i) for i in line.split()]
    for idx, i in enumerate(nums):
        for j in nums[idx+1:]:
            if i % j == 0 or j % i == 0:
                return max([i, j]) // min([i, j])


def checksum2(data):
    lines = data.split('\n')
    return sum(map(line_even_score, lines))


if __name__ == '__main__':
    with open('2.txt') as f:
        data = f.read()
    print('Part 1: {}'.format(checksum(data)))
    print('Part 2: {}'.format(checksum2(data)))
