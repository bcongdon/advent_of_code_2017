def run_dance(steps, dancers, repetitions):
    seen_states = set()
    ordered_states = []

    for i in range(repetitions):
        state_repr = ''.join(dancers)
        if state_repr in seen_states:
            return list(ordered_states[repetitions % i])
        else:
            seen_states.add(state_repr)
            ordered_states.append(state_repr)

        for s in steps:
            if s[0] == 's':
                shift = int(s[1:])
                dancers = dancers[-shift:] + dancers[:-shift]
            elif s[0] == 'x':
                p1, p2 = map(int, s[1:].split('/'))
                dancers[p1], dancers[p2] = dancers[p2], dancers[p1]
            elif s[0] == 'p':
                p1, p2 = map(lambda x: dancers.index(x), s[1:].split('/'))
                dancers[p1], dancers[p2] = dancers[p2], dancers[p1]
            else:
                raise RuntimeError("Invalid step")
    return dancers

if __name__ == '__main__':
    config = list("abcdefghijklmnop")

    with open('16.txt') as f:
        steps = f.read().split(',')

    p1 = run_dance(steps, config[:], 1)
    print("Part 1: {}".format(''.join(p1)))

    p2 = run_dance(steps, config[:], 1000000000)
    print("Part 2: {}".format(''.join(p2)))
