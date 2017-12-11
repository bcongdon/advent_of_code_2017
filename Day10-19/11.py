def manhatten_hex_dist(orig, dest):
    dist_x = dest[0] - orig[0]
    dist_y = dest[1] - orig[1]

    if dist_x > 0 == dist_y > 0:
        return abs(dist_x + dist_y)
    else:
        return max(abs(dist_x), abs(dist_y))


def next_loc(orig, direc):
    if direc == 'n':
        return (orig[0], orig[1]+1)
    elif direc == 'ne':
        return (orig[0]+1, orig[1])
    elif direc == 'se':
        return (orig[0]+1, orig[1]-1)
    elif direc == 's':
        return (orig[0], orig[1]-1)
    elif direc == 'sw':
        return (orig[0]-1, orig[1])
    elif direc == 'nw':
        return (orig[0]-1, orig[1]+1)


def path_dist(path):
    loc = (0, 0)
    m = 0
    for direc in path:
        loc = next_loc(loc, direc)
        m = max(m, manhatten_hex_dist((0, 0), loc))
    return manhatten_hex_dist((0, 0), loc), m


if __name__ == '__main__':
    with open('11.txt') as f:
        directions = f.read().split(',')
    part1, part2 = path_dist(directions)
    print("Part 1: {}".format(part1))
    print("Part 2: {}".format(part2))
