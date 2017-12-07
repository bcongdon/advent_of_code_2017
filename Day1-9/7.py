import re
input_regex = re.compile('(.+) \((\d+)\)( -> )?(.+)?')


def parse_program(line):
    match = input_regex.match(line)
    pid = match.group(1)
    weight = int(match.group(2))
    if type(match.group(4)) == str:
        children = tuple(match.group(4).split(', '))
    else:
        children = ()
    return (pid, weight, children)


def find_source(programs):
    non_sources = set()
    vertices = set()
    for p in programs:
        pid, weight, children = p
        non_sources.update(children)
        vertices.update(children)
        vertices.add(pid)
    source = list(vertices - non_sources)[0]
    return source


def balance_tree(programs, root):
    vertices = {}
    weights = {}
    for p in programs:
        pid, weight, children = p
        vertices[pid] = p

    def _balance(root):
        pid, weight, children = vertices[root]
        if children:
            for c in children:
                fixed_weight = _balance(c)
                if fixed_weight:
                    return fixed_weight
            cost = min(weights[c] for c in children)
            for c in children:
                if weights[c] != cost:
                    c_pid, c_weight, c_children = vertices[c]
                    fixed = cost - sum(weights[c] for c in c_children)
                    return fixed
            weights[pid] = weight + len(children) * cost
        else:
            weights[pid] = weight
    return _balance(root)


if __name__ == '__main__':
    with open('7.txt') as f:
        lines = f.readlines()
    programs = [parse_program(p) for p in lines]
    source = find_source(programs)
    print("Part 1: {}".format(source))
    fixed_weight = balance_tree(programs, source)
    print("Part 2: {}".format(fixed_weight))
