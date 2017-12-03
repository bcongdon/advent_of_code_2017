def next_location(x, y):
    det = max(abs(x), abs(y))

    # Exit current shell
    if det == x and det == -y:
        return (x+1, y)
    # Up
    elif det == x and det != y:
        return (x, y+1)
    # Left
    elif det == y and det != -x:
        return (x-1, y)
    # Down
    elif det == -x and det != -y:
        return (x, y-1)
    # Right
    elif det == -y:
        return (x+1, y)


def spiral_cost(n):
    curr = 2
    loc = (1, 0)

    while curr < n:
        loc = next_location(loc[0], loc[1])
        curr += 1
    return dist(loc[0], loc[1])


def dist(x, y):
    return abs(x) + abs(y)


def spiral2(n):
    mem = {}
    mem[(0, 0)] = 1
    loc = (1, 0)

    cost = 0
    while cost <= n:
        cost = 0
        for i in [-1, 0, 1]:
            for j in [-1, 0, 1]:
                cx, cy = loc
                if (cx + i, cy + j) in mem:
                    cost += mem[(cx + i, cy + j)]
        mem[loc] = cost
        loc = next_location(loc[0], loc[1])
    return cost

if __name__ == '__main__':
    print("Part 1: {}".format(spiral_cost(312051)))
    print("Part 2: {}".format(spiral2(312051)))
