def part1_valid(pw):
    words = pw.split()
    return len(set(words)) == len(words)


def part2_valid(pw):
    words = pw.split()
    sorted_words = [''.join(sorted(w)) for w in words]
    return len(set(sorted_words)) == len(words)


def count_valid_passwords(lines, validator):
    return sum(1 for pw in lines if validator(pw))


if __name__ == '__main__':
    with open('4.txt') as f:
        lines = f.readlines()
    print("Part 1: {}".format(
        count_valid_passwords(lines, part1_valid))
    )
    print("Part 2: {}".format(
        count_valid_passwords(lines, part2_valid))
    )
