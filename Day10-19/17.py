def run_spinlock(offset, iterations):
    spinlock = [0]
    loc = 0

    for new_val in range(1, iterations+1):
        loc = (loc + offset) % len(spinlock) + 1
        spinlock.insert(loc, new_val)
    return spinlock, loc


def spinlock_sim(offset, iterations):
    loc = 0
    target_value = None
    for new_val in range(1, iterations+1):
        loc = ((loc + offset) % new_val) + 1
        if loc == 1:
            target_value = new_val
    return target_value


if __name__ == '__main__':
    spinlock, final_loc = run_spinlock(354, 2017)
    print("Part 1: {}".format(spinlock[final_loc+1]))

    target_value = spinlock_sim(354, 50000000)
    print("Part 2: {}".format(target_value))
