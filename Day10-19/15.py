a_factor = 16807
b_factor = 48271
generator_mod = 2147483647
check_mask = 0xffff


def part1(a_seed, b_seed):
    judge = 0
    a = a_seed
    b = b_seed
    for i in range(40*10**6):
        a = (a*a_factor) % generator_mod
        b = (b*b_factor) % generator_mod

        if a & check_mask == b & check_mask:
            judge += 1
    return judge


def part2(a_seed, b_seed):
    judge = 0
    a = a_seed
    b = b_seed
    for i in range(5*10**6):
        a = (a*a_factor) % generator_mod
        while a % 4 != 0:
            a = (a*a_factor) % generator_mod

        b = (b*b_factor) % generator_mod
        while b % 8 != 0:
            b = (b*b_factor) % generator_mod

        if a & check_mask == b & check_mask:
            judge += 1
    return judge

if __name__ == '__main__':
    a_seed = 783
    b_seed = 325

    p1 = part1(a_seed, b_seed)
    print("Part 1: {}".format(p1))

    p2 = part2(a_seed, b_seed)
    print("Part 2: {}".format(p2))