def parse_line(l):
    fields = l.split()
    depth = int(fields[0][:-1])
    rnge = int(fields[1])
    return (depth, rnge)


def parse_ranges(lines):
    max_depth, _ = parse_line(lines[-1])
    out = [0 for i in range(max_depth+1)]
    for l in lines:
        fields = l.split()
        depth = int(fields[0][:-1])
        rnge = int(fields[1])
        out[depth] = rnge
    return out


def trip_severity(ranges):
    severity = 0
    for time, rnge in enumerate(ranges):
        if rnge != 0 and time % (2*rnge-2) == 0:
            severity += time*rnge
    return severity


def is_caught(ranges, delay):
    for time, rnge in enumerate(ranges):
        if rnge != 0 and (time+delay) % (2*rnge-2) == 0:
            return True
    return False


def min_delay(ranges):
    delay = 0
    while is_caught(ranges, delay):
        delay += 1
    return delay

if __name__ == '__main__':
    with open('13.txt') as f:
        lines = f.readlines()
    ranges = parse_ranges(lines)
    part1 = trip_severity(ranges)
    print("Part 1: {}".format(part1))

    part2 = min_delay(ranges)
    print("Part 2: {}".format(part2))
