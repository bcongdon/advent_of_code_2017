import functools


def parse_component(line):
    a, b = line.split('/')
    return (int(a), int(b))


@functools.lru_cache(None)
def _strongest_chain(components, next_comp):
    max_strength = 0
    for c in list(components):
        if next_comp in c:
            c_next_comp = c[0] if c[1] == next_comp else c[1]
            s = _strongest_chain(components - frozenset([c]), c_next_comp)
            if s + next_comp > max_strength:
                max_strength = s + next_comp
    return max_strength + next_comp


def find_strongest_bridge(components):
    max_strength = 0

    for c in list(components):
        if 0 in c:
            non_zero = c[0] if c[1] == 0 else c[1]
            s = _strongest_chain(components - frozenset([c]), non_zero)
            if s > max_strength:
                max_strength = s
    return max_strength


@functools.lru_cache(None)
def _longest_chain(components, next_comp):
    max_strength, max_length = 0, 0
    for c in list(components):
        if next_comp in c:
            c_next_comp = c[0] if c[1] == next_comp else c[1]
            s, l = _longest_chain(components - frozenset([c]), c_next_comp)
            if l > max_length or (l == max_length and s + next_comp > max_strength):
                max_strength = s + next_comp
                max_length = l
    return max_strength + next_comp, max_length + 1


def find_longest_bridge(components):
    max_strength, max_length = 0, 0

    for c in list(components):
        if 0 in c:
            non_zero = c[0] if c[1] == 0 else c[1]
            s, l = _longest_chain(components - frozenset([c]), non_zero)
            if l > max_length or (l == max_length and s > max_strength):
                max_strength = s
                max_length = l
    return max_strength


if __name__ == '__main__':
    with open('24.txt') as f:
        lines = f.readlines()
    components = frozenset(map(parse_component, lines))
    print("Part 1: {}".format(find_strongest_bridge(components)))
    print("Part 2: {}".format(find_longest_bridge(components)))
